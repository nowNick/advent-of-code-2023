use std::env;
use std::fs;

use task_1::trebuchet;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Too few arguments passed. Usage: ./run <file_path>")
    }
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("The result is: {}", trebuchet(&contents))
}
