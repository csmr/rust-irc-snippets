/*
[dependencies]
axum = "0.2.8"
axum-deb = "0.1.0"
serde = {version="1.0.130", features=["derive"]}
tokio = {version="1.12.0", features=["full"]}
*/

use axum::{extract, handler, routing::BoxRoute, AddExtensionLayer, Router};
use axum_debug::{debug_handler, debug_router};
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
    // uncomment to see problem
    //let _form;

    match myformdata {
        Some(_m) => {
            //uncomment to see problem
            //_form = Myform::new();
        }
        None => {
            // uncomment to see problem
            //_form = Myform::new();
        }
    }

    let mut writer = shared_writer.0.lock().await;

    writer.log(&String::from("logthis")).await;

    String::from("myhandler")
}

struct Myform {
    // comment to fix problem
    fields: Vec<Box<dyn Formfield>>,
}

impl Myform {
    fn new() -> Self {
        // comment to fix problem
        Self { fields: vec![] }
        // uncomment to fix problem
        //Self {}
    }
}

impl Form for Myform {
    // comment to fix problem
    fn fields(&self) -> &Vec<Box<dyn Formfield>> {
        &self.fields
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(dead_code)]
pub struct MyformData {}

pub trait Form {
    // comment to fix problem
    fn fields(&self) -> &Vec<Box<dyn Formfield>>;
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
   Compiling reducer2 v0.1.0 (/usr/home/rickame/tmp/reducer2)
error[E0277]: the trait bound `fn(Extension<Arc<tokio::sync::Mutex<Writer>>>, std::option::Option<axum::extract::Form<MyformData>>) -> impl Future {myhandler}: Handler<_, _>` is not satisfied
   --> src/main.rs:33:35
    |
33  |         .route("/", handler::post(myhandler))
    |                     ------------- ^^^^^^^^^ the trait `Handler<_, _>` is not implemented for `fn(Extension<Arc<tokio::sync::Mutex<Writer>>>, std::option::Option<axum::extract::Form<MyformData>>) -> impl Future {myhandler}`
    |                     |
    |                     required by a bound introduced by this call
    |
note: required by a bound in `axum::handler::post`
   --> /home/rickame/.local/share/cargo/registry/src/github.com-1ecc6299db9ec823/axum-0.2.8/src/handler/mod.rs:156:8
    |
156 |     H: Handler<B, T>,
    |        ^^^^^^^^^^^^^ required by this bound in `axum::handler::post`

error[E0599]: the method `boxed` exists for struct `Router<axum::routing::Layered<AddExtension<Route<axum::handler::OnMethod<fn(Extension<Arc<tokio::sync::Mutex<Writer>>>, std::option::Option<axum::extract::Form<MyformData>>) -> impl Future {myhandler}, _, _, EmptyRouter>, EmptyRouter<_>>, Arc<tokio::sync::Mutex<Writer>>>>>`, but its trait bounds were not satisfied
   --> src/main.rs:36:12
    |
36  |     router.boxed()
    |            ^^^^^ method cannot be called on `Router<axum::routing::Layered<AddExtension<Route<axum::handler::OnMethod<fn(Extension<Arc<tokio::sync::Mutex<Writer>>>, std::option::Option<axum::extract::Form<MyformData>>) -> impl Future {myhandler}, _, _, EmptyRouter>, EmptyRouter<_>>, Arc<tokio::sync::Mutex<Writer>>>>>` due to unsatisfied trait bounds
    |
   ::: /home/rickame/.local/share/cargo/registry/src/github.com-1ecc6299db9ec823/axum-0.2.8/src/handler/mod.rs:455:1
    |
455 | pub struct OnMethod<H, B, T, F> {
    | -------------------------------
    | |
    | doesn't satisfy `<_ as tower_service::Service<Request<_>>>::Response = Response<http_body::combinators::box_body::BoxBody<axum::body::Bytes, axum::Error>>`
    | doesn't satisfy `_: tower_service::Service<Request<_>>`
    |
   ::: /home/rickame/.local/share/cargo/registry/src/github.com-1ecc6299db9ec823/axum-0.2.8/src/routing/mod.rs:602:1
    |
602 | pub struct Route<S, F> {
    | ---------------------- doesn't satisfy `_: tower_service::Service<Request<_>>`
    |
   ::: /home/rickame/.local/share/cargo/registry/src/github.com-1ecc6299db9ec823/tower-http-0.1.1/src/add_extension.rs:95:1
    |
95  | pub struct AddExtension<S, T> {
    | ----------------------------- doesn't satisfy `_: tower_service::Service<Request<_>>`
    |
    = note: the following trait bounds were not satisfied:
            `AddExtension<Route<axum::handler::OnMethod<fn(Extension<Arc<tokio::sync::Mutex<Writer>>>, std::option::Option<axum::extract::Form<MyformData>>) -> impl Future {myhandler}, _, _, EmptyRouter>, EmptyRouter<_>>, Arc<tokio::sync::Mutex<Writer>>>: tower_service::Service<Request<_>>`
            `Route<axum::handler::OnMethod<fn(Extension<Arc<tokio::sync::Mutex<Writer>>>, std::option::Option<axum::extract::Form<MyformData>>) -> impl Future {myhandler}, _, _, EmptyRouter>, EmptyRouter<_>>: tower_service::Service<Request<_>>`
            `<axum::handler::OnMethod<fn(Extension<Arc<tokio::sync::Mutex<Writer>>>, std::option::Option<axum::extract::Form<MyformData>>) -> impl Future {myhandler}, _, _, EmptyRouter> as tower_service::Service<Request<_>>>::Response = Response<http_body::combinators::box_body::BoxBody<axum::body::Bytes, axum::Error>>`
            `fn(Extension<Arc<tokio::sync::Mutex<Writer>>>, std::option::Option<axum::extract::Form<MyformData>>) -> impl Future {myhandler}: Handler<_, _>`
            `axum::handler::OnMethod<fn(Extension<Arc<tokio::sync::Mutex<Writer>>>, std::option::Option<axum::extract::Form<MyformData>>) -> impl Future {myhandler}, _, _, EmptyRouter>: tower_service::Service<Request<_>>`

Some errors have detailed explanations: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `reducer2` due to 2 previous errors

with debug_handler macro:
   Compiling reducer2 v0.1.0 (/usr/home/rickame/tmp/reducer2)
warning: unused import: `debug_router`
  --> src/main.rs:10:33
   |
10 | use axum_debug::{debug_handler, debug_router};
   |                                 ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

error: future cannot be sent between threads safely
  --> src/main.rs:42:10
   |
42 | async fn myhandler(
   |          ^^^^^^^^^ future returned by `myhandler` is not `Send`
   |
   = help: the trait `Send` is not implemented for `(dyn Formfield + 'static)`
note: future is not `Send` as this value is used across an await
  --> src/main.rs:62:5
   |
47 |     let _form;
   |         ----- has type `Myform` which is not `Send`
...
62 |     writer.log(&String::from("logthis")).await;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ await occurs here, with `_form` maybe used later
...
65 | }
   | - `_form` is later dropped here
note: required by a bound in `myhandler::{closure#0}::debug_handler`
  --> src/main.rs:42:10
   |
42 | async fn myhandler(
   |          ^^^^^^^^^ required by this bound in `myhandler::{closure#0}::debug_handler`

warning: `reducer2` (bin "reducer2") generated 1 warning
error: could not compile `reducer2` due to previous error; 1 warning emitted
*/
