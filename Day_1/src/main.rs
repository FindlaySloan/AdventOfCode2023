use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path: &str = "src/puzzle.in";

    let file_contents: String =
        fs::read_to_string(file_path).expect("Error occured when reading the input file");

    let total_part_1: u32 = file_contents
        .split("\n")
        .map(|s| extract_number_part_1(s))
        .sum();

    let total_part_2: u32 = file_contents
        .split("\n")
        .map(|s| extract_number_part_2(s))
        .sum();

    println!("Total - Part 1: {total_part_1}");
    println!("Total - Part 2: {total_part_2}");
}

fn extract_number_part_1(string: &str) -> u32 {
    let err_msg: &str = "Found an entry with no numbers";

    let res: Vec<char> = string.chars().filter(|c| c.is_digit(10)).collect();

    let number: u32 = res.first().expect(err_msg).to_digit(10).unwrap() * 10
        + res.last().expect(err_msg).to_digit(10).unwrap();

    return number;
}

fn extract_number_part_2(string: &str) -> u32 {
    let lookup: Vec<(u32, &str)> = vec![
        (3, "one"),
        (3, "two"),
        (5, "three"),
        (4, "four"),
        (4, "five"),
        (3, "six"),
        (5, "seven"),
        (5, "eight"),
        (4, "nine"),
    ];

    let string_mapper: HashMap<&'static str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let err_msg: &str = "Found an entry with no numbers";

    let chars: Vec<char> = string.chars().collect();

    let mut digits: Vec<u32> = Vec::new();

    for i in 0..chars.len() {
        match chars[i].to_digit(10) {
            Some(x) => digits.push(x),
            None => {
                for &(x, value) in &lookup {
                    if i + (x as usize) <= chars.len()
                        && chars[i..i + x as usize].iter().collect::<String>() == value
                    {
                        digits.push(string_mapper[value])
                    }
                }
            }
        }
    }

    let number: u32 = digits.first().expect(err_msg) * 10 + digits.last().expect(err_msg);

    println!("{string} -> {number}");

    return number;
}
