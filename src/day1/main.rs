use std::fs;

fn main() {
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path).expect("Could not read file");

    let mut sum: u16 = 0;

    for line in contents.lines() {
        let mut l = 0;
        let mut r = line.len() - 1;
        let zero = '0' as u16;

        while l <= r {
            let left_char = line.chars().nth(l).unwrap();
            let right_char = line.chars().nth(r).unwrap();

            if left_char.is_numeric() && right_char.is_numeric() {
                sum += 10 * (left_char as u16 - zero);
                sum += right_char as u16 - zero;
                break;
            }

            if !left_char.is_numeric() {
                l += 1;
            }

            if !right_char.is_numeric() {
                r -= 1;
            }
        }
    }

    println!("{}", sum);
}
