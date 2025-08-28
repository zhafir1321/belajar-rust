use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error, Visitor},
};
use serde_json::{from_str, to_string};

fn main() {
    println!("Hello, world!");
}

/*
SERIALIZATION DAN DESERIALIZATION
- Salah satu hal yang sering dilakukan saat membuat aplikasi adalah melakukan konversi format data
- Saat kita menggunakan Rust, data biasanya akan dibentuk dalam Struct, sedangkan ketika ketika berkomunikasi dengan aplikasi lain
(yang mungkin berbeda teknologi), biasanya kita akan menggunakan format data yang disepakati, misal JSON, XML, CSV, dan lain - lain
- Serialization merupakan proses mengubah tipe data yang ada di Rust (misal struct) menjadi format data lain (JSON, XML, dll)
- Sedangkan Deserialization merupakan proses kebalikannya, mengubah tipe data lain menjadi tipe data yang ada di Rust

SERDE
- Rust sendiri tidak memiliki standard library untuk melakukan proses Serialization dan Deserialization
- Namun, kita bisa memanfaatkan library Serde untuk melakukan ini, yaitu library yang paling populer untuk melakukan Serialization dan Deserialization di Rust

IMPLEMENTASI SERDE
- Library Serde sendiri, berisi kontrak untuk melakukan Serialization dan Deserialization
- Namun untuk implementasinya, kita harus gunakan library sesuai dengan format data yang kita gunakan

STRUCT
- Salah satu proses Serialization dan Deserialization yang sering dilakukan adalah untuk tipe data Struct
- Misal kita ingin mengubah dari Struct ke JSON atau kebalikannya
- Untuk melakukan itu, kita bisa tambahkan derive Serialize dan Deserialize pada struct
- Serde mendukung banyak tipe data pada attribute struct
*/

#[derive(Debug, Serialize, Deserialize)]
struct UserLoginRequest {
    username: String,
    password: String,
}

#[test]
fn test_serialization() {
    let login_request = UserLoginRequest {
        username: "zhafir1321".to_string(),
        password: "password123".to_string(),
    };

    let json = to_string(&login_request).unwrap();
    println!("JSON: {}", json);

    let login_result: UserLoginRequest = from_str(&json).unwrap();
    println!("Login Result: {:?}", login_result);
}

/*
NESTED STRUCT
- Salah satu yang biasa kita lakukan saat membuat Struct adalah menambah atribut dengan tipe Struct lainnya, atau bisa dibilang Nested Struct
- Serialization dan Deserialization juga bisa digunakan untuk Nested Struct
*/

