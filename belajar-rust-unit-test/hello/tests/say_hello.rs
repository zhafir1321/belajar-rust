use hello::say_hello;

/*
INTEGRATION TEST
Integration test adalah testing secara external, terpisah dari kode program kita dan hanya digunakan untuk testing kode public.
Tujuan integration test adalah melakukan pengujian seluruh integrasi dari aplikasi yang kita buat.

TESTS FOLDER
Saat membuat integration test, di Rust biasanya akan disimpan dalam folder `tests`, terpisah dari folder `src`.
Karena foldernya terpisah, otomatis modulenya juga terpisah.
Hal ini menyebabkan integration test hanya bisa menggunakan kode public pada kode program yang kita buat.

INTEGRATION TEST DI APP PROJECT
Integration test hanya bisa digunakan untuk jenis project Library, jika kita membuat jenis project Application (yang berisi `src/main.rs`),
maka kita tidak bisa membuat integration test.
Cara lain jika kita ingin membuat integration test di jenis project Application, kita bisa buat dalam bentuk workspace.
Misal kita sekarang akan buat sub package dengan nama `hello`:
cargo new hello --lib

MENJALANKAN INTEGRATION TEST
Sama aja sepertin menjalankan unit test, cargo test.
Namun, jika kita buat dalam sub package, untuk menjalankan test pada package yang ingin kita pilih,
kita bisa gunakan perintah:
cargo test -p <nama-package>
 */

#[test]
fn test_say_hello() {
    let result = say_hello("Zhafir");
    assert_eq!(result, "Hello, Zhafir!", "Expected 'Hello, Zhafir!'");
}
