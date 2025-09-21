use std::{collections::HashSet, pin::Pin, vec};

// currently only works when map is a square

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
// static FILE_CONTENTS: &str = include_str!("test-input2.txt");
// static FILE_CONTENTS: &str = include_str!("test-input3.txt");
// static FILE_CONTENTS: &str = include_str!("test-input4.txt");
// static FILE_CONTENTS: &str = include_str!("test-input5.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

type Map = Vec<Vec<char>>;
type ValMap = Vec<Vec<i32>>;

#[derive(Debug, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}



fn parse_map() ->  Vec<Vec<char>> {
    FILE_CONTENTS.split("\n").map(|line| line.chars().collect()).collect()
}


fn get_next_position_by_pipe(pos: &Position, entry_dir: Direction, tile: char, map_size: usize) -> (Option<Position>, Option<Direction>) {

    match tile {
    'F' => {
        match entry_dir {
            Direction::UP  => {(
                get_position_by_direction(&pos, Direction::RIGHT, map_size),
                Some(Direction::RIGHT)
            )}
            Direction::LEFT => {(
                get_position_by_direction(&pos, Direction::DOWN, map_size),
                Some(Direction::DOWN)
            )}
            _ => {(None, None)}
        }
    },
    '-' => {
        match entry_dir {
            Direction::LEFT  => {(
                get_position_by_direction(&pos, Direction::LEFT, map_size),
                Some(Direction::LEFT)
            )}
            Direction::RIGHT => {(
                get_position_by_direction(&pos, Direction::RIGHT, map_size),
                Some(Direction::RIGHT)
            )}
            _ => {(None, None)}
        }
    },
    '7' => {
        match entry_dir {
            Direction::RIGHT  => {(
                get_position_by_direction(&pos, Direction::DOWN, map_size),
                Some(Direction::DOWN)
            )}
            Direction::UP => {(
                get_position_by_direction(&pos, Direction::LEFT, map_size),
                Some(Direction::LEFT)
            )}
            _ => {(None, None)}
        }
    },
    '|' => {
        match entry_dir {
            Direction::UP  => {(
                get_position_by_direction(&pos, Direction::UP, map_size),
                Some(Direction::UP)
            )}
            Direction::DOWN => {(
                get_position_by_direction(&pos, Direction::DOWN, map_size),
                Some(Direction::DOWN)
            )}
            _ => {(None, None)}
        }
    },
    'J' => {
        match entry_dir {
            Direction::DOWN  => {(
                get_position_by_direction(&pos, Direction::LEFT, map_size),
                Some(Direction::LEFT)
            )}
            Direction::RIGHT => {(
                get_position_by_direction(&pos, Direction::UP, map_size),
                Some(Direction::UP)
            )}
            _ => {(None, None)}
        }
    },
    'L' => {
        match entry_dir {
            Direction::DOWN  => {(
                get_position_by_direction(&pos, Direction::RIGHT, map_size),
                Some(Direction::RIGHT)
            )}
            Direction::LEFT => {(
                get_position_by_direction(&pos, Direction::UP, map_size),
                Some(Direction::UP)
            )}
            _ => {(None, None)}
        }
    },
    // '.' => {
    //     None
    // },
    _ => (None, None)
    }

}

fn get_position_by_direction(pos: &Position, direction: Direction, map_size: usize) -> Option<Position> {
    let x = pos.x;
    let y = pos.y;

    let mut position = None;

    match direction {
        Direction::UP => {
            if y != 0 {
                position =  Some(Position{ x, y:y-1})
            }
        },
        Direction::DOWN => {
            if y != map_size - 1 {
                position = Some(Position{ x, y:y + 1} )
            }
        },
        Direction::LEFT => {
            if x != 0 {
                position = Some(Position{ x:x - 1, y })
            }
        },
        Direction::RIGHT => {
            if x != map_size - 1 {
                position = Some(Position{ x:x+1 , y })
            }
        }
    };

    position
}

fn find_starting_location_and_pipes(map: &Map) -> (Position, Vec<(Position,Direction)>) {
    let mut start = None;

    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == 'S' {
                start = Some(Position {x, y});
                break;
            }
        }
    }
    let map_size = map.len();
    let start = start.unwrap();
    let mut starting_pipes = vec![];
    // let starting_pipes = vec![
    //     get_position_by_direction(&start, Direction::UP, map_size).unwrap(),
    //     get_position_by_direction(&start, Direction::DOWN, map_size).unwrap(),
    //     get_position_by_direction(&start, Direction::LEFT, map_size).unwrap(),
    //     get_position_by_direction(&start, Direction::RIGHT, map_size).unwrap()
    // ];
    if let Some(pos) = get_position_by_direction(&start, Direction::UP, map_size) {
        starting_pipes.push((pos, Direction::UP));
    }

    if let Some(pos) = get_position_by_direction(&start, Direction::DOWN, map_size) {
        starting_pipes.push((pos, Direction::DOWN));
    }
    if let Some(pos) = get_position_by_direction(&start, Direction::LEFT, map_size) {
        starting_pipes.push((pos, Direction::LEFT));
    }
    if let Some(pos) = get_position_by_direction(&start, Direction::RIGHT, map_size) {
        starting_pipes.push((pos, Direction::RIGHT));
    }

    // println!("{:?}", starting_pipes);



    (start, starting_pipes)
}



