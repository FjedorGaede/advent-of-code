use std::fs;

fn get_digit(line: &str) -> i32 {
    let mut first_digit = String::from("");
    let mut last_digit = String::from("");
    for c in line.chars() {
        if c.is_numeric() {
            if first_digit.is_empty() {
                first_digit = c.to_string();
            }
            last_digit = c.to_string();
        }
    }

    return format!("{}{}", first_digit, last_digit).parse().unwrap();
}

fn main() {
    let lines_iterator = fs::read_to_string("./part1_input.txt").unwrap();

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
        assert_eq!(get_digit("aa3dasd12dd11"), 31);
        assert_eq!(get_digit("1"), 11);
    }
}
