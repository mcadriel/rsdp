use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Result};
use csv::ReaderBuilder;
use futures_util::StreamExt as _;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Clone)]
pub struct AppState {
    pub data: Arc<RwLock<Vec<Record>>>,
}

pub async fn get_json(state: web::Data<AppState>) -> impl actix_web::Responder {
    let data = state.data.read().await;
    HttpResponse::Ok().json(&*data)
}

pub async fn upload_csv(
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
    *data = records.clone();

    Ok(HttpResponse::Ok().json(records))
}

pub fn load_csv(file_path: &str) -> std::io::Result<Vec<Record>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    let mut records = vec![];
    for result in rdr.deserialize() {
        let record: Record = result?;
        records.push(record);
    }
    Ok(records)
}