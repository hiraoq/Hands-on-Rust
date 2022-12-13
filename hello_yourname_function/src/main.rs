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
    if your_name == "bert" || your_name == "steve" {
        println!("Welcom, {:?}!", your_name);
    } else {
        println!("Sorry, You are not in list.")
    }
}
