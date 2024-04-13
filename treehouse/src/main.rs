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

    println!("Hello! What's your name?");

    let name = ask_name();
    let mut let_them_in = false;

    for visitor in &visitor_list {
        if visitor == &name {
            let_them_in = true;
        }
    }

    if let_them_in {
        println!("Welcome to the Bungalow!");
    } else {
        println!("DENIED!");
    }
}
