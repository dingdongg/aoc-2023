use std::{fs, u32};

const FILE_PATH: &str = "src/services/day_one__input.txt";

pub fn get_calibration_sum() -> u32 {
    let input = fs::read_to_string(FILE_PATH)
        .expect("SHOULDVE READ THE FILE JUST FINE");

    let mut sum: u32 = 0;

    for line in input.split("\n") {
        let line_sum = get_line_sum(line);
        println!("line sum: {}", line_sum);
        sum += line_sum;
    }

    println!("THe sum is {}", sum);
    sum
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