/*
edition = "2021"
[dependencies]
axum = "0.2.8"
axum-debug = "0.1.0"
serde = {version="1.0.130", features=["derive"]}
tokio = {version="1.12.0", features=["full"]}
*/

use axum::{extract, handler, routing::BoxRoute, AddExtensionLayer, Router};
use axum_debug::debug_handler;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let writer = Writer::new().await;

    let shared_writer = Arc::new(Mutex::new(writer));

    let router = router(shared_writer.clone());

    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 8000)))
        .serve(router.into_make_service())
        .await
        .unwrap();
}

fn router(shared_writer: Arc<Mutex<Writer>>) -> Router<BoxRoute> {
    let router = Router::new()
        .route("/", handler::post(myhandler))
        .layer(AddExtensionLayer::new(shared_writer));

    router.boxed()
}

#[debug_handler]
async fn myhandler(
    shared_writer: extract::Extension<Arc<Mutex<Writer>>>,
    myformdata: Option<extract::Form<MyformData>>,
) -> String {
    let _form;

    match myformdata {
        Some(_m) => {
            _form = Myform::new();
        }
        None => {
            _form = Myform::new();
        }
    }

    let mut writer = shared_writer.0.lock().await;

    writer.log(&String::from("logthis")).await;

    String::from("myhandler")
}

struct Myform {
    fields: Vec<Box<dyn Formfield + Send>>,
}

impl Myform {
    fn new() -> Self {
        Self { fields: vec![] }
    }
}

impl Form for Myform {
    fn fields(&self) -> &Vec<Box<dyn Formfield + Send>> {
        &self.fields
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(dead_code)]
pub struct MyformData {}

pub trait Form {
    fn fields(&self) -> &Vec<Box<dyn Formfield + Send>>;
}

pub trait Formfield {}

pub struct Writer {
    file: File,
}

impl Writer {
    #[must_use]
    pub async fn new() -> Self {
        let file = tokio::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(PathBuf::from("/tmp/rustwriter.log"))
            .await
            .unwrap();

        Self { file }
    }

    pub async fn log(&mut self, msg: &String) {
        self.file.write_all(msg.as_bytes()).await.unwrap();
    }
}

/* problem:
*/
