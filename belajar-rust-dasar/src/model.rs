pub struct User {
    pub name: String,
    pub age: u8,
    pub email: String,
}

impl User {
    pub fn say_hello(&self, name: &str) {
        // This method takes a reference to self and a string slice as an argument
        println!("Hello, {}! My name is {} and I am {} years old.", name, self.name, self.age);
    }
}