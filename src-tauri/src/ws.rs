

// #[derive(Default)]
// struct MyState {
//   s: std::sync::Mutex<String>,
//   t: std::sync::Mutex<std::collections::HashMap<String, String>>,
// }
// // remember to call `.manage(MyState::default())`
// #[tauri::command]
// async fn command_name(state: tauri::State<'_, MyState>) -> Result<(), String> {
//   *state.s.lock().unwrap() = "new string".into();
//   state.t.lock().unwrap().insert("key".into(), "value".into());
//   Ok(())
// }


pub mod chat_serivce {
    use chute::{spmc::{Queue, Reader}, LendingReader};
    use tokio::task::JoinHandle;

    use super::ws_handler::WsRequest;


  pub struct ChatService {
    queue: Queue<WsRequest>
  }

  impl ChatService {
      fn new() -> Self {
        Self {
          queue: Queue::new()
        }
      }

      fn reader(&self) -> Reader<WsRequest> {
        self.queue.reader()
      }

      async fn catch(&self, message_id: i32) -> WsRequest {
        let mut reader = self.queue.reader();
        let task = tokio::spawn(async move {
            loop {
              if let Some(msg) = reader.next() {
                if msg.id == message_id {
                  break msg.clone();
                }
              }
            }
        });

        task.await.unwrap()
      }

      async fn catch_as_task(&self, message_id: i32) -> JoinHandle<WsRequest> {
        let mut reader = self.queue.reader();
        let task = tokio::spawn(async move {
            loop {
              if let Some(msg) = reader.next() {
                if msg.id == message_id {
                  break msg.clone();
                }
              }
            }
        });

        task
      }
  }
}

pub mod chat_runtime {
  use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
  use tokio::task::JoinHandle;
  use tokio_tungstenite::tungstenite::Message;


  pub enum ChatRuntime {
      Uninitialised,
      Running(ChatService),
      Closed,
      Error(String)
  }

  impl Default for ChatRuntime {
    fn default() -> Self {
        ChatRuntime::Uninitialised
    }
  }

  pub struct ChatService {
    tx: UnboundedSender<Message>,
    rx: UnboundedReceiver<Message>,
    ws_handle: JoinHandle<()>
  }

  impl ChatService {
    pub fn new(
      tx: UnboundedSender<Message>,
      rx: UnboundedReceiver<Message>,
      ws_handle: JoinHandle<()>
    ) -> Self {
      Self {
        tx: tx,
        rx: rx,
        ws_handle
      }
    }

    pub fn sender(&self) -> UnboundedSender<Message> {
      self.tx.clone()
    }

    pub fn abort(&self) {
      self.ws_handle.abort();
    }
  }
}


pub mod ws_handler {
  use std::collections::HashMap;
  use async_trait::async_trait;
  use serde::{Deserialize, Serialize};

  pub enum WsError {
    RouteNotFound,
    MethodNotFound,
    Custom(String)
  }

  #[derive(Deserialize, Debug, Clone)]
  pub struct WsRequest {
    pub id: i32,
    pub route: String,
    pub method: String,
    pub data: Option<serde_json::Value>
  }

  #[derive(Serialize, Debug, Clone)]
  pub struct WsResponse {
    id: i32,
    route: String,
    method: String,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>
  }

  struct WsResponseBuilder {
    response : WsResponse
  }

  impl WsResponseBuilder {
    fn from_request(req: &WsRequest) -> Self {
        Self {
            response: WsResponse {
              id: req.id.clone(),
              route: req.route.clone(),
              method: req.method.clone(),
              data: None,
              error: None
            }
        }
    }

    fn finish(mut self, data: Option<serde_json::Value>, error: Option<String>) -> WsResponse {
      self.response.data = data;
      self.response.error = error;

      self.response
    }
}
  
  #[async_trait]
  pub trait WsHandler: Send + Sync {
    async fn handle(&self, message: Option<serde_json::Value>) -> Result<Option<serde_json::Value>, String>;
  }

  pub struct WsGlobalRouterBuilder {
    routers: HashMap<String, WsRouter>
  }

