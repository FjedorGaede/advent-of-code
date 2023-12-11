use std::collections;
use std::fs;

fn get_result(file_name: &str) -> usize {
    let read_lines = fs::read_to_string(file_name).unwrap();

    let mut number_of_copies: collections::HashMap<usize, usize> = collections::HashMap::new();

    for (index, line) in read_lines.lines().enumerate() {
        let number_of_matches = get_result_for_line(line);

        let num_of_copies_on_index = number_of_copies.entry(index).or_insert(0);
        *num_of_copies_on_index += 1;

        let copies_for_next_cards: usize = num_of_copies_on_index.clone();

        if number_of_matches == 0 {
            continue;
        }
        for j in 1..(number_of_matches + 1) {
            let hash_key: usize = index + j;
            let num_copies = number_of_copies.entry(hash_key).or_insert(0);
            *num_copies += copies_for_next_cards;
        }
    }

    let result = number_of_copies.iter().map(|(key, value)| value).sum();

    return result;
}

fn get_result_for_line(line: &str) -> usize {
    let numbers_string: &str = line.split(":").collect::<Vec<&str>>().last().unwrap();
    let splitted = numbers_string.split("|").collect::<Vec<&str>>();
    let winning_numbers: Vec<usize> = splitted
        .first()
        .unwrap()
        .split(" ")
        .filter(|&x| !x.is_empty())
        .map(|num_str: &str| num_str.parse::<usize>().unwrap())
        .collect();
    let player_numbers: Vec<usize> = splitted
        .last()
        .unwrap()
        .split(" ")
        .filter(|&x| !x.is_empty())
        .map(|num_str: &str| num_str.parse::<usize>().unwrap())
        .collect();

    let mut counter = 0;
    for &player_num in player_numbers.iter() {
        if winning_numbers.contains(&player_num) {
            counter += 1;
        }
    }

    return counter;
}

fn main() {
    let result = get_result("./part2_input.txt");

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_result() {
        let result = get_result("./part2_test_input.txt");
        assert_eq!(result, 30);
    }

    #[test]
    fn my_test() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let numbers_string: &str = line.split(":").collect::<Vec<&str>>().last().unwrap();
        let splitted = numbers_string.split("|").collect::<Vec<&str>>();
        let winning_numbers: Vec<i32> = splitted
            .first()
            .unwrap()
            .split(" ")
            .filter(|&x| !x.is_empty())
            .map(|num_str: &str| num_str.parse::<i32>().unwrap())
            .collect();
        let player_numbers: Vec<i32> = splitted
            .last()
            .unwrap()
            .split(" ")
            .filter(|&x| !x.is_empty())
            .map(|num_str: &str| num_str.parse::<i32>().unwrap())
            .collect();

        println!("{:?}", winning_numbers);
        println!("{:?}", player_numbers);
    }
}
