use std::io::stdin;

fn ask_name() -> String {
    let mut user_name = String::new();
    stdin()
        .read_line(&mut user_name)
        .expect("Failed to read input!");

    user_name.trim().to_lowercase()
}

fn main() {
    let visitor_list = ["bubba", "annita", "chimichanga"];

    println!("Welcome to the Bungalow! What's your name?");

    let name = ask_name();
    for i in 0..visitor_list.len() {
        if visitor_list[i] == name {
            println!("Hello, {}", name);
        }
    }
}
