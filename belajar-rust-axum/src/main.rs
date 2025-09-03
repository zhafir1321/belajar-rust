/*
AXUM
- Axum adalah salah satu library untuk di Rust yang populer
- Axum terintegrasi baik dengan library Tokio, sehingga jika kita sudah terbiasa menggunakan Tokio,
kita bisa menggunakan Axum dengan mudah
- Axum merupakan library minimalis, sehingga sangat mudah menggunakannya
- Axum menggunakan ekosistem library yang sudah ada, seperti Tokio, Tower, dan Hyper,
sehingga memudahkan ketika menggunakan library - library yang sudah terintegrasi dengan ekosistem tersebut dengan baik

SETUP
- Untuk menggunakan Axum, pertama kita perlu membuat aplikasi Axum dalam bentuk Router
- Selanjutnya, kita perlu membuat Listener Axum untuk menentukan ip dan port web akan dijalankan
- Selanjutnya, kita bisa menjalankan aplikasi Axum menggunakan method serve
*/

fn init_logging() {
    env_logger::init();
}

use axum::{
    Router,
    extract::Request,
    routing::{get, post},
    serve,
};
use axum_test::TestServer;
use log::debug;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}

/*
TEST
- Saat kita membuat web menggunakan Axum, kadang agak menyulitkan jika kita harus melakukan test secara manual
- Untungnya, ada library untuk integration test Axum, sehingga kita tidak perlu menjalankan web Axum secara manual lagi
*/

#[tokio::test]
async fn test_axum() {
    let app = Router::new().route("/test", get(|| async { "Hello, Test!" }));

    let server = TestServer::new(app).unwrap();

    let response = server.get("/test").await;
    println!("Response: {:?}", response);

    response.assert_status_ok();
    response.assert_text("Hello, Test!");
}

/*
ROUTER
- Struct Router merupakan object utama dalam Axum yang digunakan untuk memetakan antara path dan kode yang dieksekusi ketika path tersebut diakses
- Untuk menambahkan pemetaan baru, kita bisa menggunakan method `route(path, method_router)`

PATH
- Path di Router merupakan tipe data &str yang berisikan lokasi Path di URL
- `/`, `/path`, `/path/nested/path`
- Pembuatan path harus dipastikan unik, jika ada yang duplikat, maka akan terjadi error

METHOD ROUTER
- Method di Router merupakan implementasi dari Struct MethodRouter
- Untuk membuat MethodRouter, kita bisa menggunakan function helper yang sudah disediakan untuk mempermudah,
yang berada di module Axum::routing
- Nama - nama function disesuaikan dengan nama HTTP method
*/

#[tokio::test]
async fn test_method_routing() {
    init_logging();
    async fn hello_world() -> String {
        "Hello, World!".to_string()
    }

    let app = Router::new()
        .route("/get", get(hello_world))
        .route("/post", post(hello_world));

    let server = TestServer::new(app).unwrap();
    let response = server.get("/get").await;
    debug!("Response GET: {}", response.text());
    response.assert_status_ok();
    response.assert_text("Hello, World!");

    let response = server.post("/post").await;
    debug!("Response POST: {}", response.text());
    response.assert_status_ok();
    response.assert_text("Hello, World!");
}

/*
REQUEST
- Saat kita membuat routing (pemetaan path dan kode), kadang kita ingin mendapatkan informasi Request yang dikirim oleh pengguna
- Data Request di Axum direpresentasikan dalam bentuk `axum::extract::Request`, dimana ini sebenernya adalah alias untuk `http::Request`
- Dari object Request, kita bisa mendapatkan seluruh informasi HTTP Request yang kita butuhkan
*/

#[tokio::test]
async fn test_request() {
    init_logging();
    async fn hello_world(request: Request) -> String {
        debug!("Request: {:?}", request);
        format!("Hello, {}!", request.method())
    }

    let app = Router::new()
        .route("/get", get(hello_world))
        .route("/post", post(hello_world));

    let server = TestServer::new(app).unwrap();

    let response = server.get("/get").await;
    debug!("Response GET: {}", response.text());
    response.assert_status_ok();
    response.assert_text("Hello, GET!");

    let response = server.post("/post").await;
    debug!("Response POST: {}", response.text());
    response.assert_status_ok();
    response.assert_text("Hello, POST!");
}
