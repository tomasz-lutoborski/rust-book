use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("query: {}, filename: {}", query, filename);

    let contents = fs::read_to_string(filename).expect("somethin went wrong with reading file");

    println!("contents:/n{}", contents);
}
