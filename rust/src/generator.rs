use std::{env, process};

const DEFAULT_ROW_COUNT: i32 = 1_000_000_000;

fn main() {
    let rows = &env::args()
        .nth(1)
        .unwrap_or_else(|| {
            eprintln!("Expected some number to be passed as an argument.");
            process::exit(1);
        })
        .parse::<i32>()
        .unwrap_or_else(|_| {
            println!("Could not parse the argument as i32.");
            println!("Make sure a valid integer was provided as argument.");
            println!("Falling back to default: 1_000_000_000");
            DEFAULT_ROW_COUNT
        });

    println!("{}", &rows);
}
