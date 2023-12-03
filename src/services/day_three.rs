use std::fs;
use std::cmp::{min, max};
use std::collections::HashMap;

const FILE_PATH: &str = "src/services/inputs/day_three__input.txt";

/// * 1. get starting index + length of all part numbers
/// * 2. for each number from part 1,
/// *      a. check its perimeter in the engine
/// *      b. if it contains non-period symbol(s), ignore and continue
/// *      c. otherwise, add this number to the total sum
/// * 3. return total sum
/// * 
/// * 
/// * requirements
/// * - read text input and format it into 2d-array (each cell in matrix holds one character)
/// * - edge handling for perimeters (when on row 0, don't check row -1 b/c that's invalid indexing)
/// * 
/// * facts
/// * - different part numbers are separated by at least 1 period/symbol
pub fn sum_part_numbers() -> () {
    let input = fs::read(FILE_PATH).expect("SHOULD READ THE FILE JUST FINE");
    let parsed_input = parse_input(input);

    // vector of (row_num, offset, part #, part # length)
    let nums = get_num_info(&parsed_input);
    let mut sum: u32 = 0;

    // println!("{:#?}", nums);

    for num_info in nums {
        if num_touches_symbol(num_info, &parsed_input) {
            sum += num_info.2;
        }
    }

    println!("total parts sum: {}", sum);
    ()
}

fn get_num_info(matrix: &Vec<Vec<char>>) -> Vec<(usize, usize, u32, usize)> {
    let mut ret: Vec<(usize, usize, u32, usize)> = Vec::new();

    for (row_num, row) in matrix.iter().enumerate() {
        let mut buffer: String = String::new();

        for (i, cell) in row.iter().enumerate() {
            if cell.is_numeric() {
                buffer.push(*cell);

                if (i + 1 == row.len() || !row[i + 1].is_numeric()) && !buffer.is_empty() {
                    let num_info = (
                        row_num, 
                        i + 1 - buffer.len(), 
                        buffer.parse::<u32>().unwrap(), 
                        buffer.len(),
                    );
                    ret.push(num_info);
                    buffer.clear();
                }
            }
        }
    }

    // println!("{:#?}", ret);
    ret
}

fn num_touches_symbol(num: (usize, usize, u32, usize), matrix: &Vec<Vec<char>>) -> bool {
    let max_index = matrix.len() as i32;
    let x_min = max(num.1 as i32 - 1, 0);
    let x_max = min((num.1 + num.3) as i32 + 1, max_index);
    let x_range: Vec<i32> = (x_min..x_max).collect();

    let y_min = max(num.0 as i32 - 1, 0);
    let y_max = min(num.0 as i32 + 2, max_index);
    let y_range: Vec<i32> = (y_min..y_max).collect();
    
    for x in x_range {
        for (_, y) in y_range.iter().enumerate() {
            let val = matrix[*y as usize][x as usize];
            if val != '.' && !val.is_numeric() {
                // println!("{} at ({}, {}) is a part num! hit {}", num.2, num.0, num.1, val);
                return true;
            }
        }
    }

    false
}

fn parse_input(input: Vec<u8>) -> Vec<Vec<char>> {
    let mut ret: Vec<Vec<char>> = Vec::new();
    let mut row: Vec<char> = Vec::new();
    
    for byte in input {
        if byte == b'\n' {
            ret.push(row.clone());
            row.clear();
        } else {
            row.push(char::from(byte));
        }
    }

    ret.push(row.clone());
    ret
}

/// part 2 below

pub fn sum_gear_ratios() -> () {
    let input = fs::read(FILE_PATH).expect("SHOULD READ THE FILE JUST FINE");
    let parsed_input = parse_input(input);

    let mut gear_ratio_sum = 0;
    let nums = get_num_info(&parsed_input);
    let mut new_nums: HashMap<usize, Vec<(usize, u32, usize)>> = HashMap::new();

    for num in nums {
        match new_nums.get_mut(&num.0) {
            Some(vec) => {
                vec.push((num.1, num.2, num.3));
            },
            None => {
                new_nums.insert(num.0, vec![(num.1, num.2, num.3)]);
            },
        };
    }

    let gears = get_gear_info(&parsed_input);

    for gear_info in gears {
        gear_ratio_sum += get_gear_ratio(gear_info, &new_nums, &parsed_input);
    }
    
    println!("gear ratio sum: {}", gear_ratio_sum);
    // locate all numbers
    // locate all asterisks
    // for every asterisk at (i, j), iterate rows i - 1 to i + 1 inclusive (however, consider edge cases)
        // for every row, check all numbers
            // for every number, if j - 1 <= starting index <= j + 1 or j - 1 <= last digit <= j + 1, 
            // increment count by 1
        // if count == 2, calculate gear ratio (first num * second num). add gear ratio to sum
    // return sum
}

fn get_gear_ratio(gear_info: (usize, usize), nums: &HashMap<usize, Vec<(usize, u32, usize)>>, matrix: &Vec<Vec<char>>) -> u32 {
    let (row, offset) = gear_info;
    let max_index = matrix.len();

    let rows: Vec<usize> = (max(row - 1, 0)..min(row + 2, max_index)).collect();
    let offset_range = max(offset - 1, 0)..min(offset + 2, max_index);
    let mut count = 0;
    let mut gear_ratio = 1;

    for row in rows {
        let nums = nums.get(&row).unwrap();

        for num in nums {
            if offset_range.contains(&num.0) || offset_range.contains(&(num.0 + num.2 - 1)) {
                count += 1;
                gear_ratio *= num.1;
            }
        }
    }

    if count == 2 {
        return gear_ratio;
    }

    0
}

fn get_gear_info(matrix: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut ret: Vec<(usize, usize)> = Vec::new();

    for (row_num, row) in matrix.iter().enumerate() {
        for (i, cell) in row.iter().enumerate() {
            if *cell == '*' {
                ret.push((row_num, i));
            }
        }
    }

    ret
}