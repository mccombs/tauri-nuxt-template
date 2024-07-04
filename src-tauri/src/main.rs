use dotenvy::dotenv;
use libsql::Builder;
use serde::{Deserialize, Serialize};
// use tauri_plugin_log::{Target, TargetKind};
use tauri::Manager;
use std::env;
use log::info;
// use tracing::info;

#[derive(Serialize, Debug)]
struct Error {
    msg: String,
}

type Result<T> = std::result::Result<T, Error>;

impl<T> From<T> for Error
where
    T: std::error::Error,
{
    fn from(value: T) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Item {
    id: i32,
    text: String,
}
#[tauri::command]
async fn test_command() -> String {
    return "Hello from Rust".to_string();
}


#[tauri::command]
async fn get_all_notes() -> Result<Vec<Item>> {
    info!("PUNCHOUT: Getting all notes");
    dotenv().ok();

    // for (key, value) in env::vars() {
    //     println!("PUNCHOUT - {}: {}", key, value);
    // }
    let db_path = env::var("DB_PATH").unwrap();
    let sync_url = env::var("TURSO_SYNC_URL").unwrap();
    let auth_token = env::var("TURSO_AUTH_TOKEN").unwrap();


    let db = Builder::new_remote_replica(db_path, sync_url.to_string(), auth_token.to_string())
        .build()
        .await
        .unwrap();
    // db.sync().await.unwrap();


    let conn = db.connect().unwrap();
    
    // info!("PUNCHOUT: Connected to database: {:?}", conn);

    let mut results = conn.query("SELECT * FROM states", ()).await.unwrap();

    let mut items: Vec<Item> = Vec::new();
    while let Ok(Some(row)) = results.next().await {
        let item: Item = Item {
            id: row.get(0).unwrap(),
            text: row.get(1).unwrap(),
        };
        items.push(item);
    }
    info!("{:?}", items);

    Ok(items)
}

fn main() {
    // tracing_subscriber::fmt::init();
    let devtools = tauri_plugin_devtools::init();

    tauri::Builder::default()
        // .plugin(tauri_plugin_updater::Builder::new().build())
        // .plugin(tauri_plugin_log::Builder::new().targets([
        //     Target::new(TargetKind::Stdout),
        //     Target::new(TargetKind::LogDir { file_name: None }),
        //     Target::new(TargetKind::Webview),
        // ]).build())
        .plugin(devtools)
        .plugin(tauri_plugin_shell::init())
        .setup(|_app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = _app.get_webview_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_all_notes, test_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
