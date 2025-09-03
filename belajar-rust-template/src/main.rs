use std::collections::HashMap;

use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, handlebars_helper,
};
use serde::Serialize;
use serde_json::json;

fn main() {
    println!("Hello, world!");
}

/*
TEMPLATING
- Templating adalah proses atau teknik dalam pemrograman yang digunakan untuk membuat struktur atau kerangka dokumen,
file, atau kode yang bersifat dinamis
- Dengan menggunakan template, bagian - bagian yang bersifat tetap (statis) dan bagian yang berubah (dinamis) dapat dipisahkan.
Biasanya, templating digunakan untuk menghasilkan output seperti halaman web, laporan, atau email dengan konten yang dapat disesuaikan

MANFAAT TEMPLATING
- Menghemat waktu dengan menghindari duplikasi kode
- Mempermudah pemeliharaan kode karena perubahan pada template akan tercermin di semua output yang menggunakannya
- Memisahkan logika pemrograman dari tampilan (seperation of concerns)

RUST TEMPLATE LIBRARY
- Rust tidak menyediakan template library secara default, oleh karena itu, kita perlu menggunakan third party library untuk melakukan templating
- Salah satu format template yang banyak digunakan adalah Mustache dan Handlebars
- Dan salah satu library yang bisa kita gunakan untuk implementasi Mustache atau Handlebars adalah Library Rust Handlebars

SETUP
- Untuk menggunakan Library Handlebars, kita bisa membuat object dari Struct Handlebars

TEMPLATE
- Setelah membuat object Handlebars, kita perlu registrasikan dulu template - template yang kita buat
- Setiap template perlu ditambah key secara unique
- Selanjutnya ketika ingin menggunakan template yang kita buat, kita bisa menggunakan key yang sudah kita tentukan

RENDER
- Setelah kita membuat template, kita melakukan proses render, yaitu mengisi template dengan data yang kita inginkan
- Dengan begitu, kita bisa menggunakan data dinamis sesuai yang kita inginkan hanya dengan menggunakan template yang sama
- Kita bisa menggunakan method render pada object Handlebars
*/

#[test]
fn test_handlebars() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_string("hello", "Hello, {{name}}")
        .unwrap();

    handlebars
        .register_template_string("bye", "Bye, {{name}}")
        .unwrap();

    let mut data = HashMap::new();
    data.insert("name", "Zhafir");

    let rendered = handlebars.render("hello", &data).unwrap();
    println!("{}", rendered);
    assert_eq!(rendered, "Hello, Zhafir");

    let rendered = handlebars.render("bye", &data).unwrap();
    println!("{}", rendered);
    assert_eq!(rendered, "Bye, Zhafir");
}

/*
TEMPLATE VARIABLE
- Saat kita membuat template, kita bisa membuat variable di template yang bisa kita ganti menjadi value yang dinamis
- Kita bisa menggunakan {{ dan diikuti nama variable dan ditutup dengan }}
- `{{name}}`

NESTED VARIABLE
- Kita juga bisa menggunakan Nested Variable di Template, misal kita buat menggunakan `{{person.first_name}}` dan `{{person.last_name}}`
- Saat menggunakan Nested Variable, kita berarti harus menggunakan data yang nested juga, misal Map di dalam map atau nested struct
*/

#[test]
fn test_handlebars_nested_variable() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_string("hello", "Hello, {{person.first_name}} {{person.last_name}}")
        .unwrap();

    let mut data = HashMap::new();
    let mut person = HashMap::new();
    person.insert("first_name", "Zhafir");
    person.insert("last_name", "Hafidz");
    data.insert("person", person);

    let rendered = handlebars.render("hello", &data).unwrap();
    println!("{}", rendered);
    assert_eq!(rendered, "Hello, Zhafir Hafidz");
}

/*
HTML ESCAPE
- Handlebars biasanya digunakan untuk membuat template dalam bentuk HTML, oleh karena itu jika kita mengirimkan data berupa karakter - karakter yang mempresentasikan tag HTML,
maka secara otomatis akan di escape agar aman
- Namun, jika kita memang tidak ingin melakukan HTML Escape, kita bisa menggunakan tanda kurung kurawal sebanyak 3 kali
- `{{{name}}}`
*/

#[test]
fn test_handlebars_html_escape() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_string("hello", "Hello, {{{name}}}")
        .unwrap();

    let mut data = HashMap::new();
    data.insert("name", "<b>Zhafir</b>");

    let rendered = handlebars.render("hello", &data).unwrap();
    println!("{}", rendered);
    assert_eq!(rendered, "Hello, <b>Zhafir</b>");
}

