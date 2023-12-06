use std::{fs, ops::Add};

const FILE_PATH: &str = "src/services/inputs/day_six__input.txt";

#[derive(Debug)]
struct Race {
    time: i32,
    distance: i32,
}

pub fn count_record_beating_strats() -> () {
    let input: String = fs::read_to_string(FILE_PATH)
        .expect("SHOULDVE READ THE FILE FINE");

    let lines: Vec<&str> = input.split("\n").collect();

    let times = parse_times(&lines[0]);
    let distances = parse_distances(&lines[1]);
    let mut races = Vec::new();
    let len = times.len();

    for i in 0..len {
        races.push(Race { time: times[i], distance: distances[i] });
    }

    let mut num_ways = 1;

    for (i, race) in races.iter().enumerate() {
        let mut hold_time = 0;
        let mut total_num_ways = race.time + 1;

        while hold_time < race.time {
            let travel_time = race.time - hold_time;
            let total_distance = hold_time * travel_time;
            if total_distance <= race.distance {
                hold_time += 1;
                total_num_ways -= 2;
            } else {
                // include current race (and its corresponding pair, if any)
                break;
            }
        }

        println!("{:#?}: ways = {}", race, total_num_ways);
        num_ways *= total_num_ways;
    }

    println!("TOTAL # WYS TO BEAT ALL RECORDS: {}", num_ways);

    // println!("TOTAL # WAYS TO BEAT ALL RECORDS: {:#?}", input);
}

fn parse_times(times: &str) -> Vec<i32> {
    let tokens = times.split(" ").into_iter();
    let mut ret = Vec::new();

    for t in tokens {
        if let Ok(n) = t.parse::<i32>() {
            ret.push(n);
        }
    }

    ret
}

fn parse_distances(distances: &str) -> Vec<i32> {
    let tokens = distances.split(" ").into_iter();
    let mut ret = Vec::new();

    for t in tokens {
        if let Ok(n) = t.parse::<i32>() {
            ret.push(n);
        }
    }
    
    ret
}

/// part 2

pub fn solve() -> () {
    let input: String = fs::read_to_string(FILE_PATH)
        .expect("SHOULDVE READ THE FILE FINE");

    let lines: Vec<&str> = input.split("\n").collect();

    let time = parse_bad_kerning_time(&lines[0]);
    let distance = parse_bad_kerning_distance(&lines[1]);
    let race = (time, distance);

    let mut hold_time = 0;
    let mut total_ways = race.0 + 1;

    while hold_time < race.0 {
        let travel_time = race.0 - hold_time;
        let total_distance = hold_time * travel_time;
        if total_distance <= race.1 {
            hold_time += 1;
            total_ways -= 2;
        } else {
            break;
        }
    }

    println!("TOTAL WAYS TO BEAT THIS RACE: {}", total_ways)
}

fn parse_bad_kerning_time(time: &str) -> u64 {
    let mut ret: String = String::new();

    for byte in time.as_bytes() {
        if byte.is_ascii_digit() {
            ret.push(char::from(*byte));
        }
    }
    
    ret.parse::<u64>().unwrap()
}

fn parse_bad_kerning_distance(distance: &str) -> u64 {
    let mut ret: String = String::new();

    for byte in distance.as_bytes() {
        if byte.is_ascii_digit() {
            ret.push(char::from(*byte));
        }
    }
    
    ret.parse::<u64>().unwrap()
}