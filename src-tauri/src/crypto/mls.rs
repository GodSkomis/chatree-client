use base64::Engine;
use tauri::command;
use openmls::prelude::*;
use openmls_basic_credential::SignatureKeyPair;
use openmls_rust_crypto::OpenMlsRustCrypto;
use openmls::prelude::tls_codec::*;

use crate::auth::signup::SignUpSchema;


fn ciphersuite() -> Ciphersuite {
  Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519
}


fn provider() -> OpenMlsRustCrypto {
  OpenMlsRustCrypto::default()
}


#[command]
pub fn generate_keypackage(user_id: i64) -> String {
  let provider = provider();

  let (cred, signer) = {
    let signature_keys = SignatureKeyPair::new(ciphersuite().signature_algorithm()).unwrap();
    signature_keys.store(provider.storage()).unwrap();
    let credential = BasicCredential::new(user_id.to_le_bytes().to_vec());
    (CredentialWithKey {
      credential: credential.into(),
      signature_key: signature_keys.public().into(),
    }, signature_keys)
  };

  let kp = KeyPackage::builder()
    .build(ciphersuite(), &provider, &signer, cred)
    .unwrap()
    .key_package()
    .clone();

  let serialized = kp.tls_serialize_detached().unwrap();

  base64::engine::general_purpose::STANDARD.encode(serialized)
}


#[command]
pub async fn signup(schema: &str) -> Result<(), String> {
  let signup_schema: SignUpSchema = serde_json::from_str(schema).map_err(|e| e.to_string())?;
  let result = crate::auth::signup::signup(signup_schema).await?;
  Ok(())
}

//////////////////////


mod tst {
  use std::{collections::HashMap, ops::Deref, sync::Arc};
  use parking_lot::{Mutex, RwLock};

use openmls::{group::{GroupId, MlsGroup, MlsGroupCreateConfig, MlsGroupJoinConfig, StagedWelcome}, prelude::{group_info::GroupInfo, tls_codec::{Deserialize, Serialize}, BasicCredential, Ciphersuite, CredentialType, CredentialWithKey, KeyPackage, KeyPackageBundle, MlsMessageBodyIn, MlsMessageIn, MlsMessageOut, OpenMlsProvider, SignatureScheme, Welcome}};
  use openmls_basic_credential::SignatureKeyPair;
  use openmls_rust_crypto::OpenMlsRustCrypto;


  const CIPHERSUITE: openmls::prelude::Ciphersuite = Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519;


  fn generate_credential_with_key(
      identity: Vec<u8>,
      credential_type: CredentialType,
      signature_algorithm: SignatureScheme,
      provider: &impl OpenMlsProvider,
  ) -> (CredentialWithKey, SignatureKeyPair) {
      let credential = BasicCredential::new(identity);
      let signature_keys =
          SignatureKeyPair::new(signature_algorithm)
              .expect("Error generating a signature key pair.");

      // Store the signature key into the key store so OpenMLS has access
      // to it.
      signature_keys
          .store(provider.storage())
          .expect("Error storing signature keys in key store.");
      
      (
          CredentialWithKey {
              credential: credential.into(),
              signature_key: signature_keys.public().into(),
          },
          signature_keys,
      )
  }

  
  fn generate_key_package(
      provider: &impl OpenMlsProvider,
      signer: &SignatureKeyPair,
      credential_with_key: CredentialWithKey,
  ) -> KeyPackageBundle {
      KeyPackage::builder()
          .build(
              CIPHERSUITE,
              provider,
              signer,
              credential_with_key,
          )
          .unwrap()
  }


  pub struct User {
    pub id: String,
    credential_with_key: CredentialWithKey,
    signer: SignatureKeyPair,
    provider: Provider,
    key_package: KeyPackageBundle,
    groups: RwLock<HashMap<String, GroupRecord>>,
  }

  type GroupRecord = Arc<Mutex<MlsGroup>>;
  type Provider = Arc<Mutex<OpenMlsRustCrypto>>;

