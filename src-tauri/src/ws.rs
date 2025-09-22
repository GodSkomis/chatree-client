

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

pub mod chat_runtime {
  use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
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
    rx: UnboundedReceiver<Message>
  }

  impl ChatService {
    pub fn new(tx: UnboundedSender<Message>, rx: UnboundedReceiver<Message>) -> Self {
      Self {
        tx: tx,
        rx: rx
      }
    }
    pub fn sender(&self) -> UnboundedSender<Message> {
      self.tx.clone()
    }
  }
}



pub mod delivery_service {
  use futures::{channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender}, StreamExt};
  use parking_lot::Mutex;
  use tauri::command;
  use tokio_tungstenite::{connect_async, tungstenite::Message};
  use tokio::net::TcpListener;
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
    let sink = ws_sink;

    let q = tokio::spawn(async move {
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
    });

    // *runtime = ChatRuntime::Running(ChatService::new(tx, rx));
    Ok(())
  }
}