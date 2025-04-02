use std::io::{self, Write};

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = "the letter e";
    let mut trials = 0;

    loop {
        println!("{}", riddle);
        print!("What is your answer? ");
        io::stdout().flush().unwrap();

        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        let response = response.trim();

        trials += 1;

        if response.eq_ignore_ascii_case(answer) {
            println!("Number of trials: {}", trials);
            break;
        } else {
            println!("Sorry, that is not the answer. Please try again.");
        }
    }
}
