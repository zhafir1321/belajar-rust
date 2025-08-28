pub fn say_hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::say_hello;

    #[test]
    fn test_say_hello() {
        let result = say_hello("Alice");
        assert_eq!(result, "Hello, Alice!", "Expected greeting for Alice");
    }
}
