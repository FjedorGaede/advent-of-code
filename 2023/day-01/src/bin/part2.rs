use std::collections::HashMap;
use std::fs;

fn get_digit(line: &str) -> i32 {
    let mut digits: Vec<String> = Vec::new();

    let val_map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        // ("zero", "0"),
    ]);

    for (index, char) in line.chars().enumerate() {
        if char.is_numeric() {
            digits.push(char.to_string());
        } else {
            for val in val_map.keys() {
                if line[index..].starts_with(val) {
                    digits.push(val_map.get(val).unwrap().to_string());
                }
            }
        }
    }

    let result = format!("{}{}", digits[0], digits[digits.len() - 1])
        .parse()
        .unwrap();

    // println!("Result: {}", result);
    return result;
}

fn main() {
    let lines_iterator = fs::read_to_string("./part2_input.txt").unwrap();

    let mut result = 0;

    for line in lines_iterator.lines() {
        result += get_digit(line);
    }

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(get_digit("onetwothree4"), 14);
        assert_eq!(get_digit("onetwothree"), 13);
        assert_eq!(get_digit("one1"), 11);
        assert_eq!(get_digit("one"), 11);
        assert_eq!(get_digit("eightwo"), 82);
        assert_eq!(get_digit("145612"), 12);
    }
}
