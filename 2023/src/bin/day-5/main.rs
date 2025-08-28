use std::cmp::min;

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

// enum MapKind {
//     SEED_TO_SOIL,
// }

#[derive(Debug)]
struct Map {
    dest_start: i64,
    src_start: i64,
    range: i64
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_maps(mapping: &str) -> Vec<Map> {
    mapping.split('\n').skip(1).map(|line| {
        let p:Vec<i64> = line.split(" ").map(|x| x.parse().unwrap()).collect(); 
        Map{dest_start: p[0], src_start: p[1], range: p[2] }
    }).collect()
}

fn find_location(seed: i64, mappings: &Vec<Vec<Map>>) -> i64{
    let mut current_val = seed;
    for map_list in mappings {
        let mut next_val = current_val;
        for map in map_list {
            if current_val >= map.src_start && current_val <= map.src_start + map.range {
                next_val = map.dest_start + ((current_val - map.src_start).abs());
                break;
            }
        }
        // if seed == 82 {
        //     print!("{}, ", next_val);
        //     if current_val == 77 {
        //         print!("CV {}, ", current_val);
        //     }
        // }
        current_val = next_val;
        // println!("{current_val}")
    }
    // println!();
    current_val
}

fn part1 ()  -> i64 {

    let split_file:Vec<&str> = FILE_CONTENTS.split("\n\n").collect();
    let seeds: Vec<i64> = split_file[0][7..].split(" ").map(|x| x.parse().unwrap()).collect();
    let maps: Vec<Vec<Map>> = split_file[1..].into_iter().map(|mapping |get_maps(mapping)).collect();

    seeds.into_iter().map(|seed| find_location(seed, &maps)).min().unwrap()
}

fn part2 () -> i64 {
    let split_file:Vec<&str> = FILE_CONTENTS.split("\n\n").collect();
    let seeds: Vec<i64> = split_file[0][7..].split(" ").map(|x| x.parse().unwrap()).collect();
    let maps: Vec<Vec<Map>> = split_file[1..].into_iter().map(|mapping: &&str |get_maps(mapping)).collect();

    // seeds.into_iter().enumerate().map(|(seed)| find_location(seed, &maps)).min().unwrap()
    let mut smallest = std::i64::MAX;
    for i in (0..seeds.len()).step_by(2) {
        for seed in seeds[i]..seeds[i]+seeds[i+1]+1{
            // if seed == 82 {
            //     print!("hey");
            //     println!( "{:?}", find_location(seed, &maps))

            // }
            smallest = min(smallest, find_location(seed, &maps));
        }

    }
    smallest
}
