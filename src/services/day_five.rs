use std::{fs, str::Split};
use std::collections::{HashMap};
use std::fmt;
use std::cmp::{min, max};

const FILE_PATH: &str = "src/services/inputs/day_five__input.txt";

/// part 1

/// 1. split based on two newlines (\n\n)
/// 2. split first item from step 1 on ": "
///     a. split second item from step 2 on " "
///     b. convert each string from step 2a to u64
/// 3. for every other item at index i,
///     a. split on "\n"
///     b. remove first item from step 3a
///     c. for every remaining item from step 3b, split on " "
///     d. convert results from 3c to u64
///     e. use results from 3d to populate a HashMap with key = vec.1, value = { offset: vec.2, dest_start: vec.0 }

/// 4. for every seed (s) from step 2,
///     a. next_id = s
///     b. for every map in order of MAP_ORDER,
///         a. for every key (k) in that map,
///             a. if next_id >= k and next_id < k + map[k].offset,
///                 a. next_id = map[k].dest_start + (next_id - k)
///         b. otherwise (if no match found), leave next_id as is
///     c. set lowest_location to min(lowest_location, next_id)
/// 5. return lowest_location

#[derive(Debug)]
struct MapInfo {
    offset: u64,
    dest_start: u64,
}

#[derive(Debug)]

struct MapRange {
    start: u64,
    end: u64,
}

impl fmt::Display for MapRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {})", self.start, self.end)
    }
}

struct DetailedMapRange {
    in_start: u64,
    in_end: u64,
    out_start: u64,
    out_end: u64,
}

impl fmt::Display for DetailedMapRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}) -> [{}, {})", self.in_start, self.in_end, self.out_start, self.out_end)
    }
}

pub fn get_lowest_location() -> () {
    let input = fs::read_to_string(FILE_PATH)
        .expect("SHOULDVE READ THE FILE");

    let mut parsed_input = input.split("\n\n").into_iter();
    let seeds: Vec<u64> = parse_seeds(&mut parsed_input); // part 1
    let map_vec: Vec<HashMap<u64, MapInfo>> = parse_maps(&mut parsed_input);
    let mut min_location: u64 = u64::MAX;

    for seed in seeds {
        let mut id = seed;
        println!("seed: {seed}");
        for map in &map_vec {
            for key in map.keys() {
                let val = map.get(key).unwrap();
                if (id >= *key) && (id < key + val.offset) {
                    println!("key: {key}, val: {:#?}", val);
                    id = val.dest_start + (id - key);
                    break;
                }
            }
            println!("next id: {id}");
        }
        // break;

        println!("current location: {id}");

        min_location = std::cmp::min(min_location, id);
    }

    println!("MIN LOCATION {min_location}");
}

fn parse_maps(input: &mut Split<'_, &str>) -> Vec<HashMap<u64, MapInfo>> {
    let mut map_vec: Vec<HashMap<u64, MapInfo>> = Vec::new();

    for map_string in input {
        let lines: Vec<Vec<&str>> = map_string
            .split("\n")
            .skip(1)
            .map(|l| l.split(" ").collect())
            .collect();

        let mut map: HashMap<u64, MapInfo> = HashMap::new();
        let mut min_input_value = u64::MAX;

        for line in lines {
            let key = line[1].parse::<u64>().unwrap();
            let value = MapInfo {
                offset: line[2].parse::<u64>().unwrap(),
                dest_start: line[0].parse::<u64>().unwrap(),
            };

            map.insert(key, value);
            min_input_value = min(min_input_value, key);
        }
        // println!("{:#?}", map);

        if min_input_value != 0 {
            map.insert(
                0,
                MapInfo {
                    dest_start: 0,
                    offset: min_input_value,
                },
            );
        }

        map_vec.push(map);
    }

    map_vec
}


