use serde::Serialize;
use validator::{Validate, ValidateArgs};

fn main() {
    println!("Hello, world!");
}

/*
STRUCT
- Untuk melakukan validasi data, kita bisa membuat Struct lalu menambahkan informasi Atttribute/ Annotation pada field yang akan kita tambahkan validasi nya
- Library Validator mendukung banyak sekali jenis validasi, kita bisa lihat semua daftar jenis validasi yang tersedia di halaman README library Validator

MENAMBAHKAN VALIDASI
- Untuk menambahkan validasi, kita harus tambahkan derive Validate pada Struct yang akan di validasi
- Selanjutnya kita bisa menggunakan Attribute/ Annotation Validate pada tiap field yang akan kita validasi
- Selanjutnya kita bisa sebutkan jenis validasi yang akan kita gunakan

MELAKUKAN VALIDASI
- Saat kita menambahkan derive Validate, secara otomatis method `validate()` akan ditambahkan
- Method `validate()` ini digunakan untuk melakukan validasi

VALIDATION MESSAGE
- Saat kita membuat validasi, kadang kita ingin menambahkan pesan error pada validasi yang kita buat
- Kita bisa menambahkan attribute message saat menambah annotation validate
*/

#[derive(Debug, Validate)]
struct LoginRequest {
    #[validate(length(
        min = 5,
        max = 20,
        message = "Username must be between 5 and 20 characters"
    ))]
    username: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    password: String,
}

#[test]
fn test_validate_success() {
    let request = LoginRequest {
        username: "rizal".to_string(),
        password: "password".to_string(),
    };
    let result = request.validate();
    assert!(result.is_ok());
}

#[test]
fn test_validate_error() {
    let request = LoginRequest {
        username: "riz".to_string(),
        password: "pass".to_string(),
    };
    let result = request.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    println!("{:?}", errors.errors());
    assert_eq!(errors.field_errors().len(), 2);
}

/*
NESTED STRUCT
- Saat kita melakukan validasi, kadang kita melakukan validasi terhadap Struct yang memiliki field dengan tipe Struct yang lainnya
- Secara default, Validator tidak akan melakukan validasi terhadap field tersebut,
kecuali kita tambahkan jenis validasi nested
- Namun, field Nested Struct yang bisa divalidasi adalah jenis Struct yang juga implement trait Validate
*/

#[derive(Debug, Validate)]
struct AddressRequest {
    #[validate(length(min = 5, message = "Street must be at least 5 characters"))]
    street: String,
    #[validate(length(min = 2, message = "City must be at least 2 characters"))]
    city: String,
    #[validate(length(min = 2, message = "Country must be at least 2 characters"))]
    country: String,
}

#[derive(Debug, Validate)]
struct RegisterRequest {
    #[validate(length(
        min = 5,
        max = 20,
        message = "Username must be between 5 and 20 characters"
    ))]
    username: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    password: String,
    #[validate(email(message = "Email must be a valid email address"))]
    email: String,
    #[validate(nested)]
    address: AddressRequest,
}

#[test]
fn test_validate_nested_success() {
    let request = RegisterRequest {
        username: "rizal".to_string(),
        password: "password".to_string(),
        email: "rizal@test.com".to_string(),
        address: AddressRequest {
            street: "Jl. Merdeka No. 1".to_string(),
            city: "Jakarta".to_string(),
            country: "Indonesia".to_string(),
        },
    };

    let result = request.validate();
    assert!(result.is_ok());
}

#[test]
fn test_validate_nested_error() {
    let request = RegisterRequest {
        username: "rizal".to_string(),
        password: "password".to_string(),
        email: "rizal@mail.com".to_string(),
        address: AddressRequest {
            street: "Jl".to_string(),
            city: "J".to_string(),
            country: "I".to_string(),
        },
    };

    let result = request.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    println!("{:?}", errors.errors());
}

/*
COLLECTION
- Rust memiliki banyak tipe Collection, seperti Vec, Map, Set, dan lain-lain
- Kita juga bisa melakukan validasi untuk object - object yang terdapat di dalam Collection tersebut
- Syaratnya jika tipe data di dalam Collection tersebut adalah Struct,
maka Struct nya harus implement Validate dan juga serde::Serialize
- Sama seperti Nested Struct, secara default isi dari Collection tidak akan divalidasi,
kecuali kita gunakan jenis validation nested
*/

#[derive(Debug, Validate)]
struct Product {
    #[validate(length(min = 1, message = "ID must not be empty"))]
    id: String,
    #[validate(length(min = 1, message = "Name must not be empty"))]
    name: String,
    #[validate(nested, length(min = 1, message = "Variants must not be empty"))]
    variants: Vec<ProductVariant>,
}

#[derive(Debug, Validate, Serialize)]
struct ProductVariant {
    #[validate(length(min = 1, message = "Name must not be empty"))]
    name: String,
    #[validate(range(min = 1, message = "Price must be at least 1"))]
    price: u32,
}

#[test]
fn test_validate_collection_success() {
    let product = Product {
        id: "1".to_string(),
        name: "Product 1".to_string(),
        variants: vec![
            ProductVariant {
                name: "Variant 1".to_string(),
                price: 100,
            },
            ProductVariant {
                name: "Variant 2".to_string(),
                price: 200,
            },
        ],
    };

    let result = product.validate();
    assert!(result.is_ok());
}

