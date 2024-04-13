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

    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
    match known_visitor {
        Some(visitor) => visitor.greet(),
        None => println!("You're not on the list. Move on!"),
    }
}
