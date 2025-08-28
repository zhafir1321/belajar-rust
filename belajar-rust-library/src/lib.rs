/*

CARGO

Package mananger buat Rust.
Cargo dapat men-download package dependency secara otomatis, melakukan kompilasi package, dan membuat distribution file.
Cargo juga bisa digunakan untuk meng-upload package ke crates.io.

*/

pub fn say_hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

/*

JENIS PROJECT

Saat kita membuat project baru menggunakan Cargo, kita bisa memilih jenis project yang akan kita buat.
1. Application
        Jenis project untuk aplikasi, misal aplikasi berbasis terminal, aplikasi web, dan lain-lain.

2. Library
        Jenis project untuk library (kode yang digunakan oleh library lain atau aplikasi).

*/

/*

MEMBUAT PROJECT

Untuk membuat project dengan Cargo, kita bisa menggunakan printah:
    cargo new <nama-project>

Secara default, jenis project yang akan dibuat adalah Application.

Jika kita ingin membuat project dengan jenis Library, kita bisa menambahkan flag `--lib`:
    cargo new <nama-project> --lib
*/

/*

MEMBUAT LIBRARY

Buatlah library dengan nama 'say_hello' menggunakan perintah:
    cargo new say_hello --lib

*/

/*

STRUKTUR FOLDER

- src 
    Folder ini berisi kode sumber dari project kita.
    File `lib.rs` adalah file utama untuk library, sedangkan `main.rs` adalah file utama untuk aplikasi.

- target
    Folder ini berisi hasil kompilasi dari project kita.
    Folder ini akan dibuat secara otomatis oleh Cargo saat kita melakukan build.

- Cargo.toml
    File ini berisi konfigurasi dari project kita, seperti nama project, versi, dan dependency yang digunakan.

- Cargo.lock
    File ini berisi informasi tentang versi dari dependency yang digunakan pada project kita.
    File ini akan dibuat secara otomatis oleh Cargo saat kita melakukan build.

*/

pub fn say_goodbye(name: &str) -> String {
    format!("Goodbye, {}!", name)
}
