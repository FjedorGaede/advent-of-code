use regex::{self, Regex};
use std::collections::HashMap;
use std::fs;

fn get_digit(line: &str) -> i32 {
    let mut digits: Vec<&str> = Vec::new();

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

    let number_patterns: Vec<String> = val_map
        .iter()
        .map(|(numb_name, numb)| format!("{}|{}", numb_name.to_string(), numb.to_string()))
        .collect();

    let number_pattern = number_patterns.join("|");

    let pattern_string = format!(r"(?:{})", number_pattern);
    let pattern = Regex::new(&pattern_string).unwrap();

    for mat in pattern.find_iter(line) {
        let mat_as_str = mat.as_str();
        if mat_as_str.len() == 1 {
            digits.push(mat_as_str);
        } else {
            digits.push(val_map[mat_as_str]);
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