/*
TEMPLATE FILE
- Saat kita membuat template, jarang sekali kita buat dalam bentuk String
- Biasanya, kita akan simpan dalam file .mustache atau file .hbs
- Struct Handlebars memiliki method khusus untuk mengambil template dari file
*/

#[test]
fn test_handlebars_template_file() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("hello", "templates/hello.mustache")
        .unwrap();

    let mut data = HashMap::new();
    data.insert("name", "Zhafir");

    let rendered = handlebars.render("hello", &data).unwrap();
    println!("{}", rendered);
    assert_eq!(rendered, "Hello, Zhafir");
}

/*
WITH
- Saat kita menggunakan Nested Object yang banyak, kadang agak menyulitkan jika kita harus terus - terusan menyebutkan parent variable nya
- Handlebars memiliki helper bernama `#with`, dimana kita bisa menyebutkan nama variable nya tanpa harus menggunakan parent variable nya lagi
*/

#[test]
fn test_handlebars_with() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("hello", "templates/with-hello.mustache")
        .unwrap();

    let mut data = HashMap::new();
    let mut person = HashMap::new();
    person.insert("first_name", "Zhafir");
    person.insert("last_name", "Hafidz");
    data.insert("person", person);

    let rendered = handlebars.render("hello", &data).unwrap();
    println!("{}", rendered);
    assert_eq!(rendered.contains("<h1>Hello, Zhafir Hafidz</h1>"), true);
}

/*
SERDE
- Kadang, agak sulit ketika harus membuat data template dengan tipe data Map
- Handlebars bisa diintegrasikan dengan Library Serde, sehingga kita bisa membuat Struct yang memiliki implementasi Serialize,
dan bisa kita gunakan sebagai data untuk template di Handlebars
*/

#[derive(Debug, Serialize)]
struct Person {
    first_name: String,
    last_name: String,
}

#[derive(Debug, Serialize)]
struct Data {
    person: Person,
}

#[test]
fn test_handlebars_serde() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("hello", "templates/with-hello.mustache")
        .unwrap();

    let data = Data {
        person: Person {
            first_name: "Zhafir".to_string(),
            last_name: "Hafidz".to_string(),
        },
    };

    let rendered = handlebars.render("hello", &data).unwrap();
    println!("{}", rendered);
    assert_eq!(rendered.contains("<h1>Hello, Zhafir Hafidz</h1>"), true);
}

/*
SERDE JSON
- Selain dengan Struct Serialize, Handlebars juga bisa diintegrasikan dengan JSON value dari serde JSON
- Kita bisa menggunakan macro json! untuk membuat data untuk template nya
*/

#[test]
fn test_handlebars_serde_json() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("hello", "templates/with-hello.mustache")
        .unwrap();

    let data = json!({
        "person": {
            "first_name": "Zhafir",
            "last_name": "Hafidz"
        }
    });

    let rendered = handlebars.render("hello", &data).unwrap();
    println!("{}", rendered);
    assert_eq!(rendered.contains("<h1>Hello, Zhafir Hafidz</h1>"), true);
}

/*
IF
- Handlebars memiliki helper #if yang bisa digunakan untuk melakukan pengecekan #if
- Jika kondisi dalam #if bernilai false, undefined, null, "", 0, atau [], maka isi #if tidak akan ditampilkan
- #if juga memiliki bagian #else jika kita ingin menampilkan sesuatu ketika memang kondisi #if bernilai false
*/

#[test]
fn test_handlebars_if() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("blog", "templates/blog.mustache")
        .unwrap();

    let data = json!({
        "title": "Belajar Rust",
        "content": "Belajar Rust",
        "author": "Zhafir"
    });

    let rendered = handlebars.render("blog", &data).unwrap();
    println!("{}", rendered);
    assert_eq!(rendered.contains("<h1>Belajar Rust</h1>"), true);
}

/*
UNLESS
- Selain #if, Handlebars memiliki helper #unless
- Unless ini digunakan kebalikan dari #if, dimana isi #unless akan ditampilkan ketika memang nilai kondisinya bernilai false,
undefined, null, "", 0, atau []
*/

#[test]
fn test_handlebars_unless() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("footer", "templates/footer.mustache")
        .unwrap();

    let data = json!({
        "footer": false
    });

    let rendered = handlebars.render("footer", &data).unwrap();
    println!("{}", rendered);
    assert_eq!(
        rendered.contains("This content doesn't have a footer."),
        true
    );
}

