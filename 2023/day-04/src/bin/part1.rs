use std::fs;

fn get_result(file_name: &str) -> i32 {
    let read_lines = fs::read_to_string(file_name).unwrap();

    let mut result = 0;

    for line in read_lines.lines() {
        let line_result = get_result_for_line(line);
        result += line_result.points;
    }

    return result;
}

struct CardResult {
    winning_numbers: Vec<i32>,
    player_numbers: Vec<i32>,
    points: i32,
}

fn get_result_for_line(line: &str) -> CardResult {
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

    let mut points = 0;

    for &player_num in player_numbers.iter() {
        if winning_numbers.contains(&player_num) {
            points = std::cmp::max(points * 2, 1);
        }
    }

    return CardResult {
        winning_numbers,
        player_numbers,
        points,
    };
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
        assert_eq!(result, 13);
    }

    #[test]
    fn test_get_numbers() {
        let result = get_result_for_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(result.winning_numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(result.player_numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
        assert_eq!(result.points, 8);
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
