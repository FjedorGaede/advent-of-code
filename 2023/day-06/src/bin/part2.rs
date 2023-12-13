use std::fs;

fn main() {
    let game = Game {
        time: 60808676.0,
        distance: 601116315591300.0,
    };

    println!("result: {}", game.get_number_of_possible_wins());
}

struct Game {
    time: f64,
    distance: f64,
}

impl Game {
    fn get_number_of_possible_wins(&self) -> u32 {
        // Calculate the interval
        //D d, T t -> l t halten -> l d/t -> (T-l)*l = Tl - l^2 >= D
        // -l^2 + lT + D < 0
        // x_1 = 1/2(m - sqrt(m^2 + 4n))
        // x_2 = 1/2(m + sqrt(m^2 + 4n))
        // So interval of valid is (x_1, x_2)
        let radicand: f64 = self.time * self.time - 4.0 * self.distance;
        let root = radicand.sqrt();
        let lower_bound = 0.5 * (self.time - root);
        let upper_bound = 0.5 * (self.time + root);

        println!("lower_bound {}", lower_bound);
        println!("upper_bound {}", upper_bound);

        // have to offset here a little as we want to be in the inner of the interval not on the
        // bounaries itself
        return ((upper_bound - 0.01).floor() - (lower_bound + 0.01).ceil()) as u32 + 1;
    }
}

#[test]
fn test_game_2() {
    let game = Game {
        time: 71530.0,
        distance: 940200.0,
    };

    println!("result: {}", game.get_number_of_possible_wins());
}
