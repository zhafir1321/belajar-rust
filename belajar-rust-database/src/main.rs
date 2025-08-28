fn main() {
    println!("Hello, world!");
}

/*
RUST DATABASE
- Rust sendiri tidak memiliki library standard untuk terkoneksi ke database
- Rust memerlukan library tambahan untuk terkoneksi ke database

SQLX
- SQLx merupakan library untuk koneksi ke database di Rust yang mendukung database PostgreSQL, MySQL, MariaDB, dan SQLite

MENAMBAH SQLX
- `cargo add chrono`
- `cargo add tokio --features full`
- `cargo add sqlx --features runtime-tokio,chrono,mysql`

CONNECTION
- Untuk membuat koneksi ke MySQL, kita bisa menggunakan struct MySqlConnection

*/

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use chrono::{DateTime, Utc};
    use futures::TryStreamExt;
    use sqlx::{
        Connection, Error, MySql, MySqlConnection, Pool, Row, Transaction,
        mysql::{MySqlPoolOptions, MySqlRow},
        prelude::FromRow,
    };

    #[tokio::test]
    async fn test_manual_connection() -> Result<(), Error> {
        let url = "mysql://root:@Zhafir99@localhost:3306/belajar-rust-database";
        let connection: MySqlConnection = MySqlConnection::connect(url).await?;

        connection.close().await?;
        Ok(())
    }

    /*
    DATABASE POOL
    - Membuat koneksi ke database adalah salah satu hal yang mahal dalam aplikasi,
    oleh karena itu tidak dianjurkan selalu membuka tutup koneksi di aplikasi kita
    - Salah satu praktik yang baik saat membuat koneksi ke database adalah membuat Database Pool (kumpulan koneksi database)
    - Database Pool adalah konsep dimana kita membuat tempat untuk menampung koneksi ke database,
    sehingga ketika membutuhkan koneksi, kita bisa ambil dari Database Pool, dan ketika sudah tidak membutuhkan lagi,
    kita kembalikan ke Database Pool
    - Database Pool juga bisa menjaga agar kita tidak terlalu banyak membuat koneksi ke database sehingga tidak membebani database terlalu berat

    DATABASE POOL DI SQLX
    - Untuk membuat Database Pool di SQLx, terutama untuk MySQL, kita bisa menggunakan struct MySqlPool
    - Kita bisa melakukan banyak pengaturan ketika membuat Database Pool, seperti jumlah minimal dan maksimal koneksi menggunakan MySqlPoolOptions
     */

    async fn get_pool() -> Result<Pool<MySql>, Error> {
        let url = "mysql://root:@Zhafir99@localhost:3306/belajar-rust-database";
        MySqlPoolOptions::new()
            .max_connections(10)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(60))
            .connect(url)
            .await
    }

    #[tokio::test]
    async fn test_pool_connection() -> Result<(), Error> {
        let pool = get_pool().await?;
        pool.close().await;
        Ok(())
    }

    /*
    QUERY
    - Untuk mengirim perintah SQL ke database, kita bisa menggunakan function `query()` pada module sqlx
    - Hasil dari function `query()` tersebut adalah object query

    EXECUTE SQL
    - Jika kita ingin mengeksekusi perintah SQL yang tidak menghasilkan data, kita bisa menggunakan method `execute()` pada object query
     */

    #[tokio::test]
    async fn test_execute() -> Result<(), Error> {
        let pool = get_pool().await?;
        sqlx::query("insert into category(id, name, description) values('A', 'contoh', 'contoh');")
            .execute(&pool)
            .await?;
        Ok(())
    }

    /*
    PREPARE STATEMENT
    - Salah satu masalah ketika kita membuat SQL menggunakan string concat adalah SQL Injection
    - Oleh karena itu, tidak disarankan menggunakan string concat untuk membuat perintah SQL yang akan kita kirim menggunakan SQLx
    - SQLx Query mendukung Prepare Statement, sehingga jika kita ingin mengirim parameter ke perintah SQL, kita bisa menggunakan `bind()`
     */

    #[tokio::test]
    async fn test_prepare_statement() -> Result<(), Error> {
        let pool = get_pool().await?;
        sqlx::query("insert into category(id, name, description) values(?, ?, ?)")
            .bind("B")
            .bind("contoh lagi")
            .bind("inideskripsi")
            .execute(&pool)
            .await?;
        Ok(())
    }

    /*
    QUERY SQL
    - Untuk membuat perintah SQL yang menghasilkan data, kita bisa menggunakan function yang sama, yaitu `query()`
    - Namun hasil object Query nya kita tidak menggunakan method `execute()`, melainkan menggunakan method `fetch()`

    FETCH METHOD
    - `fetch_optional(): Result<Option>` => Jika menghasilkan satu data atau kosong
    - `fetch_one(): Result` => Jika menghasilkan satu data, jika tidak ada, maka akan error
    - `fetch_all(): Vec<Result>` => Untuk data banyak
    - `fetch(): Stream<Result>` => Untuk mengambil data dalam bentuk stream (lazy)
     */

    #[tokio::test]
    async fn test_fetch_optional() -> Result<(), Error> {
        let pool: Pool<MySql> = get_pool().await?;
        let result: Option<MySqlRow> = sqlx::query("select * from category where id = ?")
            .bind("B")
            .fetch_optional(&pool)
            .await?;

        if let Some(row) = result {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");
            println!("ID: {}, Name: {}, Description: {}", id, name, description);
        } else {
            println!("Data not found");
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_one() -> Result<(), Error> {
        let pool: Pool<MySql> = get_pool().await?;
        let result: MySqlRow = sqlx::query("select * from category where id = ?")
            .bind("C")
            .fetch_one(&pool)
            .await?;

        let id: String = result.get("id");
        let name: String = result.get("name");
        let description: String = result.get("description");
        println!("ID: {}, Name: {}, Description: {}", id, name, description);

        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_all() -> Result<(), Error> {
        let pool: Pool<MySql> = get_pool().await?;
        let results: Vec<MySqlRow> = sqlx::query("select * from category")
            .fetch_all(&pool)
            .await?;

        for row in results {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");
            println!("ID: {}, Name: {}, Description: {}", id, name, description);
        }

        Ok(())
    }

    /*
    FETCH
    - Khusus untuk method `fetch()`, hasil return dari method nya berupa Stream (versi async dari Iterator)
    - Oleh karena itu, kita perlu menambah library Futures untuk mengambil data di Stream tersebut
    - `cargo add futures`
     */

    #[tokio::test]
    async fn test_fetch() -> Result<(), Error> {
        let pool: Pool<MySql> = get_pool().await?;
        let mut results = sqlx::query("select * from category").fetch(&pool);

        while let Some(result) = results.try_next().await? {
            let id: String = result.get("id");
            let name: String = result.get("name");
            let description: String = result.get("description");
            println!("ID: {}, Name: {}, Description: {}", id, name, description);
        }

        Ok(())
    }

    /*
    RESULT MAPPING
    - Salah satu yang biasa kita lakukan ketika membuat Query SQL adalah, mengubah hasil tiap baris menjadi object struct
    - Untuk mengubah data hasil Query dalam bentuk Row menjadi Struct, kita bisa memanfaatkan method `map()` pada Query
     */

    #[derive(FromRow, Debug)]
    struct Category {
        id: String,
        name: String,
        description: String,
    }

    #[tokio::test]
    async fn test_result_mapping() -> Result<(), Error> {
        let pool: Pool<MySql> = get_pool().await?;
        let results: Vec<Category> = sqlx::query("select * from category")
            .map(|row: MySqlRow| Category {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
            })
            .fetch_all(&pool)
            .await?;

        for category in results {
            println!("Category: {:?}", category);
        }

        Ok(())
    }

    /*
    AUTOMATIC RESULT MAPPING
    - SQLx memiliki fitur untuk melakukan mapping secara otomatis, kita bisa menggunakan attribut FromRow darl SQLx
    - Caranya kita harus menambahkan attribute FromRow pada Struct yang kita buat, dan pastikan hasil query kolom sama dengan nama field di Struct
    - Dan untuk membuat Query SQL, kita perlu mengganti dari function `query()` menjadi `query_as()`
     */

    #[tokio::test]
    async fn test_automatic_result_mapping() -> Result<(), Error> {
        let pool: Pool<MySql> = get_pool().await?;
        let results: Vec<Category> = sqlx::query_as("select * from category")
            .fetch_all(&pool)
            .await?;

        for category in results {
            println!("Category: {:?}", category);
        }

        Ok(())
    }

    /*
    DATA TYPE
    - Saat membuat perintah SQL kadang kita perlu mengirim data, atau bisa menghasilkan data
    - Tiap database, memiliki jenis data masing - masing, dan bisa berbeda dengan jenis data di Rust
     */

    #[derive(FromRow, Debug)]
    struct Brand {
        id: String,
        name: String,
        description: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    }

    #[tokio::test]
    async fn test_result_mapping_brand() -> Result<(), Error> {
        let pool: Pool<MySql> = get_pool().await?;
        let results: Vec<Brand> = sqlx::query_as("select * from brands")
            .fetch_all(&pool)
            .await?;
        for brand in results {
            println!("Brand: {:?}", brand);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_insert_brand() -> Result<(), Error> {
        let pool: Pool<MySql> = get_pool().await?;
        sqlx::query("insert into brands (id, name, description, created_at, updated_at) values (?, ?, ?, ?, ?)")
            .bind("A")
            .bind("Brand Name")
            .bind("Brand Description")
            .bind(Utc::now().naive_local())
            .bind(Utc::now().naive_local())
            .execute(&pool)
            .await?;

        Ok(())
    }

    /*
    TRANSACTION
    - Salah satu fitur yang sangat penting di Database adalah Transaction
    - SQLx juga memiliki fitur yang bisa digunakan untuk membuat database transaction
    - Kita bisa menggunakan method `begin()` yang menghasilkan object transaction
     */
    #[tokio::test]
    async fn test_transaction() -> Result<(), Error> {
        let pool: Pool<MySql> = get_pool().await?;

        let mut transaction: Transaction<MySql> = pool.begin().await?;

        sqlx::query("insert into brands(id, name, description, created_at, updated_at) values (?, ?, ?, ?, ?)")
        .bind("D")
        .bind("Ini contoh D")
        .bind("Ini contoh D")
        .bind(Utc::now().naive_local())
        .bind(Utc::now().naive_local())
        .execute(&mut *transaction)
        .await?;

        sqlx::query("insert into brands(id, name, description, created_at, updated_at) values (?, ?, ?, ?, ?)")
        .bind("E")
        .bind("Ini contoh E")
        .bind("Ini contoh E")
        .bind(Utc::now().naive_local())
        .bind(Utc::now().naive_local())
        .execute(&mut *transaction)
        .await?;

        sqlx::query("insert into brands(id, name, description, created_at, updated_at) values (?, ?, ?, ?, ?)")
        .bind("A")
        .bind("Brand Name")
        .bind("Brand Description")
        .bind(Utc::now().naive_local())
        .bind(Utc::now().naive_local())
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    /*
    AUTO INCREMENT
    - Beberapa database, kadang memiliki fitur Auto Increment, misal pada MySQL PostgreSQL
    - SQLx sendiri tidak punya fitur untuk mendapatkan nilai terakhir Auto Increment,
    oleh karena itu kita harus lakukan query secara manual
    - Untungnya tiap database biasanya punya cara untuk mendapatkan nilai Auto Increment terakhir
    - Namun perlu diingat, untuk dapat nilai Auto Increment, kita harus menggunakan koneksi yang sama pada SQLx,
    jika menggunakan Database Pool, kita bisa saja mendapatkan koneksi yang berbeda
    - Oleh karena itu, kita bisa memanfaatkan juga Transaction, karena akan menggunakan koneksi yang sama
     */

    #[tokio::test]
    async fn test_auto_increment() -> Result<(), Error> {
        let pool: Pool<MySql> = get_pool().await?;
        let mut transaction: Transaction<MySql> = pool.begin().await?;

        sqlx::query("insert into sellers(name) values(?)")
            .bind("Seller A")
            .execute(&mut *transaction)
            .await?;

        let result: MySqlRow = sqlx::query("select last_insert_id() as id")
            .fetch_one(&mut *transaction)
            .await?;

        let id: i32 = result.get_unchecked("id");
        println!("Last Inserted ID: {}", id);

        transaction.commit().await?;
        Ok(())
    }

    /*
    DATABASE MIGRATION
    - SQLx memiliki fitur bernama database migration, yang bisa kita gunakan untuk melakukan management versi skema perubahan database
    - Fitur ini sangat berguna, terutama ketika membuat aplikasi dengan ukuran besar, sehingga tidak perlu lagi melakukan manajemen perubahan skema database secara manual
    - Fitur database migration ini dibuat dalam library yang berbeda, dan berbasis CLI (Command Line Interface), yaitu SQLx-CLI

    SQLX-CLI
    - SQLx-CLI tidak perlu ditambahkan dalam aplikasi kita, kita cukup menginstallnya sehingga menjadi program yang berjalan via terminal menggunakan perintah
    - `cargo install sqlx-cli`
    - Selanjutnya kita bisa menggunakan perintah sqlx via terminal

    SETUP DATABASE MIGRATION
    - SQLx menggunakan environment variable untuk mendeteksi lokasi databasenya
    - kita bisa menggunakan nama environment variable DATABASE_URL yang berisi url menuju ke database
    - Atau, kita bisa menggunakan .env

    MEMBUAT DATABASE
    - Jika database nya belum ada, kita bisa meminta SQLx untuk membuatkan database, kita bisa gunakan perintah
    - `sqlx database create`

    MEMBUAT MIGRATION
    - Untuk membuat file migration, kita bisa menggunakan perintah
    - `sqlx migrate add -r <nama_migration>`
    - Secara otomatis file migration akan dibuat di folder migrations

    MENJALANKAN MIGRATION
    - Untuk menjalankan migration, kita bisa gunakan perintah
    - `sqlx migrate run --target-version <version>`
    - Atau jika ingin menjalankan seluruh file migration yang belum dijalankan
    - `sqlx migrate run`

    MEMBATALKAN MIGRATION
    - Untuk membatalkan migration, kita bisa menggunakan perintah
    - `sqlx migrate revert --target-version <version>`
    - Atau jika ingin membatalkan satu migration
    - `sqlx migrate revert`
     */
}
