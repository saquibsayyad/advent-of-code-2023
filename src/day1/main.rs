use std::fs;

fn main() {
    // Read file
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    // Read line by line
    // let mut result = Vec::new();
    let mut sum = 0;

    for line in contents.lines() {

        println!("{}",line);
    }

    sum = 1;

    // println!("{:?}", result)
    println!("{}", sum);
}