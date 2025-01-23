use std::{collections::HashMap, num::NonZeroU32, sync};

use ironcore_alloy::{
    self, standalone,
    standard::{self, StandardDocumentOps},
    vector::{self, VectorOps},
};
use ring::{digest, pbkdf2, rand};

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA512;
const ENCRYPTION_KEY_LEN: usize = digest::SHA256_OUTPUT_LEN;
const SALT_LEN: usize = ENCRYPTION_KEY_LEN / 2;
pub type EncryptionKey = [u8; ENCRYPTION_KEY_LEN];
pub type SecretKey = [u8; SALT_LEN];

pub struct ChatLockedEncryptedDocument(pub standard::EncryptedDocument);

#[derive(serde::Serialize)]
pub struct EncryptedDocumentWrapper {
    edek: String,
    document: HashMap<String, String>,
}

struct IroncoreStandalone {
    secret_path: ironcore_alloy::SecretPath,
    derivation_path: ironcore_alloy::DerivationPath,
    config: sync::Arc<standalone::config::StandaloneConfiguration>,
}

impl serde::Serialize for ChatLockedEncryptedDocument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let edek: String = hex::encode(self.0.edek.0.clone());
        let data = &self.0.document;
        let document: HashMap<String, String> = data
            .iter()
            .map(|(field_id, encrypted_bytes)| {
                (field_id.0.clone(), hex::encode(encrypted_bytes.0.clone()))
            })
            .collect();
        let wrapper = EncryptedDocumentWrapper { edek, document };
        wrapper.serialize(serializer)
    }
}

pub fn generate_secret_key() -> String {
    match rand::generate::<SecretKey>(&rand::SystemRandom::new()) {
        Ok(key_bytes) => hex::encode(key_bytes.expose())
            .to_ascii_uppercase()
            .chars()
            .enumerate()
            .flat_map(|(i, character)| {
                if i != 0 && i % 5 == 0 {
                    Some('-')
                } else {
                    None
                }
                .into_iter()
                .chain(std::iter::once(character))
            })
            .collect::<String>(),
        Err(err) => {
            println!("{}", err);
            String::from("Uh oh.")
        }
    }
}

fn salt(username: &str, secret_key: &str) -> Vec<u8> {
    let mut salt = Vec::with_capacity(SALT_LEN + username.as_bytes().len());
    salt.extend(secret_key.as_bytes());
    salt.extend(username.as_bytes());
    salt
}

pub fn create_encryption_key(
    pbkdf2_iterations: NonZeroU32,
    email: &str,
    password: &str,
    secret_key: &str,
) -> String {
    let salt: Vec<u8> = salt(email, secret_key);
    let mut key: EncryptionKey = [0u8; ENCRYPTION_KEY_LEN];
    pbkdf2::derive(
        PBKDF2_ALG,
        pbkdf2_iterations,
        &salt,
        password.as_bytes(),
        &mut key,
    );
    hex::encode(key)
}

fn ironcore_config(key: &str, vector: Option<bool>) -> IroncoreStandalone {
    let secret_path = ironcore_alloy::SecretPath("key".into());
    let derivation_path = ironcore_alloy::DerivationPath("message".into());
    let standard_secrets = standalone::config::StandardSecrets::new(
        Some(1),
        vec![standalone::config::StandaloneSecret::new(
            1,
            ironcore_alloy::Secret::new(hex::decode(key).unwrap()).unwrap(),
        )],
    )
    .unwrap();
    let vector_secrets = match vector {
        Some(true) => HashMap::from([(
            secret_path.clone(),
            standalone::config::VectorSecret::new(
                2.0,
                standalone::config::RotatableSecret::new(
                    Some(standalone::config::StandaloneSecret::new(
                        1,
                        ironcore_alloy::Secret::new(hex::decode(key).unwrap()).unwrap(),
                    )),
                    None,
                )
                .unwrap(),
            ),
        )]),
        _ => HashMap::new(),
    };
    let config: sync::Arc<standalone::config::StandaloneConfiguration> =
        standalone::config::StandaloneConfiguration::new(
            standard_secrets,
            HashMap::new(),
            vector_secrets,
        );
    IroncoreStandalone {
        secret_path,
        derivation_path,
        config,
    }
}

pub async fn encrypt_embeddings(
    embeddings: &Vec<Vec<f32>>,
    key: &str,
) -> Result<Vec<Vec<f32>>, ()> {
    let standalone_config = ironcore_config(key, Some(true));

    let standalone: sync::Arc<ironcore_alloy::Standalone> =
        ironcore_alloy::Standalone::new(&standalone_config.config);

    let mut encrypted_vectors: Vec<Vec<f32>> = Vec::new();

    for embedding in embeddings {
        let plaintext_vector = vector::PlaintextVector {
            plaintext_vector: embedding.clone(),
            secret_path: standalone_config.secret_path.clone(),
            derivation_path: standalone_config.derivation_path.clone(),
        };

        let metadata: sync::Arc<ironcore_alloy::AlloyMetadata> =
            ironcore_alloy::AlloyMetadata::new_simple(ironcore_alloy::TenantId("Personal".into()));

        let encrypted_vector = standalone
            .vector()
            .encrypt(plaintext_vector, &metadata)
            .await
            .unwrap();
        encrypted_vectors.push(encrypted_vector.encrypted_vector);
    }

    let encrypted_vectors: Vec<Vec<f32>> = encrypted_vectors;

    Ok(encrypted_vectors)
}

pub async fn encrypt_text(text: &str, key: &str) -> Result<ChatLockedEncryptedDocument, ()> {
    let standalone_config = ironcore_config(key, None);
    let standalone: sync::Arc<ironcore_alloy::Standalone> =
        ironcore_alloy::Standalone::new(&standalone_config.config);

    let metadata: sync::Arc<ironcore_alloy::AlloyMetadata> =
        ironcore_alloy::AlloyMetadata::new_simple(ironcore_alloy::TenantId("Personal".into()));

    let mut document_data: HashMap<ironcore_alloy::FieldId, ironcore_alloy::PlaintextBytes> =
        HashMap::new();
    document_data.insert(
        "message".to_string().into(),
        Vec::from(text.as_bytes()).into(),
    );
    let document_data = document_data;

    let document = standard::PlaintextDocument(document_data);

    let encrypted_text = standalone
        .standard()
        .encrypt(document, &metadata)
        .await
        .unwrap();

    Ok(ChatLockedEncryptedDocument(encrypted_text))
}
