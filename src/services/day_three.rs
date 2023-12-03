use std::fs;
use std::cmp::{min, max};

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