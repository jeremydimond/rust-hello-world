pub mod hello {
    pub fn say_hello(name: &String) -> String {
        format!("Hello \"{}\"!  Your name has {} characters.", name, name.len())
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_say_hello() {
            let name = "blah blah".to_string();
            assert_eq!(say_hello(&name), "Hello \"blah blah\"!  Your name has 9 characters.");
        }
    }
}

