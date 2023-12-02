use std::cmp::max;
use std::fs;
// sum of the IDs of games that were possible, given that:
// maximum # reds: 12
// maximum # blues: 13
// maximum # greens: 14

/**
 * 
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

        only games 1, 2, 5 possible; 1 + 2 + 5 = 8
 */

const FILE_PATH: &str = "src/services/inputs/day_two__input.txt";
const MAX_REDS: u32 = 12;
const MAX_GREENS: u32 = 13;
const MAX_BLUES: u32 = 14;

pub fn sum_game_ids() -> () {
    let input = fs::read_to_string(FILE_PATH)
        .expect("SHOULDVE READ THE FILE JUST FINE");

    let mut sum: usize = 0;

    for (i, game_line) in input.split("\n").enumerate() {
        println!("Game #{}:\n", i + 1);
        if is_game_possible(game_line) {
            println!("\t possible!");
            sum += i + 1;
        }
    }

    println!("sum: {}", sum);
}

pub fn sum_min_set_powers() -> () {
    let input = fs::read_to_string(FILE_PATH)
        .expect("SHOULDVE READ THE FILE JUST FINE");

    let mut sum: u32 = 0;

    for (i, game_line) in input.split("\n").enumerate() {
        println!("Game #{}:\n", i + 1);
        sum += get_power(game_line);
    }

    println!("sum of minimum set powers: {}", sum);
}

fn get_power(game_line: &str) -> u32 {
    let game = game_line.split(": ").last().unwrap();
    let subsets: Vec<&str> = game.split("; ").collect(); // ["3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"]
    let (mut min_red, mut min_blue, mut min_green) = (0, 0, 0);

    for subset in subsets {
        let counts: (u32, u32, u32) = parse_subset_count(subset);

        min_red = max(min_red, counts.0);
        min_blue = max(min_blue, counts.1);
        min_green = max(min_green, counts.2);
    }

    println!("\t min set: {} reds, {} blues, {} greens", min_red, min_blue, min_green);
    
    min_red * min_blue * min_green
}

fn is_game_possible(game_line: &str) -> bool {
    let game = game_line.split(": ").last().unwrap();
    let subsets: Vec<&str> = game.split("; ").collect();

    for subset in subsets {
        let counts: (u32, u32, u32) = parse_subset_count(subset);

        if counts.0 > MAX_REDS || counts.1 > MAX_BLUES || counts.2 > MAX_GREENS {
            println!("COUNTS: {} {} {}", counts.0, counts.1, counts.2);
            return false;
        }
    }

    true
}

fn parse_subset_count(subset: &str) -> (u32, u32, u32) {
    let counts: Vec<&str> = subset.split(", ").collect(); // ["1 blue", "1 red"]
    let (mut red, mut blue, mut green) = (0, 0, 0);

    for count in counts {
        let info: Vec<&str> = count.split(" ").collect();
        let value = String::from(*info.get(0).unwrap()).parse::<u32>().unwrap();

        match info.get(1) {
            Some(&"red") => red = value,
            Some(&"blue") => blue = value,
            Some(&"green") => green = value,
            _ => panic!("BRUHHHH"),
        }
    }

    (red, blue, green)
}