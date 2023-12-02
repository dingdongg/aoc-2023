use std::{fs, u32};
use std::collections::HashMap;

const FILE_PATH: &str = "src/services/inputs/day_one__input.txt";
const STRINGS: [&str;18] = [
    "one", "two", "three", 
    "four", "five", "six", 
    "seven", "eight", "nine",
    "1", "2", "3", "4", "5", "6", "7", "8", "9",
];

pub fn get_calibration_sum() -> u32 {
    let input = fs::read_to_string(FILE_PATH)
        .expect("SHOULDVE READ THE FILE JUST FINE");

    let mut sum: u32 = 0;
    let mut num_mappings: HashMap<String, char> = HashMap::new();
    populate_mappings(&mut num_mappings);

    for line in input.split("\n") {
        // let line_sum = get_line_sum(line);
        let line_sum = get_line_sum_new(line, &num_mappings);
        println!("line: {}; sum: {}", line, line_sum);
        sum += line_sum;
    }

    println!("THe sum is {}", sum);
    sum
}

fn populate_mappings(m: &mut HashMap<String, char>) {
    for (i, s) in STRINGS.iter().enumerate() {
        let deref = *s;
        m.insert(String::from(deref), char::from_digit(((i as u32) % 9) + 1, 10).unwrap());
    }
}

fn get_line_sum(input: &str) -> u32 {
    let mut num_chars: Vec<char> = Vec::new();

    for char in input.chars() {
        if char.is_numeric() {
            num_chars.push(char);
        }
    }

    let mut ret = num_chars[0].to_string().to_owned();
    let second_num = num_chars[num_chars.len() - 1].to_string();
    ret.push_str(&second_num);

    ret.parse::<u32>().unwrap()
}

fn get_line_sum_new(input: &str, mapping: &HashMap<String, char>) -> u32 {
    let mut min_index = input.len();
    let mut max_index = 0;

    let mut min_val: char = ' ';
    let mut max_val: char = ' ';
    let mut encounters: i32 = 0;

    for str in STRINGS {
        let idx: Vec<usize> = input.match_indices(str).map(|t| t.0).collect();
        if idx.len() > 0 {
            let first = idx.get(0);
            let last = idx.get(idx.len() - 1);

            if first.is_some_and(|f| f < &min_index) {
                min_index = *first.unwrap();
                if str.len() > 1 {
                    min_val = *mapping.get(str).unwrap();
                } else {
                    min_val = str.chars().next().unwrap();
                }
            }

            if last.is_some_and(|l| l > &max_index) {
                max_index = *last.unwrap();
                if str.len() > 1 {
                    max_val = *mapping.get(str).unwrap();
                } else {
                    max_val = str.chars().next().unwrap();
                }
            }

            encounters += 1;
        }
    }

    let mut ret = String::new();

    ret.push(min_val);
    if encounters == 1 {
        ret.push(min_val);
    } else {
        ret.push(max_val);
    }

    ret.parse::<u32>().unwrap_or(0)
}