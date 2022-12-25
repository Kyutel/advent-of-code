use std::{collections::HashMap};


static FILE_CONTENTS: &str = include_str!("input.txt");
// static FILE_CONTENTS: &str = include_str!("test-input.txt");

type Position = (usize, usize);

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_map() ->  Vec<Vec<char>>{
    FILE_CONTENTS.split('\n')
    .map(|x| x.chars().collect())
    .collect()
} 

fn _print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for height in row {
            print!("{}", height);
        }
        println!();
    }
}

fn get_start_and_end_positions(map: &Vec<Vec<char>>) -> (Position, Position) {
    let mut start_position = (0,0);
    let mut end_position = (0,0);

    for (x, row) in map.iter().enumerate() {
        for (y, height) in row.iter().enumerate() {
            if *height == 'S' {
                start_position = (x,y);
            }
            else if *height == 'E'{
                end_position =  (x,y);
            }
        }
    }

    (start_position, end_position)
}

fn get_next_possible_positions(current_position: Position, map: &Vec<Vec<char>>) -> Vec<Position> {
    let mut possible_positions = vec![];

    let current_value = map[current_position.0][current_position.1] as u8;


    // North
    if current_position.0 != 0 {
        let north = map[current_position.0 - 1][current_position.1] as u8;
        if north == current_value + 1 || north <= current_value {
            possible_positions.push((current_position.0 - 1, current_position.1))
        }
    } 
    
    // South
    if current_position.0 != map.len() - 1 {
        let south = map[current_position.0 + 1][current_position.1] as u8;
        if south == current_value + 1 || south <= current_value {
            possible_positions.push((current_position.0 + 1, current_position.1))
        }
    } 


    // East
    if current_position.1 != map[0].len() - 1 {
        let east = map[current_position.0][current_position.1 + 1] as u8;
        if east == current_value + 1 || east <= current_value {
            possible_positions.push((current_position.0, current_position.1 + 1))
        }
    }

    // West
    if current_position.1 != 0 {
        let west = map[current_position.0][current_position.1 - 1] as u8;
        if west == current_value + 1 || west <= current_value {
            possible_positions.push((current_position.0, current_position.1 - 1))
        }
    } 


    possible_positions
}

fn insert_into_search_queue(search_queue: &mut Vec<(Position, i32)>, position_to_insert: (Position,i32)) {
    let mut index_to_insert = 0;
    for item in search_queue.iter() {
        if item.0 == position_to_insert.0 {
            return;
        }
    }

    for (index, item) in search_queue.iter().enumerate() {
        if position_to_insert.0 < item.0 {
            index_to_insert = index;
            break;
        }
    }

    search_queue.insert(index_to_insert, position_to_insert);

}


fn get_shortest_path(start: Position, end: Position, map: &Vec<Vec<char>>)  -> i32{
    let mut search_queue = vec![(start, 0)];
    let mut distance_map = HashMap::new();

    distance_map.insert(start, 0);


    while search_queue.len() > 0 {
        let next = search_queue.pop().unwrap();
        let current_position = next.0;
        if current_position == end {
            return next.1;
        }

        let next_positions = get_next_possible_positions(current_position, &map);
        let current_value = next.1;

        for position in next_positions {
            let next_position_value = current_value + 1;
            let current_known = distance_map.entry(position).or_insert(next_position_value);
            if next_position_value <= *current_known {
                distance_map.insert(position, next_position_value);
                insert_into_search_queue(&mut search_queue, (position, next_position_value));
            }
            
            
        }
    }
    i32::MAX

}

fn part1()  -> i32 {
    let mut map = get_map();
    let (start, end) = get_start_and_end_positions(&map);
    map[start.0][start.1] = 'a';
    map[end.0][end.1] = 'z';


    return get_shortest_path(start, end, &map);

    // print_map(&map);
    // println!("S: {:?} \nE {:?}", start, end);
}

fn part2() -> i32{
    let mut map = get_map();
    let (start, end) = get_start_and_end_positions(&map);
    map[start.0][start.1] = 'a';
    map[end.0][end.1] = 'z';

    let mut starting_positions = vec![];
    for (x, row) in map.iter().enumerate() {
        for (y, height) in row.iter().enumerate() {
            if *height == 'a' {
                starting_positions.push((x,y));
            }
        }
    }

    let shortest_of_paths= starting_positions.iter()
    .map(|x| get_shortest_path(*x, end, &map))
    .min().unwrap();

    shortest_of_paths
}