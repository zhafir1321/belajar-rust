use std::env::set_var;

use config::{Case, Config, Environment, File, FileFormat};
use serde::Deserialize;

fn main() {
    println!("Hello, world!");
}

/*
CONFIGURATION
- Saat kita membuat aplikasi, kita pasti tidak akan pernah menulis konfigurasi langsung di kode aplikasi Rust kita
- Contoh, kita perlu melakukan koneksi ke database dari aplikasi yang kita buat
- Saat di komputer pribadi, lokasi database mungkin sama dengan lokasi aplikasi, namun ketika nanti aplikasi berjalan di server,
lokasi database pun bisa berbeda
- Dengan demikian, konfigurasi aplikasi tidak bisa kita tulis (hardcode) di dalam kode program
- Kita butuh menyimpan konfigurasi aplikasi terpisah dari aplikasi kita

CONFIGURATION LIBRARY
- Biasanya, ada banyak sekali lokasi untuk menyimpan konfigurasi, misal di environment variable sistem operasi,
atau di file (JSON, XML, INI, TOML, dan lain - lain)
- Sayangnya, Rust tidak menyediakan standard library untuk membaca konfigurasi, oleh karena itu,
kita perlu library untuk configuration

CONFIG LIBRARY
- Salah satu library yang populer untuk management configuration adalah library Config
- Config merupakan library yang opensource dan juga gratis, sehingga bisa kita gunakan secara bebas di aplikasi yang kita buat
- Config mendukung banyak jenis konfigurasi file, seperti INI, JSON, TOML, dan lain - lain

CONFIG STRUCT
- Config library memiliki struct bernama Config, yang digunakan sebagai struct utama untuk melakukan management konfigurasi
- Kita bisa membuat object Config, lalu kita bisa menambahkan sumber konfigurasi dari beberapa tempat
- Struct Config juga memiliki banyak method yang bisa kita gunakan untuk mendapatkan informasi konfigurasi dengan prefix `get_`
*/

#[test]
fn test_config() {
    let config = Config::builder().build().unwrap();

    assert!(config.get_string("APP_NAME").is_err());
}

/*
ENVIRONMENT VARIABLE
- Environment Variable adalah salah satu tempat yang umum digunakan untuk menyimpan konfigurasi
- Hal ini karena bisa mudah tanpa harus membuat file apapun, kita cukup tambahkan environment variable pada sistem operasi yang kita gunakan
- Untuk mengambil data dari env variable, kita bisa gunakan struct Environment

CASE
- Secara default, key di environment variable akan diubah menjadi lower_snake
- Jika kita ingin mengubahnya, kita bisa ubah field convert_case pada Environment dengan enum case sesuai yang kita mau
*/

#[test]
fn test_config_env() {
    unsafe {
        set_var("DB_HOST", "localhost");
        set_var("DB_PORT", "5432");
        set_var("DB_USER", "user");
        set_var("DB_PASS", "password");
    }

    let config = Config::builder()
        .add_source(Environment::default().convert_case(Case::Snake))
        .build()
        .unwrap();

    assert_eq!(config.get_string("db_host").unwrap(), "localhost");
    assert_eq!(config.get_string("db_port").unwrap(), "5432");
    assert_eq!(config.get_string("db_user").unwrap(), "user");
    assert_eq!(config.get_string("db_pass").unwrap(), "password");
}

/*
JSON
- Config juga bisa digunakan untuk mengambil konfigurasi dari file JSON
- Kita bisa menggunakan struct File untuk menentukan lokasi file JSON nya
- Dan menggunakan enum `FileFormat::Json`
*/

#[test]
fn test_config_json() {
    let config = Config::builder()
        .add_source(File::new("application.json", FileFormat::Json))
        .build()
        .unwrap();

    println!("{:?}", config);
    println!("{:?}", config.get_string("name"));
    println!("{:?}", config.get_string("database.host"));
    println!("{:?}", config.get_string("database.port"));
    println!("{:?}", config.get_string("database.user"));

    assert_eq!(config.get_string("name").unwrap(), "My Application");
    assert_eq!(config.get_string("database.host").unwrap(), "localhost");
    assert_eq!(config.get_string("database.port").unwrap(), "5432");
    assert_eq!(config.get_string("database.user").unwrap(), "user");
    assert_eq!(config.get_string("database.password").unwrap(), "password");
}

/*
YAML
- Config juga bisa digunakan untuk mengambil konfigurasi dari file YAML
- Kita bisa menggunakan struct File untuk menentukan lokasi file YAML nya
- Dan menggunakan enum `FileFormat::Yaml`
*/

#[test]
fn test_config_yaml() {
    let config = Config::builder()
        .add_source(File::new("application.yaml", FileFormat::Yaml))
        .build()
        .unwrap();

    println!("{:?}", config);
    println!("{:?}", config.get_string("name"));
    println!("{:?}", config.get_string("database.host"));
    println!("{:?}", config.get_string("database.port"));
    println!("{:?}", config.get_string("database.user"));
    println!("{:?}", config.get_string("database.password"));

    assert_eq!(config.get_string("name").unwrap(), "My Application");
    assert_eq!(config.get_string("database.host").unwrap(), "localhost");
    assert_eq!(config.get_string("database.port").unwrap(), "5432");
    assert_eq!(config.get_string("database.user").unwrap(), "user");
    assert_eq!(config.get_string("database.password").unwrap(), "password");
}

/*
TOML
- Config juga bisa digunakan untuk mengambil konfigurasi dari file TOML
- Kita bisa menggunakan struct File untuk menentukan lokasi file TOML nya
- Dan menggunakan enum `FileFormat::Toml`
*/

#[test]
fn test_config_toml() {
    let config = Config::builder()
        .add_source(File::new("application.toml", FileFormat::Toml))
        .build()
        .unwrap();

    println!("{:?}", config);
    println!("{:?}", config.get_string("name"));
    println!("{:?}", config.get_string("database.host"));
    println!("{:?}", config.get_string("database.port"));
    println!("{:?}", config.get_string("database.user"));
    println!("{:?}", config.get_string("database.password"));

    assert_eq!(config.get_string("name").unwrap(), "My Application");
    assert_eq!(config.get_string("database.host").unwrap(), "localhost");
    assert_eq!(config.get_string("database.port").unwrap(), "5432");
    assert_eq!(config.get_string("database.user").unwrap(), "user");
    assert_eq!(config.get_string("database.password").unwrap(), "password");
}

/*
DESERIALIZATION
- Config bisa diintegrasikan dengan library Serde untuk mengkonversi data dari format yang kita gunakan menjadi object struct
- Kita bisa gunakan method `try_deserialize()` di Config
*/

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    name: String,
    database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    host: String,
    port: u16,
    name: String,
    username: String,
    password: String,
}

#[test]
fn test_config_deserialize() {
    let config = Config::builder()
        .add_source(File::new("application.json", FileFormat::Json))
        .build()
        .unwrap();

    let app_config: AppConfig = config.try_deserialize().unwrap();
    println!("Config = {:?}", app_config);
    println!("Name = {}", app_config.name);
    println!("Database Host = {}", app_config.database.host);
    println!("Database Port = {}", app_config.database.port);
    println!("Database Name = {}", app_config.database.name);
    println!("Database Username = {}", app_config.database.username);
    println!("Database Password = {}", app_config.database.password);

    assert_eq!(app_config.name, "My Application");
    assert_eq!(app_config.database.host, "localhost");
    assert_eq!(app_config.database.port, 5432);
    assert_eq!(app_config.database.name, "my_application");
    assert_eq!(app_config.database.username, "user");
    assert_eq!(app_config.database.password, "password");
}
