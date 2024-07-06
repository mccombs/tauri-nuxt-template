use dotenvy::dotenv;
use libsql::Builder;
// use std::time::Duration;
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
async fn test_sync() -> Result<Vec<Item>> {
    // tracing_subscriber::fmt::init();
    dotenv().ok().expect("ERROR: Failed to load dotenv");

    info!("PUNCHOUT: Starting test_sync");

    let db_dir = tempfile::tempdir().unwrap();
    let db_file = db_dir.path().join("data.db");
    println!("Database {}", db_file.display());

    let url = std::env::var("LIBSQL_URL").unwrap();
    let auth_token = std::env::var("LIBSQL_AUTH_TOKEN").unwrap();

    let db =   Builder::new_remote_replica(&db_file, url, auth_token)
    .build()
    .await
    .unwrap();

    let conn = db.connect().unwrap();

    let f = db.sync().await.unwrap();
    println!("inital sync complete, frame no: {f:?}");
    info!("PUNCHOUT: inital sync complete, frame no: {f:?}");

    conn.execute("CREATE TABLE IF NOT EXISTS foo (x TEXT)", ())
        .await
        .unwrap();

    db.sync().await.unwrap();
        let mut items: Vec<Item> = Vec::new();


        let mut results = conn.query("SELECT * FROM states", ()).await.unwrap();

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
    let devtools = tauri_plugin_devtools::init();


    tauri::Builder::default()
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
        .invoke_handler(tauri::generate_handler![test_sync])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
