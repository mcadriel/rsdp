use actix_multipart::Multipart;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use clap::Parser;
use csv::ReaderBuilder;
use futures_util::StreamExt as _;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::sync::Arc;
use tokio::sync::RwLock;

/// CLI arguments
#[derive(Parser, Debug)]
#[command(name = "CSV to JSON API")]
struct Args {
    /// Optional path to CSV file
    #[arg(short, long)]
    file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Record {
    id: String,
    name: String,
    email: String,
}

#[derive(Clone)]
struct AppState {
    data: Arc<RwLock<Vec<Record>>>,
}

/// GET: Return stored CSV data as JSON
async fn get_json(state: web::Data<AppState>) -> impl Responder {
    let data = state.data.read().await;
    HttpResponse::Ok().json(&*data)
}

/// POST: Upload a CSV file, parse it, and return as JSON
async fn upload_csv(
    mut payload: Multipart,
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    let mut bytes = web::BytesMut::new();

    while let Some(field) = payload.next().await {
        let mut field = field?;

        while let Some(chunk) = field.next().await {
            bytes.extend_from_slice(&chunk?);
        }
    }

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(bytes.as_ref());

    let mut records = vec![];

    for result in rdr.deserialize() {
        let record: Record = result.map_err(|e| {
            actix_web::error::ErrorBadRequest(format!("CSV Parse Error: {}", e))
        })?;
        records.push(record);
    }

    let mut data = state.data.write().await;
    *data = records.clone(); // update shared state

    Ok(HttpResponse::Ok().json(records))
}

/// Read CSV from file
fn load_csv(file_path: &str) -> std::io::Result<Vec<Record>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut records = vec![];
    for result in rdr.deserialize() {
        let record: Record = result?;
        records.push(record);
    }
    Ok(records)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let records = if let Some(file_path) = args.file {
        load_csv(&file_path).expect("Failed to load CSV")
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