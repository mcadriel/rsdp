use actix_web::{web, App, HttpServer};
use clap::Parser;
use std::sync::Arc;
use tokio::sync::RwLock;

use csv_lib::{AppState, get_json, load_csv, upload_csv};

#[derive(Parser, Debug)]
#[command(name = "CSV to JSON API")]
struct Args {
    /// Optional path to CSV file
    #[arg(short, long)]
    file: Option<String>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let records = if let Some(path) = args.file {
        load_csv(&path).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    };

    let state = AppState {
        data: Arc::new(RwLock::new(records)),
    };

    println!("Server running at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/data", web::get().to(get_json))
            .route("/upload", web::post().to(upload_csv))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}