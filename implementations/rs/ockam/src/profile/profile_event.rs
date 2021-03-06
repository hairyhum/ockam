use crate::profile::error::Error;
use crate::profile::profile::ProfileEventAttributes;
use crate::profile::profile_event_binary_model::ProfileEventBinaryModel;
use crate::profile::ProfileVault;
use ockam_common::error::OckamResult;
use ockam_vault::types::{SecretAttributes, SecretPersistence, SecretType};
use ockam_vault::Secret;
use std::sync::{Arc, Mutex};

pub struct ProfileEvent {
    version: u8,
    identifier: String,
    model_binary: Vec<u8>,
    // TODO: Check attributes serialization
    attributes: ProfileEventAttributes,
    public_key: Option<Vec<u8>>,
    prev_event_id: Option<String>,
    next_event_id: Option<String>,
    private_key: Option<Box<dyn Secret>>,
    self_signature: Option<[u8; 64]>,
    previous_self_signature: Option<[u8; 64]>,
}

impl ProfileEvent {
    pub fn version(&self) -> u8 {
        self.version
    }
    pub fn identifier(&self) -> &str {
        &self.identifier
    }
    pub fn model_binary(&self) -> &Vec<u8> {
        &self.model_binary
    }
    pub fn attributes(&self) -> &ProfileEventAttributes {
        &self.attributes
    }
    pub fn public_key(&self) -> &Option<Vec<u8>> {
        &self.public_key
    }
    pub fn prev_event_id(&self) -> &Option<String> {
        &self.prev_event_id
    }
    pub fn next_event_id(&self) -> &Option<String> {
        &self.next_event_id
    }
    pub fn private_key(&self) -> &Option<Box<dyn Secret>> {
        &self.private_key
    }
    pub fn self_signature(&self) -> Option<[u8; 64]> {
        self.self_signature
    }
    pub fn previous_self_signature(&self) -> Option<[u8; 64]> {
        self.previous_self_signature
    }
}

impl ProfileEvent {
    pub(crate) fn take_private_key(&mut self) -> Option<Box<dyn Secret>> {
        self.private_key.take()
    }

    pub fn new(
        is_revoke: bool,
        attributes: ProfileEventAttributes,
        previous_event: Option<&ProfileEvent>,
        vault: Arc<Mutex<dyn ProfileVault>>,
    ) -> OckamResult<Self> {
        let mut vault = vault.lock().unwrap();

        let keys = (|| {
            if is_revoke {
                Ok((None, None))
            } else {
                let attributes = SecretAttributes {
                    stype: SecretType::Curve25519,
                    persistence: SecretPersistence::Persistent,
                    length: 0,
                };

                let private_key = vault.secret_generate(attributes)?;
                let public_key = vault.secret_public_key_get(&private_key)?.as_ref().to_vec();

                Ok((Some(private_key), Some(public_key)))
            }
        })()?;

        let prev_event_id = match previous_event {
            Some(event) => Some(event.identifier.clone()),
            None => None,
        };

        let model = ProfileEventBinaryModel::new(
            1,
            keys.1.clone(),
            attributes.clone(),
            prev_event_id.clone(),
            None,
        );
        let model_binary: Vec<u8> =
            serde_bare::to_vec(&model).map_err(|_| Error::BareError.into())?;
        let identifier = vault.sha256(&model_binary)?;
        let self_signature = match &keys.0 {
            Some(s) => Some(vault.sign(s, &identifier)?),
            None => None,
        };

        let previous_self_signature = match previous_event {
            Some(event) => {
                let private_key: &Box<dyn Secret>;
                if let Some(key) = event.private_key() {
                    private_key = key;
                } else {
                    return Err(Error::InvalidInternalState.into());
                }
                Some(vault.sign(private_key, &identifier)?)
            }
            None => None,
        };

        let identifier = format!("E_ID.{}", hex::encode(&identifier));

        Ok(ProfileEvent {
            version: 1,
            identifier,
            model_binary,
            attributes,
            public_key: keys.1,
            prev_event_id,
            next_event_id: None,
            private_key: keys.0,
            self_signature,
            previous_self_signature,
        })
    }
}
