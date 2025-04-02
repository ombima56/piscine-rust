use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = "the letter e";
    let mut trials = 0;

    loop {
        println!("{}", riddle);
        
        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        let response = response.trim();
        
        trials += 1;
        
        if response.eq_ignore_ascii_case(answer) || response.eq_ignore_ascii_case("e") {
            println!("Number of trials: {}", trials);
            break;
        }
    }
}