#[derive(Debug, Serialize, Deserialize)]
struct AddressRequest {
    street: String,
    city: String,
    state: String,
    zip: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserRequest {
    username: String,
    email: String,
    password: String,
    address: AddressRequest,
}

#[test]
fn test_nested_serialization() {
    let create_user = CreateUserRequest {
        username: "zhafir1321".to_string(),
        email: "zhafir1321@example.com".to_string(),
        password: "password123".to_string(),
        address: AddressRequest {
            street: "Jalan Raya".to_string(),
            city: "Kota".to_string(),
            state: "Provinsi".to_string(),
            zip: "12345".to_string(),
        },
    };

    let json = to_string(&create_user).unwrap();
    println!("JSON: {}", json);

    let create_user_result: CreateUserRequest = from_str(&json).unwrap();
    println!("Create User Result: {:?}", create_user_result);
}

/*
ARRAY
- Saat kita menggunakan format data seperti JSON misalnya, kita akan sering menggunakan data Array
- Di Javascript, ukuran Array bersifat dinamis, bisa bertambah dan berkurang, berbeda dengan Rust
- Oleh karena itu, penggunaan Array di Rust tidak sesering digunakan seperti di Javascript
- Proses Serialization bisa dilakukan untuk tipe Array, namun tidak bisa untuk Deserialization karena ukuran Array di Rust harus ditentukan,
sedangkan mungkin kita tidak bisa menentukan jumlah Array dari format data lain
*/

#[test]
fn test_array_serialization() {
    let numbers = [1, 2, 3, 4, 5];
    let json = to_string(&numbers).unwrap();
    println!("JSON: {}", json);

    // let numbers_result: [i32] = from_str(&json).unwrap();
    // println!("Numbers Result: {:?}", numbers_result);
}

/*
VECTOR
- Karena keterbatasan tipe data Array di Rust, oleh karena itu lebih banyak digunakan tipe data Vector untuk representasi Array di format data lain
- Karena Vector ukurannya dinamis, jadi kita bisa dengan mudah melakukan Serialization dan Deserialization
*/

#[derive(Debug, Serialize, Deserialize)]
struct Profile {
    name: String,
    age: u32,
    hobbies: Vec<String>,
}

#[test]
fn test_vector_serialization() {
    let profile = Profile {
        name: "John Doe".to_string(),
        age: 30,
        hobbies: vec!["Reading".to_string(), "Traveling".to_string()],
    };

    let json = to_string(&profile).unwrap();
    println!("JSON: {}", json);

    let profile_result: Profile = from_str(&json).unwrap();
    println!("Profile Result: {:?}", profile_result);
}

/*
OPTION
- Di Rust, terdapat Option yang bisa digunakan sebagai pembungkus data yang sifatnya tidak wajib atau optional
- Rust Serialization juga mendukung tipe data Option
- Jadi kita bisa dengan mudah memberitahu atribut mana yang optional dan atribut mana yang wajib
*/

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    name: String,
    description: Option<String>,
}

#[test]
fn test_product_serialization() {
    let product = Product {
        name: "Laptop".to_string(),
        description: Some("A high-end laptop".to_string()),
    };

    let product2 = Product {
        name: "Mouse".to_string(),
        description: None,
    };

    let json = to_string(&product).unwrap();
    let json2 = to_string(&product2).unwrap();
    println!("JSON: {}", json);
    println!("JSON2: {}", json2);

    let product_result: Product = from_str(&json).unwrap();
    let product_result2: Product = from_str(&json2).unwrap();
    println!("Product Result: {:?}", product_result);
    println!("Product Result2: {:?}", product_result2);
}

/*
MAP
- Saat kita menggunakan Struct, kita wajib tahu semua nama atribut ketika melakukan proses serialization dan deserialization
- Namun, kadang ada kasus dimana kita tidak tahu nama - nama atribut, atau bahkan bisa berubah - ubah
- Pada kasus seperti ini, biasanya kita akan menggunakan tipe data Map yang bisa menggunakan key yang dinamis
*/

#[test]
fn test_map_serialization() {
    let mut user: HashMap<String, String> = HashMap::new();
    user.insert("username".to_string(), "zhafir1321".to_string());
    user.insert("email".to_string(), "zhafir1321@example.com".to_string());

    let json = to_string(&user).unwrap();
    println!("JSON: {}", json);

    let user_result: HashMap<String, String> = from_str(&json).unwrap();
    println!("User Result: {:?}", user_result);
}

/*
ATTRIBUTE
- Secara default, saat kita melakukan serialization dan deserialization dari Struct, maka Serde akan menggunakan nama atribut sesuai dengan nama atribut di Struct
- Kadang ada kalanya atribut datanya berbeda antara misal di JSON dan di Struct
- Kita tidak perlu khawatir karena itu bisa dikonfigurasi

CONTAINER ATTRIBUTE
- Konfigurasi pertama yang bisa dilakukan adalah di level Struct nya
- Konfigurasi di level Struct, akan berdampak ke semua attribute di Struct tersebut
*/

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(
    serialize = "SCREAMING_SNAKE_CASE",
    deserialize = "SCREAMING_SNAKE_CASE"
))]
struct Friend {
    first_name: String,
    last_name: String,
}

