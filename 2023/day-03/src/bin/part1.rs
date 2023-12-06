use regex;
use std::char;
use std::fs;

fn get_result(file_name: &str) -> i32 {
    let read_lines = fs::read_to_string(file_name).unwrap();
    let mut lines = read_lines.lines();

    let mut prev_line: Option<&str>;
    let mut curr_line: Option<&str> = None;
    let mut next_line: Option<&str> = lines.next();
    let number_of_lines = read_lines.lines().count();

    dbg!("{}", number_of_lines);

    let mut begin_at_index;

    let mut results: Vec<i32> = [].to_vec();

    for _i in 0..number_of_lines {
        prev_line = curr_line;
        curr_line = next_line;
        next_line = lines.next();

        begin_at_index = 0;

        while let Some(next_number) = find_next_number(curr_line.unwrap(), begin_at_index) {
            if is_valid(&next_number, prev_line, curr_line, next_line) {
                results.push(next_number.number);
            }
            begin_at_index = next_number.end_index + 1;
        }
    }

    return results.iter().sum();
}

fn is_valid(
    next_number: &NextNumber,
    prev_line: Option<&str>,
    curr_line: Option<&str>,
    next_line: Option<&str>,
) -> bool {
    let line_length = curr_line.unwrap_or("").len();
    let left_index: usize = if next_number.start_index >= 1 {
        next_number.start_index - 1
    } else {
        0
    };
    let right_index: usize = if next_number.end_index < line_length {
        next_number.end_index + 1
    } else {
        line_length
    };
    let checker = |c: char| c == '.' || c.is_numeric();

    let mut valid: bool = false;

    match curr_line {
        Some(line) => {
            let left_char = line.chars().nth(left_index).unwrap_or('.');
            let right_char = line.chars().nth(right_index).unwrap_or('.');

            if !checker(left_char) || !checker(right_char) {
                valid = true;
            }
        }
        None => {}
    }

    if valid == true {
        return valid;
    }

    match prev_line {
        Some(line) => {
            let relevant_slice = if right_index < line_length {
                &line[left_index..(right_index + 1)]
            } else {
                &line[left_index..]
            };
            if relevant_slice.chars().any(|c: char| !checker(c)) {
                valid = true;
            }
        }
        None => {}
    }

    if valid == true {
        return valid;
    }

    match next_line {
        Some(line) => {
            let relevant_slice = if right_index < line_length {
                &line[left_index..(right_index + 1)]
            } else {
                &line[left_index..]
            };
            if relevant_slice.chars().any(|c: char| !checker(c)) {
                valid = true;
            }
        }
        None => {}
    }

    valid
}

struct NextNumber {
    number: i32,
    start_index: usize,
    end_index: usize,
}

fn find_next_number(line: &str, begin_at_index: usize) -> Option<NextNumber> {
    let relevant_slice = &line.to_string()[begin_at_index..];

    let pattern = regex::Regex::new(r"\d+").unwrap();

    if let Some(mat) = pattern.find(relevant_slice) {
        let number = mat.as_str().parse::<i32>().unwrap();
        let start_index = mat.start() + begin_at_index;
        let end_index = start_index + mat.as_str().len() - 1;

        return Some(NextNumber {
            number,
            start_index,
            end_index,
        });
    }

    None
}

fn main() {
    let result = get_result("./part1_input.txt");

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_result() {
        let result = get_result("./part1_test_input.txt");
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_find_next_number() {
        let result = find_next_number(".....1231......", 0).unwrap();
        assert_eq!(result.number, 1231);
        assert_eq!(result.start_index, 5);
        assert_eq!(result.end_index, 8);
        let result = find_next_number("#=?.....1231..##./.ö..333......", 0).unwrap();
        assert_eq!(result.number, 1231);
        assert_eq!(result.start_index, 8);
        assert_eq!(result.end_index, 11);
        let result = find_next_number("#=?.....1231..##./.ö..333......", 12).unwrap();
        assert_eq!(result.number, 333);
        assert_eq!(result.start_index, 23);
        assert_eq!(result.end_index, 25);
    }
}
