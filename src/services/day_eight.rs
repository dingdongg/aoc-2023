use core::panic;
use std::fs;
use std::collections::HashMap;

const FILE_PATH: &str = "src/services/inputs/day_eight__input.txt";

enum Direction {
    Left,
    Right,
}

struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn get_next_node(&self, next_step: &Direction) -> &str {
        match next_step {
            Direction::Left => self.left.as_str(),
            Direction::Right => self.right.as_str(),
        }
    }
}

pub fn get_map_steps() -> () { 
    let input = fs::read_to_string(FILE_PATH).expect("SHOULDVE READ FILE JUST FINE");
    let splitted_input: Vec<&str> = input.split("\n\n").collect();

    let sequence = parse_sequence(splitted_input[0]);
    let sequence_leng = sequence.len();
    let network = parse_network(splitted_input[1]);

    let mut num_steps = 0;
    let mut i: usize = 0;
    let mut curr_node_key = "AAA";

    while curr_node_key != "ZZZ" {
        let curr_node = network.get(curr_node_key).unwrap();
        let next_step = &sequence[i];

        curr_node_key = curr_node.get_next_node(&next_step);
        i = (i + 1) % sequence_leng;
        num_steps += 1;
    }

    println!("TOTAL STEPS TAKEN: {num_steps}");
}

fn parse_sequence(input: &str) -> Vec<Direction> {
    let mut ret = Vec::new();

    for b in input.as_bytes() {
        ret.push(if *b == b'L' { Direction::Left } else { Direction::Right });
    }

    ret
}

fn parse_network(input: &str) -> HashMap<&str, Node> {
    let lines: Vec<Vec<&str>> = input
        .split("\n")
        .map(|l| l.split([' ', '=', '(', ')', ',']).filter(|&t| t != "").collect::<Vec<&str>>())
        .collect();

    let mut network = HashMap::new();

    for line in lines {
        let node = Node {
            name: String::from(line[0]),
            left: String::from(line[1]),
            right: String::from(line[2]),
        };

        network.insert(line[0], node);
    }

    network
}
