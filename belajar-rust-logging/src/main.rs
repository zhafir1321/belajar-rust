fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    /*
    RUST LOGGING
    - Rust memiliki fitur untuk logging yang dibuat dalam Crate Log
    - Namun Crate Log tersebut hanyalah kontrak untuk melakukan logging,
    untuk implementasinya sendiri kita perlu memilih Crate lainnya
    - Salah satu keuntungan menggunakan kontrak adalah,
    kita bisa berganti - ganti implementasi, tanpa harus mengubah kode loggingnya

    LEVEL
    - Saat melakukan log, kita perlu menentukan level informasi yang akan kita log
    - Rust Log mendukung beberapa level, dan level itu bertingkat jadi kita akan memilih log yang akan kita buat di tingkat level mana

    MELAKUKAN LOG
    - Untuk melakukan log, kita bisa gunakan macro log!(level, log)
    - Atau kita bisa gunakan shortcut macro
    - error!() untuk level error
    - warn!() untuk level warn
    - info!() untuk level info
    - debug!() untuk level debug
    - trace!() untuk level trace

    SIMPLE LOGGER
    - Karena Crate Log hanya kontrak, maka kita harus pilih implementasinya
    - Salah satu implementasi yang sederhana adalah Env Logger
    - Env Logger bisa digunakan untuk menampilkan log ke Console/ Terminal,
    level yang akan diaktifkan bisa di set via Env Variable sistem operasi

    MENJALANKAN ENV LOGGER
    - Ubah Env Variable di sistem operasi dengan nama RUST_LOG=<level>
     */

    use log::{debug, error, info, trace, warn};

    #[test]
    fn test_logging() {
        env_logger::init();
        error!("This is an error message");
        warn!("This is a warning message");
        info!("This is an info message");
        debug!("This is a debug message");
        trace!("This is a trace message");
    }

    /*
    COMPLEX LOGGER
    - Env Logger hanya bisa digunakan untuk menampilkan log ke Console,
    bagaimana jika kita ingin menampilkan log ke tempat lain? Misal ke file
    - Atau mengatur level tergantung module nya
    - Kita bisa gunakan implementasi Logger yang lebih kompleks, contohnya adalah log4rs

    CONFIGURATION
    - Untuk menggunakan Log4rs, kita bisa menyimpan semua konfigurasinya menggunakan file konfigurasi yaml
    - Selanjutnya kita bisa baca file konfigurasi yaml tersebut menggunakan library Log4rs

    APPENDER
    - Pada kode konfigurasi sebelumnya (log4rs.yaml), kita membuat appender dengan nama stdout dengan jenis console
    - Appender adalah lokasi log akan disimpan, contoh kita buat dengan nama stdout dengan tujuan console
    - Log4rs mendukung beberapa pilihan appender

    LOGGER
    - Salah satu kelebihan Log4rs adalah, kita bisa mudah mengubah level untuk module - module tanpa harus mengubah kode program
    - Kita hanya perlu mengubah file konfigurasinya
     */

    #[test]
    fn test_log4rs() {
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
        error!("This is an error message");
        warn!("This is a warning message");
        info!("This is an info message");
        debug!("This is a debug message");
        trace!("This is a trace message");
    }
}

#[cfg(test)]
mod tests2 {
    use log::{debug, error, info, trace, warn};
    #[test]
    fn test_logging() {
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
        error!("This is an error message");
        warn!("This is a warning message");
        info!("This is an info message");
        debug!("This is a debug message");
        trace!("This is a trace message");
    }
}
