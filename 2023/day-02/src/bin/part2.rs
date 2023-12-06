use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<SetOfCubes>,
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.sets.iter().all(|item| other.sets.contains(item)) && self.id == other.id
    }
}

#[derive(Debug)]
struct SetOfCubes {
    red: i32,
    green: i32,
    blue: i32,
}

impl PartialEq for SetOfCubes {
    fn eq(&self, other: &Self) -> bool {
        return self.green == other.green && self.red == other.red && self.blue == other.blue;
    }
}

impl SetOfCubes {
    fn update(&mut self, color: &str, update_val: i32) {
        if color == "green" {
            self.green = update_val;
        } else if color == "red" {
            self.red = update_val;
        } else if color == "blue" {
            self.blue = update_val;
        }
    }

    fn is_possible(&self, other: &SetOfCubes) -> bool {
        self.green <= other.green && self.red <= other.red && self.blue <= other.blue
    }

    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

fn get_highest_color(sets: &Vec<SetOfCubes>) -> SetOfCubes {
    let mut result_set = SetOfCubes {
        red: 0,
        green: 0,
        blue: 0,
    };

    for set in sets.into_iter() {
        if set.red > result_set.red {
            result_set.update("red", set.red);
        }
        if set.green > result_set.green {
            result_set.update("green", set.green);
        }
        if set.blue > result_set.blue {
            result_set.update("blue", set.blue);
        }
    }

    result_set
}

fn get_result(file_name: &str) -> i32 {
    let lines_iterator = fs::read_to_string(file_name).unwrap();

    let mut powers: Vec<i32> = vec![];

    for line in lines_iterator.lines() {
        let game = parse_game(line);

        let highest_color_set = get_highest_color(&game.sets);

        powers.push(highest_color_set.power());
    }

    return powers
        .into_iter()
        .reduce(|prev, curr| prev + curr)
        .unwrap_or(0);
}

fn parse_game(game: &str) -> Game {
    let splitted: Vec<&str> = game.split(":").collect();
    let game_id_string_part = splitted.first().unwrap();
    let game_id = Regex::new(r"\d+")
        .unwrap()
        .find(game_id_string_part)
        .map(|m| m.as_str())
        .unwrap_or("")
        .parse::<i32>()
        .unwrap_or(0);

    let game_round_strings: Vec<&str> = splitted.last().unwrap().split(";").collect();
    let mut game_sets: Vec<SetOfCubes> = vec![];

    for s in game_round_strings.iter() {
        let mut set_of_cubses = SetOfCubes {
            red: 0,
            blue: 0,
            green: 0,
        };
        for color in vec!["green", "blue", "red"] {
            let val = Regex::new(format!(r"(\d+)\s+{}", color).as_str())
                .unwrap()
                .captures(s)
                .map(|m| m.get(1).unwrap().as_str())
                .unwrap_or("0")
                .to_string();
            set_of_cubses.update(color, val.parse::<i32>().unwrap_or(0));
        }

        game_sets.push(set_of_cubses);
    }
    return Game {
        id: game_id,
        sets: game_sets,
    };
}

fn main() {
    let result = get_result("./part2_input.txt");

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_from_game() {
        let result = parse_game(" Game  1 : 3 blue, 4 red; 1 red, 2 green, 6 blue; 3 green    ");
        let expected_game = Game {
            id: 1,
            sets: vec![
                SetOfCubes {
                    blue: 3,
                    red: 4,
                    green: 0,
                },
                SetOfCubes {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                SetOfCubes {
                    green: 3,
                    red: 0,
                    blue: 0,
                },
            ],
        };

        assert_eq!(result, expected_game);
    }

    #[test]
    fn test_get_result() {
        let given = SetOfCubes {
            red: 12,
            green: 13,
            blue: 14,
        };
        let result = get_result("./part2_test_input.txt", given);
        assert_eq!(result, 2286);
    }
}
