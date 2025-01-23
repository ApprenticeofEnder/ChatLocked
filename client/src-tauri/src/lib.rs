use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use hex;
use ironcore_alloy::{
    standalone::config::{
        RotatableSecret, StandaloneConfiguration, StandaloneSecret, StandardSecrets, VectorSecret,
    },
    vector::{PlaintextVector, VectorOps},
    AlloyMetadata, DerivationPath, Secret, SecretPath, Standalone, TenantId,
};
use ring::{digest, pbkdf2, rand};
use std::{collections::HashMap, num::NonZeroU32, sync::Arc};

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA512;
const ENCRYPTION_KEY_LEN: usize = digest::SHA256_OUTPUT_LEN;
const SALT_LEN: usize = ENCRYPTION_KEY_LEN / 2;
pub type EncryptionKey = [u8; ENCRYPTION_KEY_LEN];
pub type SecretKey = [u8; SALT_LEN];

enum Error {
    WrongUsernameOrPassword,
}

struct AppState {
    keygen_config: KeygenConfig,
}

struct KeygenConfig {
    pbkdf2_iterations: NonZeroU32,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn generate_secret_key() -> String {
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

#[tauri::command]
fn create_encryption_key(
    state: tauri::State<AppState>,
    email: &str,
    password: &str,
    secret_key: &str,
) -> String {
    let salt: Vec<u8> = salt(email, secret_key);
    let mut key: EncryptionKey = [0u8; ENCRYPTION_KEY_LEN];
    pbkdf2::derive(
        PBKDF2_ALG,
        state.keygen_config.pbkdf2_iterations,
        &salt,
        password.as_bytes(),
        &mut key,
    );
    hex::encode(key)
}

#[tauri::command]
fn embed_string(text: &str) -> Vec<Vec<f32>> {
    let model = TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true),
    )
    .unwrap();

    let embeddings: Vec<Vec<f32>> = model.embed(vec![text], None).unwrap();
    println!("Embedding dimension: {}", embeddings[0].len());
    embeddings
}

#[tauri::command]
async fn encrypt_embeddings(embeddings: Vec<Vec<f32>>, key: &str) -> Result<Vec<Vec<f32>>, ()> {
    let secret_path = SecretPath("key".into());
    let derivation_path = DerivationPath("sentence".into());
    let config: Arc<StandaloneConfiguration> = StandaloneConfiguration::new(
        StandardSecrets::new(
            Some(1),
            vec![StandaloneSecret::new(
                1,
                Secret::new(hex::decode(key).unwrap()).unwrap(),
            )],
        )
        .unwrap(),
        HashMap::new(),
        HashMap::from([(
            secret_path.clone(),
            VectorSecret::new(
                2.0,
                RotatableSecret::new(
                    Some(StandaloneSecret::new(
                        1,
                        Secret::new(hex::decode(key).unwrap()).unwrap(),
                    )),
                    None,
                )
                .unwrap(),
            ),
        )]),
    );

    let standalone: Arc<Standalone> = Standalone::new(&config);

    let mut encrypted_vectors: Vec<Vec<f32>> = Vec::new();

    for embedding in embeddings {
        let plaintext_vector = PlaintextVector {
            plaintext_vector: embedding.clone(),
            secret_path: secret_path.clone(),
            derivation_path: derivation_path.clone(),
        };

        let metadata: Arc<AlloyMetadata> = AlloyMetadata::new_simple(TenantId("Personal".into()));

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            keygen_config: KeygenConfig {
                pbkdf2_iterations: NonZeroU32::new(100_000).unwrap(),
            },
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_stronghold::Builder::new(|password| {
                use argon2::{hash_raw, Config, Variant, Version};
                let config = Config {
                    lanes: 4,
                    mem_cost: 10_000,
                    time_cost: 10,
                    variant: Variant::Argon2id,
                    version: Version::Version13,
                    ..Default::default()
                };

                let salt = "your-salt".as_bytes();

                let key =
                    hash_raw(password.as_ref(), salt, &config).expect("failed to hash password");

                key.to_vec()
            })
            .build(),
        )
        .invoke_handler(tauri::generate_handler![
            greet,
            embed_string,
            encrypt_embeddings,
            generate_secret_key,
            create_encryption_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