#[test]
fn test_friend_serialization() {
    let friend = Friend {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
    };

    let json = to_string(&friend).unwrap();
    println!("JSON: {}", json);

    let friend_result: Friend = from_str(&json).unwrap();
    println!("Friend Result: {:?}", friend_result);
}

/*
FIELD ATTRIBUTE
- Konfigurasi kedua bisa lakukan ke attribute di Struct langsung, jika memang yang kita inginkan hanya mengubah konfigurasi untuk salah satu attribute saja
*/

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
    synopsis: String,
    #[serde(rename = "published_year")]
    year: u32,
}

#[test]
fn test_field_attribute() {
    let book = Book {
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klabnik".to_string(),
        synopsis: "An introduction to Rust".to_string(),
        year: 2018,
    };

    let json = to_string(&book).unwrap();
    println!("JSON: {}", json);

    let book_result: Book = from_str(&json).unwrap();
    println!("Book Result: {:?}", book_result);
}

/*
ENUM
- Serde juga bisa digunakan untuk melakukan serialization dan deserialization dari tipe data Enum
- Namun tipe data Enum juga wajib ditambahkan derive Serialize dan Deserialize

ENUM DATA
- Enum di Rust bisa memiliki data
- Serde juga mendukung jenis Enum Data
- Dan kita bisa ubah bagaimana cara menampilkan format Enum Data menggunakan attribute serde
*/

#[derive(Debug, Serialize, Deserialize)]
struct Customer {
    name: String,
    email: String,
    phone_number: Option<String>,
    gender: Gender,
    hobbies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
enum Gender {
    Male,
    Female,
}

#[test]
fn test_customer_serialization() {
    let customer = Customer {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        phone_number: None,
        gender: Gender::Female,
        hobbies: vec!["Reading".to_string(), "Traveling".to_string()],
    };

    let json = to_string(&customer).unwrap();
    println!("JSON: {}", json);

    let customer_result: Customer = from_str(&json).unwrap();
    println!("Customer Result: {:?}", customer_result);
}

#[derive(Debug, Serialize, Deserialize)]
struct Subscriber {
    name: String,
    email: String,
    phone_number: Option<String>,
    gender: Gender,
    payment: Payment,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum Payment {
    CreditCard {
        card_number: String,
        card_holder: String,
        expiration_date: String,
    },
    BankAccount {
        account_number: String,
        bank_name: String,
    },
}

#[test]
fn test_subscriber_serialization() {
    let subscriber = Subscriber {
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
        phone_number: Some("123-456-7890".to_string()),
        gender: Gender::Male,
        payment: Payment::CreditCard {
            card_number: "4111111111111111".to_string(),
            card_holder: "Bob".to_string(),
            expiration_date: "12/25".to_string(),
        },
    };

    let subscriber2 = Subscriber {
        name: "Eve".to_string(),
        email: "eve@example.com".to_string(),
        phone_number: Some("098-765-4321".to_string()),
        gender: Gender::Female,
        payment: Payment::BankAccount {
            account_number: "123456789".to_string(),
            bank_name: "Bank of Rust".to_string(),
        },
    };

    let json = to_string(&subscriber).unwrap();
    let json2 = to_string(&subscriber2).unwrap();
    println!("JSON: {}", json);
    println!("JSON2: {}", json2);

    let subscriber_result: Subscriber = from_str(&json).unwrap();
    let subscriber2_result: Subscriber = from_str(&json2).unwrap();
    println!("Subscriber Result: {:?}", subscriber_result);
    println!("Subscriber2 Result: {:?}", subscriber2_result);
}

/*
CHRONO
- Chrono sendiri memiliki module untuk membantu melakukan serde, namun terbatas hanya untuk tipe data DateTime<Utc>
*/

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    id: String,
    name: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    updated_at: DateTime<Utc>,
}

#[test]
fn test_category_serialization() {
    let category = Category {
        id: "1".to_string(),
        name: "Technology".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let json = to_string(&category).unwrap();
    println!("JSON: {}", json);

    let category_result: Category = from_str(&json).unwrap();
    println!("Category Result: {:?}", category_result);
}

/*
CUSTOM SERIALIZATION
- Kadang, ada kalanya kita butuh membuat implementasi Serialize dan Deserialize sendiri
- Rust Serde juga mendukung hal ini jika memang implementasinya belum tersedia,
sehingga kita bisa membuat implementasinya sendiri secara manual

CUSTOM SERIALIZE
- Untuk membuat implementasi Serialize, kita bisa implementasi trait Serialize di Serde

CUSTOM DESERIALIZE
- Untuk membuat implementasi Deserialize, kita bisa implementasi trait Deserialize di Serde
- Saat membuat Deserialize, biasanya kita akan membutuhkan Visitor untuk menentukan asal tipe data yang akan di-deserialize,
kita bisa memilih banyak jenis tipe data
*/

#[derive(Debug, Serialize, Deserialize)]
struct Admin {
    id: String,
    name: Name,
}

#[derive(Debug)]
struct Name {
    first: String,
    last: String,
}

struct NameVisitor;

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(format!("{} {}", self.first, self.last).as_str())
    }
}