  impl WsGlobalRouterBuilder {
    pub fn new() -> Self {
      Self {
        routers: HashMap::new()
      } 
    }

    pub fn add_router(mut self, route: &str, router: WsRouter) -> Self {
      let route = String::from(route).to_lowercase();
      let err_msg = format!("Router ({}) with given name already exists", &route);
      let is_exists = 
        self.routers.insert(route, router);
        
      if let Some(_) = is_exists {
        panic!("{}", err_msg);
      }

      self
    }

    pub fn result(self) -> WsGlobalRouter {
      WsGlobalRouter { routers: self.routers }
    }
  }

  pub struct WsGlobalRouter {
    routers: HashMap<String, WsRouter>
  }

  impl WsGlobalRouter {
    pub async fn handle(&self, message: WsRequest) -> Result<Option<WsResponse>, WsError> {
      let router = match self.routers.get(&message.route) {
          Some(_router) => _router,
          None => return Err(WsError::RouteNotFound)
      };

      router.handle(message).await
    }
}


  pub struct WsRouterBuilder {
    handlers: HashMap<String, Box<dyn WsHandler + 'static>>
  }

  impl WsRouterBuilder {
    
    pub fn new() -> Self {
      Self {
        handlers: HashMap::default()
      }
    }

    pub fn add_handler(mut self, method: &str, handler: impl WsHandler + 'static) -> Self {
      let method = String::from(method).to_lowercase();
      let err_msg = format!("Method ({}) with given name already exists", &method);
      let is_exists = 
      self.handlers.insert(method, Box::new(handler));
        
      if let Some(_) = is_exists {
        panic!("{}", err_msg);
      }

      self
    }

    pub fn result(self) -> WsRouter {
      WsRouter { handlers: self.handlers }
    }

  }


  pub struct WsRouter {
    handlers: HashMap<String, Box<dyn WsHandler + 'static>>
  }

  impl WsRouter {
      pub async fn handle(&self, message: WsRequest) -> Result<Option<WsResponse>, WsError> {
        let handler = match self.handlers.get(&message.method) {
            Some(_handler) => _handler,
            None => return Err(WsError::MethodNotFound)
        };

        let response_builder = WsResponseBuilder::from_request(&message);

        match handler.handle(message.data).await {
            Ok(_data) => Ok(Some(response_builder.finish(_data, None))),
            Err(_error) => Ok(Some(response_builder.finish(None, Some(_error)))),
        }
      }
  }

}


pub mod delivery_service {
  use futures::{channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender}, stream::SplitStream, StreamExt};
  use parking_lot::Mutex;
  use tauri::command;
  use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
  use tokio::{net::{TcpListener, TcpStream}, task::JoinHandle};
  use std::sync::Arc;
  
  use super::chat_runtime::{ChatRuntime, ChatService};

  #[command]
  async fn connect_to_ds(url: String, state: tauri::State<'_, Arc<Mutex<ChatRuntime>>>) -> Result<(), String> {
    let mut runtime = state.lock();
    match &*runtime {
      ChatRuntime::Uninitialised => (),
      ChatRuntime::Running(_) => return Err("Chat websocket already connected".to_string()),
      ChatRuntime::Closed => return Err("Chat websocket are closed".to_string()),
      ChatRuntime::Error(err) => return Err(err.clone()),
    };

    let (tx, mut rx) = unbounded::<Message>();

    let (ws_stream, _) = connect_async(&url)
    .await
    .map_err(|err| err.to_string())?;

    let (ws_sink, mut ws_reader) = ws_stream.split();

    

    // *runtime = ChatRuntime::Running(ChatService::new(tx, rx));
    Ok(())
  }

  async fn run_ws_handler(ws_reader: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) -> JoinHandle<()> {
    tokio::spawn(async move {
      while let Some(incoming_msg) = ws_reader.next().await {
          match incoming_msg {
              Ok(msg) => {
                  // ws_sink.lock().await.send(Message::Text("Pong".into())).await.unwrap();
                  sink.send(Message::)
              }
              Err(e) => {
                  eprintln!("Error reading message: {}", e);
                  break;
              }
          }
      }
      println!("Client disconnected / reader ended");
    })
  }
}