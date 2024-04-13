use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}
impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet(&self) {
        println!("{}", self.greeting);
    }
}

fn ask_name() -> String {
    let mut user_name = String::new();
    stdin()
        .read_line(&mut user_name)
        .expect("Failed to read input!");

    user_name.trim().to_lowercase()
}

fn main() {
    let visitor_list = [
        Visitor::new("bubba", "Hey, Bubba! Enjoy de Bungalow!"),
        Visitor::new("steve", "Hi Steve, we got a package for you."),
        Visitor::new("holt", "Who invited the Captain?"),
    ];

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
