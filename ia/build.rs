use std::collections::HashMap;
use vara::prelude::*;

pub struct Context {
    inner: vara::Context,
}

impl Context {
    pub fn new() -> Self {
        Context {
            inner: vara::Context::new(),
        }
    }

    pub fn insert<T: Serialize>(&mut self, key: &str, value: &T) {
        self.inner.insert(key, value);
    }
}

pub fn render(template_path: &str, context: &Context) -> Result<String, vara::Error> {
    vara::render(template_path, &context.inner)
}

//

use actix_web::{web, HttpResponse, Responder};
use reqwest::{multipart::Form, Error};
use serde::{Deserialize, Serialize};
use std::{env, fs};
use std::str;

#[derive(Serialize, Deserialize)]
struct IneResponse {
    resultado: String,
}

async fn validate_ine(file: &str) -> Result<bool, Error> {
    let url = "https://api.ine.gob.mx/api/v1/verificar-credencial";
    let form = Form::new().file("archivo", file)?;
    let response = reqwest::Client::new().post(url).multipart(form).send().await?;
    let json: IneResponse = response.json().await?;
    Ok(json.resultado == "VALIDA")
}

async fn upload_file_to_drive(file: &str, folder_id: &str) -> Result<String, Error> {
    let token = env::var("GOOGLE_DRIVE_API_TOKEN")?;
    let url = format!("https://www.googleapis.com/upload/drive/v3/files?uploadType=media&folderId={}", folder_id);
    let response = reqwest::Client::new()
        .post(&url)
        .bearer_auth(&token)
        .header("Content-Type", "application/pdf")
        .body(reqwest::Body::from(file))
        .send()
        .await?;
    let json: serde_json::Value = response.json().await?;
    Ok(json["id"].as_str().unwrap().to_string())
}

async fn handle_file_upload(mut payload: web::Payload) -> impl Responder {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = payload.next().await {
        bytes.extend_from_slice(&item?);
    }
    let file = String::from_utf8(bytes.to_vec()).unwrap();
    if file.len() == 0 {
        return HttpResponse::BadRequest().json(serde_json::json!({"success": false, "error": "File is empty"}));
    }
    if !file.ends_with(".pdf") {
        return HttpResponse::BadRequest().json(serde_json::json!({"success": false, "error": "File is not a PDF"}));
    }
    if validate_ine(&file).await.unwrap() {
        let folder_id = "your-folder-id-here";
        let file_id = upload_file_to_drive(&file, &folder_id).await.unwrap();
        HttpResponse::Ok().json(serde_json::json!({"success": true, "file_id": file_id}))
    } else {
        HttpResponse::Ok().json(serde_json::json!({"success": false, "error": "INE document is not valid"}))
    }
}

#[actix_web::get("/visualize_document")]
async fn visualize_document() -> impl Responder {
    // Implementar la lógica para visualizar documentos
    HttpResponse::Ok().body("Visualizing Document")
}

#[actix_web::post("/receive_notification")]
async fn receive_notification() -> impl Responder {
    // Implementar la lógica para recibir notificaciones
    HttpResponse::Ok().body("Received Notification")
}

pub fn faces_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/upload").route(web::post().to(handle_file_upload)));
}

pub fn documents_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/visualize_document").route(web::get().to(visualize_document)))
        .service(web::resource("/receive_notification").route(web::post().to(receive_notification)));
}