#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}
impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
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
    let mut visitor_list = vec![
        Visitor::new("bubba", 29, "Hey, Bubba! Enjoy de Bungalow!"),
        Visitor::new("steve", 18, "Hi Steve, we got a package for you."),
        Visitor::new("holt", 37, "Who invited the Captain?"),
    ];

    loop {
        println!("Hello! What's your name? (Press ENTER to close)");
        let name = ask_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the list.", name);
                    visitor_list.push(Visitor::new(&name, "New friend."));
                }
            }
        }
    }

    println!("The final list of visitor");
    println!("{:#?}", visitor_list);
}
