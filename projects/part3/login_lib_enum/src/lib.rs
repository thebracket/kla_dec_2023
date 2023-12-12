pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Admin,
    User,
    Denied,
}

pub fn login(username: &str, password: &str) -> LoginAction {
    let username = username.to_lowercase();
    if username == "admin" && password == "password" {
        LoginAction::Admin
    } else if username == "bob" && password == "password" {
        LoginAction::User
    } else {
        LoginAction::Denied
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_enums() {
        assert_eq!(login("admin", "password"), LoginAction::Admin);
        assert_eq!(login("bob", "password"), LoginAction::User);
        assert_eq!(login("admin", "wrong"), LoginAction::Denied);
        assert_eq!(login("wrong", "password"), LoginAction::Denied);
    }
}
