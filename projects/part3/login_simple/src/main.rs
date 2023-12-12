use login_lib_simple::{read_line, login};

fn main() {
    let mut tries = 0;
    loop {
        println!("Enter your username:");
        let username = read_line();
        println!("Enter your password:");
        let password = read_line();
        if login(&username, &password) {
            println!("Welcome, {username}!");
            break;
        } else {
            println!("Login failed.");
            tries += 1;
            if tries >= 3 {
                println!("Too many failed attempts. Exiting.");
                break;
            }
        }
    }
}