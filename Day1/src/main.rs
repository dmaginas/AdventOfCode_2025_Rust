use std::{env, fs, process};

fn main() {
    let input_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());

    let input = fs::read_to_string(&input_path).unwrap_or_else(|err| {
        eprintln!("Failed to read `{}`: {}", input_path, err);
        process::exit(1);
    });

    match day1::solve(&input) {
        Ok(answer) => println!("{answer}"),
        Err(err) => {
            eprintln!("Failed to solve: {err}");
            process::exit(1);
        }
    }
}
