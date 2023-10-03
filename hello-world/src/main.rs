#![warn(clippy::all, clippy::pedantic)]

fn main() {
    let my_list = ["one", "two", "three"];

    for item in my_list {
        print!("{}", item)
    }

    println!("Hello, world!");
}