#[test]
fn test_validate_collection_error() {
    let product = Product {
        id: "1".to_string(),
        name: "Product 1".to_string(),
        variants: vec![
            ProductVariant {
                name: "".to_string(),
                price: 0,
            },
            ProductVariant {
                name: "Variant 2".to_string(),
                price: 200,
            },
        ],
    };

    let result = product.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    println!("{:?}", errors.errors());
}

/*
CUSTOM VALIDATION
- Saat membuat validasi di aplikasi, kadang kita butuh membuat validasi yang baru yang mungkin tidak tersedia di library Validator
- Untungnya, library Validator menyediakan cara jika kita ingin membuat custom validation
- Kita bisa menggunakan jenis validasi custom, dimana kita bisa buat function untuk melakukan validasinya
- Function untuk melakukan validasinya harus mengembalikan `Result<(), ValidationError>`

STRUCT LEVEL VALIDATION
- Pada beberapa kasus, kadang kita butuh melakukan validasi di level Struct
- Misal, kita harus membandingkan beberapa field sekaligus
- Library Validator memiliki jenis validasi Schema, yang bisa kita gunakan untuk membuat validasi di level Struct
- Namun, untuk menggunakan validasi jenis Schema, kita harus membuat custom validation

CODE DAN MESSAGE
- Secara default, code akan menggunakan jenis validation, misal jika kita menggunakan validation required,
maka code nya adalah "required", jika menggunakan validation length, maka code nya adalah "length", dan seterusnya
- Namun, jika kita ingin mengubahnya, kita bisa mengubah nama code dengan menambahkan attribute `code` pada annotation validate nya
- Selain code, message juga bisa diubah dengan menggunakan attribute `message`, sehingga message default bawaan dari validation nya tidak akan digunakan

CONTEXT
- Pada beberapa kasus, kadang custom validation yang kita gunakan membutuhkan data object dari luar, misal koneksi database
- Function validation yang baik, tidak melakukan hardcode object yang dibutuhkan, sehingga bisa diubah - ubah ketika dipanggil function validationnya
- Library Validator memiliki fitur Context, dimana kita bisa mengirim data dari luar ketika kita memanggil function validationnya


*/

pub mod custom_validation {
    use std::borrow::Cow;

    use validator::ValidationError;

    use crate::RegisterCustomerRequest;

    pub fn not_blank(value: &str) -> Result<(), ValidationError> {
        if value.trim().is_empty() {
            return Err(ValidationError::new("not_blank")
                .with_message(Cow::from("Value must not be blank")));
        }

        Ok(())
    }

    pub fn passwords_match(request: &RegisterCustomerRequest) -> Result<(), ValidationError> {
        if request.password != request.confirm_password {
            return Err(ValidationError::new("passwords_match")
                .with_message(Cow::from("Password and Confirm Password must match")));
        }
        Ok(())
    }
}

#[derive(Debug, Validate)]
#[validate(context=DatabaseContext,
    schema(
    function = "custom_validation::passwords_match",
    skip_on_field_errors = false,
    code = "password",
    message = "password != confirm_password"
    ),
    schema(
        function = "context_validation::can_register",
        skip_on_field_errors = false,
        code = "can_register",
        use_context
    )
)]
struct RegisterCustomerRequest {
    #[validate(length(
        min = 5,
        max = 20,
        message = "Username must be between 5 and 20 characters",
        code = "username"
    ))]
    username: String,
    #[validate(length(
        min = 8,
        message = "Password must be at least 8 characters",
        code = "password"
    ))]
    password: String,
    confirm_password: String,
}

#[derive(Debug, Validate)]
struct CategoryRequest {
    #[validate(custom(function = "custom_validation::not_blank"))]
    name: String,
}

#[test]
fn test_custom_validation_success() {
    let category = CategoryRequest {
        name: "Category 1".to_string(),
    };

    let result = category.validate();
    assert!(result.is_ok());
}

#[test]
fn test_custom_validation_error() {
    let category = CategoryRequest {
        name: "   ".to_string(),
    };
    let result = category.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    println!("{:?}", errors.errors());
}

#[test]
fn test_struct_level_validation_success() {
    let request = RegisterCustomerRequest {
        username: "rizal".to_string(),
        password: "password".to_string(),
        confirm_password: "password".to_string(),
    };

    let result = request.validate_with_args(&DatabaseContext {
        total: 5,
        max_data: 10,
    });
    assert!(result.is_ok());
}

#[test]
fn test_struct_level_validation_error() {
    let request = RegisterCustomerRequest {
        username: "rizal".to_string(),
        password: "password".to_string(),
        confirm_password: "password1".to_string(),
    };
    let result = request.validate_with_args(&DatabaseContext {
        total: 10,
        max_data: 10,
    });
    assert!(result.is_err());
    let errors = result.unwrap_err();
    println!("{:?}", errors.errors());
}

pub struct DatabaseContext {
    total: i32,
    max_data: i32,
}

pub mod context_validation {
    use std::borrow::Cow;

    use validator::ValidationError;

    use crate::{DatabaseContext, RegisterCustomerRequest};

    pub fn can_register(
        request: &RegisterCustomerRequest,
        context: &DatabaseContext,
    ) -> Result<(), ValidationError> {
        if context.total >= context.max_data {
            return Err(
                ValidationError::new("can_register").with_message(Cow::from(format!(
                    "Cannot register user {}, database is full",
                    request.username
                ))),
            );
        }
        Ok(())
    }
}