fn parse_seeds(input_iter: &mut Split<'_, &str>) -> Vec<u64> {
    input_iter
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

/// part 2

/// 1. calculate seed ranges
/// 2. for each map,
///     a. calculate its different INPUT ranges (eg. for seed->soil, seed ranges)
///     b. calculate its different OUTPUT ranges (eg. for seed-> soil, soil ranges)
/// 3. for seed_range in seed ranges,
///     0. next_ranges = [seed_range]
///     a. for each map,
///         0. temp_ranges = []
///         a. for each range in next_ranges,
///             a. calculate the overlapping input ranges with map 
///                 (range intersection b/w seed range and input range)
///             b. use the above result to calculate resulting output ranges
///             c. temp_ranges = [results from 3.a.a.b]
///         b. next_ranges = temp_ranges
///     b. min_location = min(
///         min_location, 
///         <result of linear search thru next_ranges to get range w/ smallest start>,
///        )
/// 4. return min_location
/// 
/// 

const MAP_OUTPUTS: [&str;7] = [
    "soil", "fertilizer", "water", "light", "temperature", "humidity", "location",
];

pub fn get_lowest_location_from_ranges() -> () {
    let input = fs::read_to_string(FILE_PATH)
        .expect("SHOULDVE READ THE FILE");

    let mut parsed_input = input.split("\n\n").into_iter();
    let seeds: Vec<MapRange> = parse_seed_ranges(&mut parsed_input);
    let map_vec: Vec<HashMap<u64, MapInfo>> = parse_maps(&mut parsed_input);
    let mut min_location: u64 = u64::MAX;

    let mut ranges = Vec::new();

    for map in &map_vec {
        for key in map.keys() {
            let val = map.get(key).unwrap();
            let map_range = DetailedMapRange {
                in_start: *key,
                in_end: key + val.offset,
                out_start: val.dest_start,
                out_end: val.dest_start + val.offset,
            };
            ranges.push(map_range);
        }
        ranges.sort_by(|a, b| a.in_start.cmp(&b.in_start));
        for r in &ranges {
            println!("{r}");
        }
        println!("\n---\n");
        ranges.clear();
    }

    for seed_range in seeds {
        let mut next_ranges = vec![seed_range];
        println!("SEED RANGE: [{}, {})", next_ranges[0].start, next_ranges[0].end);
        for (i, map) in (&map_vec).iter().enumerate() {
            println!("map output: {} --------------------------", MAP_OUTPUTS[i]);
            let mut temp_ranges = Vec::new();

            for range in next_ranges {
                temp_ranges.append(&mut get_overlapping_ranges(range, &map));
            }

            next_ranges = temp_ranges;
        }

        min_location = min(
            min_location, 
            next_ranges.iter().min_by(
                |r1, r2| r1.start.cmp(&r2.start),
            ).unwrap().start,
        );
    }

    println!("MIN LOCATION {min_location}");
}

fn get_overlapping_ranges(range: MapRange, map: &HashMap<u64, MapInfo>) -> Vec<MapRange> {
    let mut br: Vec<&u64> = map.keys().collect();
    br.sort();

    let mut r = 0;
    let MapRange { start, end } = range;
    let mut ret = Vec::new();

    while r < br.len() {
        let key = br[r];
        let val = map.get(key).unwrap();

        if ranges_intersect(&range, &MapRange { start: *key, end: key + val.offset }) {
            // println!("{range}");
            // println!("start + dest_start = {} ; key = {}", start + val.dest_start, key);
            // println!("end + dest_start = {} ; key = {}", end + val.dest_start, key);

            let mut offset_range = MapRange {
                start: val.dest_start,
                end: end + val.dest_start - key,
            };

            let curr_out_range = MapRange {
                start: val.dest_start,
                end: val.dest_start + val.offset,
            };

            if start + val.dest_start >= *key { offset_range.start = start + val.dest_start - key; } 
            ret.push(get_range_intersection(offset_range, curr_out_range));
        }

        r += 1;
    }

    if ret.len() == 0 { ret.push(range) };
    println!("{:#?}", ret);
    ret
}

fn ranges_intersect(r1: &MapRange, r2: &MapRange) -> bool {
    r1.end > r2.start && r2.end > r1.start
}

fn get_range_intersection(r1: MapRange, r2: MapRange) -> MapRange {
    MapRange {
        start: max(r1.start, r2.start),
        end: min(r1.end, r2.end),
    }
}

fn parse_seed_ranges(input_iter: &mut Split<'_, &str>) -> Vec<MapRange> {
    let nums: Vec<u64> = input_iter
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let range = 1..nums.len();
    let mut ret: Vec<MapRange> = Vec::new();

    for i in range.step_by(2) {
        let seed_range = MapRange {
            start: nums[i - 1],
            end: nums[i - 1] + nums[i],
        };
        
        ret.push(seed_range);
    }

    ret
}