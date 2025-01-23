use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

use ring::digest;
use std::num::NonZeroU32;

mod config;
mod crypto;

const ENCRYPTION_KEY_LEN: usize = digest::SHA256_OUTPUT_LEN;
const SALT_LEN: usize = ENCRYPTION_KEY_LEN / 2;
pub type EncryptionKey = [u8; ENCRYPTION_KEY_LEN];
pub type SecretKey = [u8; SALT_LEN];

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn generate_secret_key() -> String {
    crypto::generate_secret_key()
}

#[tauri::command]
fn create_encryption_key(
    state: tauri::State<config::AppState>,
    email: &str,
    password: &str,
    secret_key: &str,
) -> String {
    crypto::create_encryption_key(
        state.keygen_config.pbkdf2_iterations,
        email,
        password,
        secret_key,
    )
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
    let encrypted_vectors: Vec<Vec<f32>> = crypto::encrypt_embeddings(&embeddings, key).await?;

    Ok(encrypted_vectors)
}

#[tauri::command]
async fn encrypt_text(text: &str, key: &str) -> Result<crypto::ChatLockedEncryptedDocument, ()> {
    let document = crypto::encrypt_text(text, key).await?;

    Ok(document)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(config::AppState {
            keygen_config: config::KeygenConfig {
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
            encrypt_text,
            generate_secret_key,
            create_encryption_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