  impl User {
      pub fn new(user_id: String) -> Self {
        let provider = OpenMlsRustCrypto::default();

        let (credential_with_key, signer) = generate_credential_with_key(
            "Sasha".into(),
            CredentialType::Basic,
            CIPHERSUITE.signature_algorithm(),
            &provider,
        );

        Self {
          id: user_id,
          key_package: generate_key_package(&provider, &signer, credential_with_key.clone()),
          credential_with_key: credential_with_key,
          signer: signer,
          provider: Arc::new(Mutex::new(provider)),
          groups: RwLock::new(HashMap::new())
        }
      }

      fn provider(&self) -> Provider {
        self.provider.clone()
      }

      fn signer(&self) -> &SignatureKeyPair {
        &self.signer
      }

      fn key_package(&self) -> KeyPackageBundle {
        self.key_package.clone()
      }
  }

  impl User {
      pub fn create_group(&self, group_id: String) -> Group {
        let mls_group;
        {
          let provider = &self.provider.lock();
          mls_group = MlsGroup::builder()
          .with_group_id(GroupId::from_slice(group_id.as_bytes()))
          .build(
            provider.deref(),
            &self.signer,
            self.credential_with_key.clone(),
          )
          .expect("An unexpected error occurred.");
        }

        let group: GroupRecord = Arc::new(Mutex::new(mls_group));
        {
          let mut group_storage = self.groups.write();
          group_storage.insert(group_id, group.clone());
        }

        Group{
          owner: &self,
          record: group
        }
      }

      pub fn join_group(&self, welcome: Welcome) -> Group {
        let mut mls_group;
        {
          let provider = self.provider.lock();
          let staged_join = StagedWelcome::new_from_welcome(
            provider.deref(),
            &MlsGroupJoinConfig::default(),
            welcome,
            // The public tree is needed and transferred out of band.
            // It is also possible to use the [`RatchetTreeExtension`]
            // Some(sasha_group.export_ratchet_tree().into()),
            None
          )
          .expect("Error creating a staged join from Welcome");
          
          mls_group = staged_join
            .into_group(provider.deref())
            .expect("Error creating the group from the staged join");
        }

        let group_id = String::from_utf8(mls_group.group_id().to_vec()).unwrap();
        let record = Arc::new(Mutex::new(mls_group));

        {
          let mut groups = self.groups.write();
          groups.insert(group_id, record.clone());
        }

        Group{
          owner: &self,
          record: record
        }

      }
  }

  
  struct Group<'a> {
    owner: &'a User,
    record: GroupRecord
  }

  struct AddMemberResult{
    mls_message_out: MlsMessageOut,
    welcome_out: MlsMessageOut,
    group_info: Option<GroupInfo>
  }

  impl<'a> Group<'a> {
       pub fn add_member(&self, member_key_package: &KeyPackageBundle) -> AddMemberResult {

        // LOCK ORDER!!!!!!!!!!!!!!!
        let provider_mutex = self.owner.provider();
        let provider = provider_mutex.lock();
        let mut group = self.record.lock();

        let (mls_message_out, welcome_out, group_info) = group
          .add_members(provider.deref(), &self.owner.signer, &[member_key_package.key_package().clone()])
          .expect("Could not add members.");

        group
          .merge_pending_commit(provider.deref())
          .expect("error merging pending commit");

        AddMemberResult{ mls_message_out, welcome_out, group_info }
      }

  }


  async fn main() {
    let sasha = User::new("sasha".to_string());
    let max = User::new("max".to_string());
    let sasha_group = sasha.create_group("sasha_group".to_string());

    let result = sasha_group.add_member(&max.key_package());
    
    let serialized_welcome = result.welcome_out
      .tls_serialize_detached()
      .expect("Error serializing welcome");

    // Maxim can now de-serialize the message as an [`MlsMessageIn`] ...
    let mls_message_in = MlsMessageIn::tls_deserialize(&mut serialized_welcome.as_slice())
    .expect("An unexpected error occurred.");

    // ... and inspect the message.
    let welcome = match mls_message_in.extract() {
      MlsMessageBodyIn::Welcome(welcome) => welcome,
      // We know it's a welcome message, so we ignore all other cases.
      _ => unreachable!("Unexpected message type."),
    };

    max.join_group(welcome);
  }



}



