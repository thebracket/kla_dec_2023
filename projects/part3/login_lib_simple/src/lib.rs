pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub fn login(username: &str, password: &str) -> bool {
    username.to_lowercase() == "admin" && password == "password"
}

#[cfg(test)]
mod test {
    use super::login;

    #[test]
    fn test_login() {
        assert!(login("admin", "password"));
        assert!(!login("admin", "wrong"));
        assert!(!login("wrong", "password"));
    }
}
