use std::collections::HashSet;
use std::fs;
use std::cmp::min;

const FILE_PATH: &str = "src/services/inputs/day_four__input.txt";

// part 1
/// 1. read as string
/// 2. split on newlines
/// 3. remove prefix
/// 4. split on "|"
/// 5. take left half of 4) and turn it into a set
/// 6. for every number n[i] on right half of 4), 
///     a. if it exists in the set from 5), add 2^i to points sum variable
/// 7. return points sum

pub fn sum_card_points() -> () {
    let input = fs::read_to_string(FILE_PATH)
        .expect("SHOULDVE READ THE FILE JUST FINE");

    let cards: Vec<&str> = input
        .split("\n")
        .into_iter()
        .map(|c| c.split(": ").nth(1).unwrap())
        .collect();

    let mut sum = 0;
    let mut w_nums_set: HashSet<i32>;

    for card in &cards {
        let mut split_card = card.split(" | ");
        w_nums_set = HashSet::from_iter(
            parse_numbers(split_card.next().unwrap()),
        );
        let my_nums: Vec<i32> = parse_numbers(split_card.next().unwrap());
        let mut found_count: u32 = 0;

        for n in my_nums {
            if w_nums_set.contains(&n) {
                println!("\t set contains {}", n);
                found_count += 1;
            }
        }

        if found_count > 0 {
            sum += i32::pow(2, found_count - 1);
        }
    }
    
    println!("TOTAL SUM: {}", sum);

}

// numbers are all double- or single-digit strings separated by " "
// length of string = 2 * n + (n - 1)
//                  = 3n - 1

// n = (length of string + 1) / 3
// ex) 34 12 64, starting indexes are 0, 3, 6
fn parse_numbers(data: &str) -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();
    let n = data.len();
    let mut i = 0;

    while i < n {
        let num: &str;

        if &data[i..i + 1] == " " {
            num = &data[i + 1..i + 2];
        } else {
            num = &data[i..i + 2];
        }

        ret.push(num.parse::<i32>().unwrap());
        i += 3;
    }

    // println!("'{}' => {:#?}", data, ret);
    ret
}

/// part 2

pub fn count_all_scratch_cards() -> () {
    let input = fs::read_to_string(FILE_PATH)
        .expect("SHOULDVE READ THE FILE JUST FINE");

    let cards: Vec<&str> = input
        .split("\n")
        .into_iter()
        .map(|c| c.split(": ").nth(1).unwrap())
        .collect();
    
    let mut card_counts: Vec<i32> = vec![1; cards.len()];
    let mut w_nums_set: HashSet<i32>;

    for (i, card) in cards.iter().enumerate() {
        /// card i
        /// 1. get the card's matching number count (2)
        /// 2. get card i's current card count (4)
        /// 3. add card i's card count (4) to the next (2) numbers
        
        let matching_count = get_matching_count(card);
        let curr_card_count = card_counts[i];
        let copy_range = i + 1..min(i + 1 + matching_count as usize, cards.len());
        
        println!(
            "matching count: {}; current card count for idx {}: {}; range: {:#?}", 
            matching_count,
            i,
            curr_card_count,
            copy_range,
        );
        copy_range.for_each(|j| card_counts[j] += curr_card_count);
    }

    println!("TOTAL # SCRATCH CARDS: {}", card_counts.iter().sum::<i32>());
}

fn get_matching_count(card: &str) -> i32 {
    let mut split_card = card.split(" | ");
    let w_nums_set: HashSet<i32> = HashSet::from_iter(
        parse_numbers(split_card.next().unwrap()),
    );
    let my_nums: Vec<i32> = parse_numbers(split_card.next().unwrap());
    let mut found_count: i32 = 0;

    for n in my_nums {
        if w_nums_set.contains(&n) {
            // println!("\t set contains {}", n);
            found_count += 1;
        }
    }

    found_count
}