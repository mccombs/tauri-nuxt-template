use dotenvy::dotenv;
use libsql::{params, Builder};
use serde::{Deserialize, Serialize};
use std::env;

use tracing::info;

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
    id: String,
    text: String,
}

#[tauri::command]
async fn get_all_notes() -> () {
    info!("Getting all notes");
    dotenv().expect(".env file not found");

    let db_path = env::var("DB_PATH").unwrap();
    let sync_url = env::var("TURSO_SYNC_URL").unwrap();
    let auth_token = env::var("TURSO_AUTH_TOKEN").unwrap();

    let db = Builder::new_remote_replica(db_path, sync_url, auth_token).build().await.unwrap();
    db.sync().await.unwrap();
    // let conn = db.connector();

    // let mut results = conn
    //     .query("SELECT * FROM table_name", ())
    //     .await?;

    // let mut items: Vec<Item> = Vec::new();
    // while let Some(row) = results.next()? {
    //     let item: Item = Item {
    //         id: row.get(0)?,
    //         text: row.get(1)?
    //     };
    //     items.push(item);
    // }

    // Ok(items)
}

fn main() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_all_notes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
