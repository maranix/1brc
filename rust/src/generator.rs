use std::{env, process};

const DEFAULT_ROW_COUNT: i32 = 1_000_000_000;

fn main() {
    let rows = &env::args()
        .nth(1)
        .unwrap_or_else(|| {
            eprintln!("Expected some number to be passed as an argument.");
            eprintln!("Consider the below example for generating 10000 rows.\n");
            eprintln!("cargo run --bin generator -- 10000");
            process::exit(1);
        })
        .parse::<i32>()
        .unwrap_or_else(|_| {
            eprintln!("Could not parse the argument as i32.");
            eprintln!("Make sure a valid integer was provided as argument.");
            eprintln!("Falling back to default: 1_000_000_000");
            DEFAULT_ROW_COUNT
        });

    println!("{}", &rows);
}
