use std::io::stdin;

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    your_name.trim().to_lowercase()
}

fn main() {
    println!("Helo, What's your name?");
    let your_name = what_is_your_name();
    welcome_visitor(your_name);
}

fn welcome_visitor(name: String) {
    let visitor_list = ["bert", "steve", "fred"];
    let mut allow_them_in = false;
    for visitor in &visitor_list {
        if visitor == &name {
            allow_them_in = true;
            break;
        }
    }

    if allow_them_in {
        println!("Welcome to the Treehouse, {}!", name);
    } else {
        println!("Sorry, you aren't on the list.");
    }
}