fn follow_pipes(pos: Position, map: &Map, start_direction: Direction) -> i32 {
    let mut finished = false;
    let mut current_pos = pos;
    let mut current_dir = start_direction;
    let map_size = map.len();
    let mut count = 0;
    while !finished {
        // println!("{:?}: {:?}, {:?}", current_pos, current_dir, map[current_pos.y][current_pos.x]);

        let current_tile = map[current_pos.y][current_pos.x];
        let (next_pos, next_dir) = get_next_position_by_pipe(&current_pos, current_dir, current_tile, map_size);
        if let (Some(next_pos), Some(next_dir)) = (next_pos, next_dir) {
            current_pos = next_pos;
            current_dir = next_dir;
        }
        else {
            finished = true
        }
        count += 1;
    }
    // println!();
    // println!();

    if map[current_pos.y][current_pos.x] != 'S' {
        return -1;
    }

    return count
}

fn follow_pipes_2(pos: Position, map: &Map, start_direction: Direction) -> Vec<Position> {
    let mut finished = false;
    let mut current_pos = pos;
    let mut current_dir = start_direction;
    let map_size = map.len();
    let mut pipe_path = vec![current_pos.clone()];
    while !finished {
        // println!("{:?}: {:?}, {:?}", current_pos, current_dir, map[current_pos.y][current_pos.x]);

        let current_tile = map[current_pos.y][current_pos.x];
        let (next_pos, next_dir) = get_next_position_by_pipe(&current_pos, current_dir, current_tile, map_size);
        if let (Some(next_pos), Some(next_dir)) = (next_pos, next_dir) {
            current_pos = next_pos;
            current_dir = next_dir;
            pipe_path.push(current_pos.clone());
        }
        else {
            // println!("next {:?}", (next_pos, next_dir) );
            finished = true
        }
        
    }
    // println!();
    // println!();

    if map[current_pos.y][current_pos.x] != 'S' {
        return vec![];
    }

    return pipe_path
}


fn part1 () -> i32 {
    let map = parse_map();

    let (_starting_location, starting_pipes) = find_starting_location_and_pipes(&map);

    // println!("{:?}", find_starting_location_and_pipes(map));

    for (pos, dir) in starting_pipes {
        let total_travel = follow_pipes(pos, &map, dir);
        if total_travel != -1 {
            return total_travel / 2
        }
    }    

    0
}



fn _print_map(map: &Map) {
    for line in map {
        for tile in line {
            print!("{}", tile)
        }
        println!()
    }
}

fn _print_map2(map: &ValMap) {
    for line in map {
        for tile in line {
            print!(" {:4} ", tile)
        }
        println!()
    }
}


fn find_tiles_in_loop(map:Map, val_map:ValMap) -> i32 {
    // let mut inside = true;
    let mut counter = 0;


    for (y, line) in map.iter().enumerate() {
        let mut wall_count= 0;
        for (x, tile) in line.iter().enumerate() {
            let curr_val = val_map[y][x];
            // let next_val: i32 = val_map[y][x+1];
            
            if curr_val == -1  {
                let inside = wall_count % 2 == 1;

                if inside {
                    // println!("found tile {:?}", (x,y));
                    counter += 1;
                }
                continue;
            }
            else {
                if vec!['|', 'J', 'L'].contains(&tile) {
                    wall_count +=1;
                }
            }
        }
    }

    return counter
}

fn calculate_s_pipe(travel_map: &Vec<Position>) -> char {
    let first_point = travel_map[0].clone();
    let last_point = travel_map[travel_map.len() -1].clone();
    let second_to_last_point = travel_map[travel_map.len() -2].clone();

    let mut point_set = HashSet::new();

    point_set.insert(Point{ 
        x:  ( first_point.x as i32) - (last_point.x as i32),
        y:  ( first_point.y as i32) - (last_point.y as i32)
    });

    point_set.insert(Point{ 
        x:  ( second_to_last_point.x as i32) - (last_point.x as i32),
        y:  ( second_to_last_point.y as i32) - (last_point.y as i32)
    });


    let zero_one = Point{x:0 , y:1};
    let one_zero = Point{x:1 , y:0};
    let neg_zero = Point{x:-1, y:0};
    let zero_neg = Point{x:0, y: -1};
   
    

    let dash_shape = HashSet::from([neg_zero, one_zero]);
    let pipe_shape = HashSet::from([zero_neg, zero_one]);
    let f_shape = HashSet::from([zero_one, one_zero]);
    let seven_shape = HashSet::from([zero_one, neg_zero]);
    let j_shape = HashSet::from([neg_zero, zero_neg]);
    let l_shape = HashSet::from([zero_neg, one_zero]);

    // if point_set.eq(&dash_shape) {
    //     return '-'
    // }

    if dash_shape.eq(&point_set) {
        return  '-';
    }
    else if pipe_shape.eq(&point_set) {
        return '|';
    }
    else if f_shape.eq(&point_set) {
        return 'F';
    }
    else if seven_shape.eq(&point_set) {
        return '7';
    }
    else if j_shape.eq(&point_set) {
        return 'J';
    }
    else if l_shape.eq(&point_set) {
        return 'L';
    }

    return '.'


}

fn part2 () -> i32 {
    let mut map = parse_map();
    let mut value_map = vec![vec![-1; map.len()]; map.len()];

    let (starting_location, starting_pipes) = find_starting_location_and_pipes(&map);

    // println!("{:?}", find_starting_location_and_pipes(map));
    let mut travel_map = vec![];
    for (pos, dir) in starting_pipes {
        travel_map = follow_pipes_2(pos, &map, dir);
        if travel_map.len() != 0 {
            break
        }

    }

    let start_tile = calculate_s_pipe(&travel_map);
    // println!("{:?}", travel_map);
    println!("st: {:?}", start_tile);

    map[starting_location.y][starting_location.x] = start_tile;

    for (count, pos) in travel_map.iter().enumerate() {
        value_map[pos.y][pos.x] = (count +1)as i32;
    }

    

    // print_map(&map);
    // print_map2(&value_map);


    find_tiles_in_loop(map, value_map)
}
