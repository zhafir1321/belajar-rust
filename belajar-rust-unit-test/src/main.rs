fn main() {
    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn start_application(host: &str, port: u16) {
    if host == "localhost" {
        panic!("Cannot start application on localhost");
    } else {
        println!("Starting application on {}:{}", host, port);
    }
}

/*
TEST MODULE
Salah satu praktek unit test yang biasa dilakukan di Rust adalah membuat test module.
Setiap module, saat kita akan membuat unit test, alangkah baiknya kita buat dalam test module.
Biasanya kita akan membuat sub module dengan nama `tests`, lalu menambahkan attribut `cfg(test)` untuk menandai bahwa module tersebut adalah module test.

KEUNTUNGAN TEST MODULE
Ketika kita melakukan kompilasi untuk mode production, maka test module tidak akan dicompile,
sehingga proses compile akan lebih cepat.
Selain itu, ketika kita membuat unit test yang membutuhkan development dependency,
maka dependency tersebut tidak akan di-include di dalam hasil kompilasinya, sehingga ukuran hasil kompilasi akan tetap kecil.
*/
#[cfg(test)]
mod tests {
    use crate::{add, start_application};

    /*
    PANIC
    Saat melakukan unit test, kadang kita ingin melakukan pengetesan terhadap kode program yang bisa menyebabkan panic.
    Jika kita ingin membuat unit test yang mengharuskan kode yang ditest terjadi panic, kita bisa gunakan attribut should_panic.
    */
    #[test]
    #[should_panic] // This test is expected to panic
    fn test_start_application() {
        start_application("localhost", 8080);
    }

    /*
    ASSERTION
    Saat membuat unit test, hal yang biasa dilakukan adalah memastikan kode yang ditest itu benar.
    Cara memastikan bahwa kode yang ditest itu benar, biasanya kita akan menggunakan assertions.
    Rust menyediakan beberapa macro untuk melakukan assertion.

    ASSERTION MACRO
    - `assert!` (boolean, message): Memastikan bahwa nilai boolean adalah true, jika false maka akan terjadi error dengan message.
    - `assert_eq!` (left, right, message): Memastikan bahwa nilai left sama dengan nilai right, jika berbeda, maka akan terjadi error dengan message.
    - `assert_ne!` (left, right, message): Memastikan bahwa nilai left tidak sama dengan nilai right, jika sama, maka akan terjadi error dengan message.

    TEST RESULT
    Saat membuat unit test, kita juga bisa membuat test function yang mengembalikan `Result<(), E>`.
    Jika hasilnya adalah `Ok()`, maka unit test akan sukses, jika hasilnya adalah `Err(E)`, maka hasilnya akan error.
    */
    #[test]
    fn test_add_again() -> Result<(), String> {
        let result = add(1, 2);
        if result == 3 {
            Ok(())
        } else {
            Err("Expected 1 + 2 to equal 3".to_string())
        }
    }

    #[test]
    fn test_add() {
        let result = add(2, 3);
        assert_eq!(result, 5, "Expected 2 + 3 to equal 5");
    }

    /*
    IGNORE
    Saat membuat unit test, kadang kita sering mencoba semua kemungkinan, dan kadang mungkin terjadi kesalahan dalam unit test yang kita buat.
    Hal ini kadang menyebabkan kita tidak ingin melanjalankan unit test tersebut sementara.
    Ada beberapa cara jika kita ingin menjalankan unit testnya, menghapus unit testnya, atau menambahkan komentar.
    Namun hal itu tidak direkomendasikan, cara yang lebih baik adalah menggunakan atribut `ignore`.
    Unit test yang memiliki attribut `ignore`, maka secara otomatis tidak akan dijalankan.

    MENJALANKAN TEST IGNORE
    Jika kita menjalankan unit test dengan menggunakan perintah: `cargo test`.
    Secara otomatis semua unit test yang menggunakan attribut `ignore` tidak akan dijalankan.
    Namun jika kita tetap ingin menjalankan semua unit test yang di-ignore, maka kita harus menambahkan parameter `--ignored` pada perintah `cargo test`.
    Contoh: `cargo test --ignored`
     */
    #[test]
    #[ignore] // This test is ignored and will not run unless explicitly specified with `--ignored`
    fn test_add_negative() {
        let result = add(-2, -3);
        assert_eq!(result, -5, "Expected -2 + -3 to equal -5");
    }

    #[test] // This is a simple test that will run when `cargo test` is executed
    fn test_simple() {
        println!("Running simple test");
    }
}
