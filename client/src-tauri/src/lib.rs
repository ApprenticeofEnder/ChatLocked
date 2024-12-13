// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn embed_string(text: &str) -> String {
    let model = TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true),
    ).unwrap();

    let embeddings = model.embed(vec![text], None).unwrap();
    println!("Embedding dimension: {}", embeddings[0].len());
    String::from("Hello")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, embed_string])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