impl<'de> Visitor<'de> for NameVisitor {
    type Value = Name;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string with first and last name")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let result: Vec<&str> = value.split(' ').collect();
        if result.len() != 2 {
            return Err(Error::custom("Expecting first and last name"));
        }

        Ok(Name {
            first: result[0].to_string(),
            last: result[1].to_string(),
        })
    }
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(NameVisitor)
    }
}

#[test]
fn test_admin_serialization() {
    let admin = Admin {
        id: "1".to_string(),
        name: Name {
            first: "Alice".to_string(),
            last: "Wonderland".to_string(),
        },
    };

    let json = to_string(&admin).unwrap();
    println!("JSON: {}", json);

    let admin_result: Admin = from_str(&json).unwrap();
    println!("Admin Result: {:?}", admin_result);
}

/*
SERDE MODULE
- Pada beberapa kasus, kadang kita butuh membuat Serialize dan Deserialize untuk tipe data yang terdapat di Crate lain
- Dan misal sayangnya, Crate tersebut bukanlah milik kita, contoh tipe data NaiveDateTime di library Chrono
- Kita tidak bisa membuat implementasi Serialize dan Deserialize karena NaiveDateTime tidak terdapat di Crate kita
- Namun untungnya, Serde menyediakan cara membuat Serialize dan Deserialize dengan cara membuat method
- Cara penggunaannya cukup gunakan attribute serde dan with
*/

pub mod to_ms {
    use std::fmt::Formatter;

    use chrono::{DateTime, NaiveDateTime};
    use serde::{
        Deserializer, Serializer,
        de::{Error, Visitor},
    };

    pub fn serialize<S>(datetime: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ms = datetime.and_utc().timestamp_millis();
        serializer.serialize_i64(ms)
    }

    struct NaiveDateTimeVisitor;

    impl<'de> Visitor<'de> for NaiveDateTimeVisitor {
        type Value = NaiveDateTime;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("Expecting i64")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            let datetime = DateTime::from_timestamp_millis(v as i64)
                .unwrap()
                .naive_utc();
            Ok(datetime)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(NaiveDateTimeVisitor)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SuperAdmin {
    id: String,
    name: Name,
    #[serde(with = "to_ms")]
    created_at: NaiveDateTime,
    #[serde(with = "to_ms")]
    updated_at: NaiveDateTime,
}

#[test]
fn test_serde_module() {
    let super_admin = SuperAdmin {
        id: "1".to_string(),
        name: Name {
            first: "Alice".to_string(),
            last: "Wonderland".to_string(),
        },
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    let json = to_string(&super_admin).unwrap();
    println!("JSON: {}", json);

    let super_admin_result: SuperAdmin = from_str(&json).unwrap();
    println!("SuperAdmin Result: {:?}", super_admin_result);
}
