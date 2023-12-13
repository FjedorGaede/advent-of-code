use std::fs;

fn get_result(file_name: &str) -> i32 {
    let read_lines = fs::read_to_string(file_name).unwrap();

    let mut result = 0;

    let lines_iter = read_lines.lines();

    return result;
}

fn main() {
    let games: Vec<Game> = vec![
        Game {
            time: 60,
            distance: 601,
        },
        Game {
            time: 80,
            distance: 1163,
        },
        Game {
            time: 86,
            distance: 1559,
        },
        Game {
            time: 76,
            distance: 1300,
        },
    ];

    let result: u32 = games
        .iter()
        .map(|game| game.get_number_of_possible_wins())
        .fold(1, |acc, x| acc * x);
    println!("result: {}", result);
}

struct Game {
    time: u32,
    distance: u32,
}

impl Game {
    fn get_number_of_possible_wins(&self) -> u32 {
        // Calculate the interval
        //D d, T t -> l t halten -> l d/t -> (T-l)*l = Tl - l^2 >= D
        // -l^2 + lT + D < 0
        // x_1 = 1/2(m - sqrt(m^2 + 4n))
        // x_2 = 1/2(m + sqrt(m^2 + 4n))
        // So interval of valid is (x_1, x_2)
        let radicand: f64 = (self.time * self.time - 4 * self.distance) as f64;
        let f_time = self.time as f64;
        let root = radicand.sqrt();
        let lower_bound = 0.5 * (f_time - root);
        let upper_bound = 0.5 * (f_time + root);

        // have to offset here a little as we want to be in the inner of the interval not on the
        // bounaries itself
        return ((upper_bound - 0.01).floor() - (lower_bound + 0.01).ceil()) as u32 + 1;
    }
}

#[test]
fn test_get_result() {
    let result = get_result("./part1_test_input.txt");
    assert_eq!(result, 13);
}

#[test]
fn test_game() {
    let games: Vec<Game> = vec![
        Game {
            time: 7,
            distance: 9,
        },
        Game {
            time: 15,
            distance: 40,
        },
        Game {
            time: 30,
            distance: 200,
        },
    ];

    let result: u32 = games
        .iter()
        .map(|game| game.get_number_of_possible_wins())
        .fold(1, |acc, x| acc * x);
    println!("result: {}", result);
}
