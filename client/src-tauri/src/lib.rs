use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use hex;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA512;
const ENCRYPTION_KEY_LEN: usize = digest::SHA512_OUTPUT_LEN;
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
fn create_key(state: tauri::State<AppState>, username: &str, password: &str, secret_key: &str) {
    let salt: Vec<u8> = salt(username, secret_key);
    let mut key: EncryptionKey = [0u8; ENCRYPTION_KEY_LEN];
    pbkdf2::derive(PBKDF2_ALG, state.keygen_config.pbkdf2_iterations, &salt, password.as_bytes(), &mut key);
    println!("{}", hex::encode(key))
}

#[tauri::command]
fn embed_string(text: &str) -> String {
    let model = TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true),
    )
    .unwrap();

    let embeddings = model.embed(vec![text], None).unwrap();
    println!("Embedding dimension: {}", embeddings[0].len());
    String::from("Hello")
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
        .invoke_handler(tauri::generate_handler![
            greet,
            embed_string,
            generate_secret_key,
            create_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
