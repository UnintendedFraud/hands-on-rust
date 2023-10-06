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
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse !"),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse ! {}", note);
                if self.age < 18 {
                    println!("No alcohol");
                }
            }
            VisitorAction::Probation => println!("Probation welcome"),
            _ => println!("Get out !!"),
        }
    }
}

fn main() {
    let mut list = get_allowed_list();

    loop {
        println!("Hello, what's your name? (leave empty and press ENTER to quit)");

        let name = get_name();

        let known_visitor = list.iter().find(|v| v.name.to_lowercase() == *name);

        match known_visitor {
            Some(visitor) => visitor.greet(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the list", name);
                    list.push(Visitor::new(&name, VisitorAction::Probation, 0));
                }
            }
        }
    }

    println!("The final list:");
    println!("{:#?}", list);
}

fn get_name() -> String {
    let mut name = String::new();

    stdin()
        .read_line(&mut name)
        .expect("failed to read the name");

    name.trim().to_lowercase()
}

fn get_allowed_list() -> Vec<Visitor> {
    return vec![
        Visitor::new("Brice", VisitorAction::Accept, 24),
        Visitor::new(
            "Robert",
            VisitorAction::AcceptWithNote {
                note: String::from("Lactose-free drink in the fridge"),
            },
            15,
        ),
        Visitor::new("Michelle", VisitorAction::Refuse, 33),
    ];
}
