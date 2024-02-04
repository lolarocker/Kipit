
use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use std::collections::HashMap;
use serde::Serialize;
use ink_lang as ink;

mod lib;

#[derive(Serialize)]
struct User {
    name: String,
    tarjetas_debito: Vec<String>,
    tarjetas_credito: Vec<String>,
    wallets: Vec<String>,
    documents: HashMap<String, String>,
}

#[ink::contract]
mod simple_storage {
    use ink_storage::collections::HashMap as StorageHashMap;
    use ink_prelude::string::String;
    use ink_lang as ink;

    #[ink(storage)]
    pub struct SimpleStorage {
        values: StorageHashMap<String, i32>,
        documents: StorageHashMap<String, String>,
    }

    #[ink(event)]
    pub struct DocumentAdded {
        key: String,
        value: String,
    }

    impl SimpleStorage {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                values: StorageHashMap::new(),
                documents: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn set_value(&mut self, key: String, value: i32) {
            self.values.insert(key, value);
        }

        #[ink(message)]
        pub fn get_value(&self, key: String) -> Option<i32> {
            self.values.get(&key).copied()
        }

        #[ink(message)]
        pub fn upload_document(&mut self, key: String, value: String) {
            self.documents.insert(key.clone(), value.clone());
            self.env().emit_event(DocumentAdded { key, value });
        }

        #[ink(message)]
        pub fn get_document(&self, key: String) -> Option<&String> {
            self.documents.get(&key)
        }
    }
}

async fn get_documents(_req: HttpRequest) -> impl Responder {
    let user = User {
        name: "Juan Pérez".to_string(),
        tarjetas_debito: vec!["VISA 1234".to_string(), "MASTERCARD 5678".to_string()],
        tarjetas_credito: vec!["AMEX 9012".to_string(), "DISCOVER 3456".to_string()],
        wallets: vec!["Blockchain.com".to_string(), "Metamask".to_string()],
        documents: HashMap::from([
            ("INE".to_string(), "Contenido de INE".to_string()),
            ("Comprobante de domicilio".to_string(), "Contenido de comprobante de domicilio".to_string()),
            ("Acta de nacimiento".to_string(), "Contenido de acta de nacimiento".to_string()),
        ]),
    };

    let mut context = lib::Context::new();
    context.insert("user", &user);

    let template_result = lib::render("templates/documents.html", &context);
    match template_result {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(e) => HttpResponse::InternalServerError().body(format!("Template error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/get_documents", web::get().to(get_documents))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
use actix_web::{web, App, HttpServer};

mod lib;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/faces").configure(lib::faces_config))
            .service(web::scope("/documents").configure(lib::documents_config))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}