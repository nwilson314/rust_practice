use std::io;
use rand::Rng;

fn print_intro() {
    println!("Welcome to a game of Paper, Scissors, Rocks.");
    println!("To make a selection, select 1, 2, or 3. You can exit at any time by entering 'q' or 'quit'\n");
}

fn get_computer_choice() -> String {
    let comp_choice = rand::thread_rng().gen_range(1..=3);
    comp_choice.to_string()
}

fn print_rps(choice: &String) -> String {
    match choice.trim() {
        "1" => String::from("Rock"),
        "2" => String::from("Paper"),
        "3" => String::from("Scissors"),
        _ => String::from("Not valid")
    }
}

fn main() {
    print_intro();

    loop {
        println!("Please input your choice.");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).unwrap();

        let comp_choice: String;

        match choice.trim() {
            "q" | "quit" => break,
            "1"| "2"| "3" => { comp_choice = get_computer_choice(); },
            _ => continue,
        };

        println!("You chose {} and the computer chose {}", print_rps(&choice), print_rps(&comp_choice));
        match (choice.trim(), comp_choice.trim()) {
            ("1", "1") | ("2", "2") | ("3", "3") => println!("It's a tie!"),
            ("1", "3") | ("2", "1") | ("3", "2") => println!("You won!"),
            ("1", "2") | ("2", "3") | ("3", "1") => println!("You lost!"),
            _ => continue
        }
    }
    println!("\nThanks for playing!")
}