/*
EACH
- Handlebars memiliki helper #each yang digunakan untuk melakukan iterasi data dalam bentuk Collection, seperti Array atau Vector
- Dalam blok #each, kita bisa menggunakan kata kunci this sebagai referensi ke value di iterasi tersebut

EACH INDEX
- Kadang saat melakukan iterasi data, kita ingin mengetahui nomor indexnya, kita bisa menggunakan @index di dalam blok #each

EACH ARRAY OF OBJECT
- Helper #each juga bisa digunakan untuk melakukan iterasi pada Vector yang berisi Object misal Struct
- Untuk mengakses data field dari objectnya, kita bisa gunakan this.name_field

EACH OBJECT
- Selain melakukan iterasi data array yang berisi object, kita juga bisa melakukan iterasi data object
- Melakukan iterasi data object, artinya iterasi akan dilakukan untuk tiap fieldnya,
dan untuk mendapatkan informasi nama fieldnya, kita bisa gunakan @key
*/

#[derive(Debug, Serialize)]
struct Customer {
    first_name: String,
    last_name: String,
    hobbies: Vec<String>,
    addresses: Vec<Address>,
}

#[derive(Debug, Serialize)]
struct Address {
    street: String,
    city: String,
}

#[test]
fn test_handlebars_each() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("person", "templates/person.mustache")
        .unwrap();

    let data = Customer {
        first_name: "Zhafir".to_string(),
        last_name: "Hafidz".to_string(),
        hobbies: vec![
            "Coding".to_string(),
            "Reading".to_string(),
            "Gaming".to_string(),
        ],
        addresses: vec![
            Address {
                street: "Jl. Merdeka".to_string(),
                city: "Jakarta".to_string(),
            },
            Address {
                street: "Jl. Sudirman".to_string(),
                city: "Bandung".to_string(),
            },
            Address {
                street: "Jl. Thamrin".to_string(),
                city: "Jakarta".to_string(),
            },
            Address {
                street: "Jl. Kebon Jeruk".to_string(),
                city: "Jakarta".to_string(),
            },
        ],
    };

    let rendered = handlebars.render("person", &data).unwrap();
    println!("{}", rendered);
    assert_eq!(rendered.contains("Zhafir"), true);
}

/*
HELPER
- Sebelumnya kita sudah bahas banyak sekali helper yang ada di Handlebars, helper diawali dengan #
- Handlebars juga menyediakan fitur agar kita bisa membuat custom helper,
sehingga kita bisa mudah menambahkan helper yang kita mau

HELPERDEF
- Untuk helper, kita bisa membuat implementasi trait HelperDef
- Setelah itu, kita bisa registrasikan ke Handlebars dengan method register_helper
*/

struct DoubleNumber;

impl HelperDef for DoubleNumber {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).unwrap().value();
        let number = param.as_i64().unwrap();
        out.write(&format!("{}", number * 2))?;
        Ok(())
    }
}

#[test]
fn test_custom_helper() {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("double", Box::new(DoubleNumber));
    handlebars
        .register_template_string("helper", "Result: {{double value}}")
        .unwrap();

    let data = json!({
        "value": 10
    });

    let rendered = handlebars.render("helper", &data).unwrap();
    println!("{}", rendered);
    assert_eq!(rendered.contains("Result: 20"), true);
}

/*
HELPER MACRO
- Kadang pada kasus yang sederhana, membuat implementasi HelperDef terasa rumit
- Untungnya Handlebars memiliki macro yang bisa kita gunakan untuk mempermudah dalam membuat helper, kita bisa gunakan macro handlebars_helper!
*/

handlebars_helper!(uppercase: | value: String | value.to_uppercase());

#[test]
fn test_helper_macro() {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("uppercase", Box::new(uppercase));
    handlebars
        .register_template_string("helper", "Result: {{uppercase value}}")
        .unwrap();

    let data = json!({
        "value": "hello"
    });

    let rendered = handlebars.render("helper", &data).unwrap();
    println!("{}", rendered);
    assert_eq!(rendered.contains("Result: HELLO"), true);
}

/*
PARTIALS
- Saat kita membuat template, kadang kita akan bagi jadi beberapa template, misal bagian header, content, dan footer
- Handlebars juga mendukung hal ini, kita bisa menggunakan feature bernama partials
- Cara penggunaannya adalah menggunakan {{> nama_tempalte}}, secara otomatis nama_template tersebut akan di include
*/

#[test]
fn test_handlebars_partials() {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("template/blog", "templates/layouts/blog.mustache")
        .unwrap();
    handlebars
        .register_template_file(
            "template/layouts/header",
            "templates/layouts/header.mustache",
        )
        .unwrap();
    handlebars
        .register_template_file(
            "template/layouts/footer",
            "templates/layouts/footer.mustache",
        )
        .unwrap();

    let data = json!({
        "title": "Belajar Rust",
        "content": "Belajar Rust",
        "author": "Zhafir",
        "footer": "Zhafir Hafidz"
    });

    let rendered = handlebars.render("template/blog", &data).unwrap();
    println!("{}", rendered);
    assert_eq!(rendered.contains("Belajar Rust"), true);
}
