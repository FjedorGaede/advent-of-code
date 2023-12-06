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
}

fn is_possible(game: &Game, given_cube_set: &SetOfCubes) -> bool {
    game.sets
        .iter()
        .all(|game_set| game_set.is_possible(&given_cube_set))
}

fn get_result(file_name: &str, contained_in_bag: SetOfCubes) -> i32 {
    let lines_iterator = fs::read_to_string(file_name).unwrap();

    let mut possible_game_ids = vec![];

    for line in lines_iterator.lines() {
        let game = parse_game(line);

        if is_possible(&game, &contained_in_bag) {
            possible_game_ids.push(game.id);
        }
    }

    return possible_game_ids
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
    let given = SetOfCubes {
        red: 12,
        green: 13,
        blue: 14,
    };
    let result = get_result("./part1_input.txt", given);

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
    fn test_is_possible() {
        let set_to_test = SetOfCubes {
            red: 1,
            green: 2,
            blue: 3,
        };

        let game_set1 = SetOfCubes {
            red: 0,
            green: 0,
            blue: 0,
        };
        let game_set2 = SetOfCubes {
            red: 1,
            green: 2,
            blue: 3,
        };
        let game = Game {
            id: 1,
            sets: vec![game_set1, game_set2],
        };

        assert_eq!(is_possible(&game, &set_to_test), true);

        let game_set3 = SetOfCubes {
            red: 0,
            blue: 4,
            green: 0,
        };
        let game_set4 = SetOfCubes {
            red: 20,
            blue: 2,
            green: 3,
        };
        let game2 = Game {
            id: 3,
            sets: vec![game_set3, game_set4],
        };
        assert_eq!(is_possible(&game2, &set_to_test), false);
    }

    #[test]
    fn test_get_result() {
        let given = SetOfCubes {
            red: 12,
            green: 13,
            blue: 14,
        };
        let result = get_result("./part1_test_input.txt", given);
        assert_eq!(result, 8);
    }
}
