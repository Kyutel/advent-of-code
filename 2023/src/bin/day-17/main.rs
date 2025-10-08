use std::{collections::HashMap, u32};

// CODE WORKS BUT INCREDIBLY INEFFICIENT, NEED TO IMPROVE TBH

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

type Grid = Vec<Vec<u32>>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West
}


#[derive(Debug, Hash, PartialEq, Eq)]
struct BlockTravel {
    pos: Pos,
    dir: Direction,
    straight_count: usize,
    // pos_history: Vec<Pos>
}

fn main() {
    // println!("{:?}", part1());
    println!("{:?}", part2());
}

fn _print_grid(grid : &Grid) {
    for (_line_no, line) in grid.iter().enumerate() {
        // print!("{:?} \t", _line_no);
        for tile in line {
            print!("{}", tile);
        }
        println!();
    }
}

fn get_grid() -> Grid {
    FILE_CONTENTS.split('\n').map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect()
}

fn change_direction(facing: Direction, rotation: char) -> Direction {
    match (facing, rotation) {
        (Direction::East , 'R') => Direction::South,
        (Direction::East , 'L')  => { Direction::North},
        (Direction::South , 'R') => Direction::West,
        (Direction::South , 'L')  => { Direction::East},
        (Direction::West , 'R') => Direction::North,
        (Direction::West , 'L')  => { Direction::South},
        (Direction::North , 'R') => Direction::East,
        (Direction::North , 'L')  => { Direction::West},
        _ => panic!("unexpected rotation")
    }
}

fn move_forward(pos: Pos, dir: Direction, grid_length: usize, spaces: usize) -> Option<Pos> {
    match dir {
        Direction::North => {
            if pos.y > spaces - 1 {
                Some(Pos { x: pos.x, y: pos.y - spaces })
            }
            else {
                None
            }
        },
        Direction::East => {
            if pos.x < grid_length - spaces{
                Some(Pos { x: pos.x + spaces, y: pos.y })
            }
            else {
                None
            }
        },
        Direction::South => {
            if pos.y < grid_length - spaces{
                Some(Pos { x: pos.x, y: pos.y + spaces})
            }
            else {
                None
            }
        },
        Direction::West => {
            if pos.x >  spaces - 1{
                Some(Pos { x: pos.x - spaces, y: pos.y })
            }
            else {
                None
            }
        }
    }
}

fn get_next_blocks(block: &BlockTravel, heat_loss: u32, pos_history: Vec<Pos>, grid: &Grid) -> Vec<(BlockTravel, u32, Vec<Pos>)> {
    let mut paths = vec![];
    
    let left_turn_dir = change_direction(block.dir, 'L');
    if let Some(new_pos) = move_forward(block.pos, left_turn_dir, grid.len(), 1) {
        let new_heat_loss = heat_loss + grid[new_pos.y][new_pos.x];
        let mut hist_clone = pos_history.clone();
        hist_clone.push(new_pos);
        let new_block = BlockTravel { pos: new_pos, dir: left_turn_dir, straight_count: 1};
        paths.push((new_block, new_heat_loss, hist_clone));
    }
    let right_turn_dir = change_direction(block.dir, 'R');
    if let Some(new_pos) = move_forward(block.pos, right_turn_dir, grid.len(), 1) {
        let new_heat_loss = heat_loss + grid[new_pos.y][new_pos.x];
        let mut hist_clone = pos_history.clone();
        hist_clone.push(new_pos);
        let new_block = BlockTravel { pos: new_pos, dir: right_turn_dir, straight_count: 1};
        paths.push((new_block, new_heat_loss, hist_clone));
    }

    if block.straight_count < 3{
        if let Some(new_pos) = move_forward(block.pos, block.dir, grid.len(), 1) {
            let new_heat_loss = heat_loss + grid[new_pos.y][new_pos.x];
            let mut hist_clone = pos_history.clone();
            hist_clone.push(new_pos);
            let new_block = BlockTravel { pos: new_pos, dir: block.dir, straight_count: block.straight_count + 1};
            paths.push((new_block, new_heat_loss, hist_clone));
        }
    }

    paths
}

fn get_next_blocks_two(block: &BlockTravel, heat_loss: u32, pos_history: Vec<Pos>, grid: &Grid) -> Vec<(BlockTravel, u32, Vec<Pos>)> {
    let mut paths = vec![];

    let left_turn_dir = change_direction(block.dir, 'L');
    if let Some(new_pos) = move_forward(block.pos, left_turn_dir, grid.len(), 4) {
        let mut new_heat_loss: u32 = heat_loss;
        for spaces in 0..4 {
            let interim_pos = move_forward(block.pos, left_turn_dir, grid.len(), spaces+ 1).unwrap();
            new_heat_loss +=grid[interim_pos.y][interim_pos.x];
        }
        let mut hist_clone = pos_history.clone();
        hist_clone.push(new_pos);
        let new_block = BlockTravel { pos: new_pos, dir: left_turn_dir, straight_count: 4};
        paths.push((new_block, new_heat_loss, hist_clone));
    }
    let right_turn_dir = change_direction(block.dir, 'R');
    if let Some(new_pos) = move_forward(block.pos, right_turn_dir, grid.len(), 4) {
        let mut new_heat_loss: u32 = heat_loss;
        for spaces in 0..4 {
            let interim_pos = move_forward(block.pos, right_turn_dir, grid.len(), spaces+ 1).unwrap();
            new_heat_loss += grid[interim_pos.y][interim_pos.x];
        }
        let mut hist_clone = pos_history.clone();
        hist_clone.push(new_pos);
        let new_block = BlockTravel { pos: new_pos, dir: right_turn_dir, straight_count: 4};
        paths.push((new_block, new_heat_loss, hist_clone));
    }

    if block.straight_count < 10{
        if let Some(new_pos) = move_forward(block.pos, block.dir, grid.len(), 1) {
            let new_heat_loss = heat_loss + grid[new_pos.y][new_pos.x];
            let mut hist_clone = pos_history.clone();
            hist_clone.push(new_pos);
            let new_block = BlockTravel { pos: new_pos, dir: block.dir, straight_count: block.straight_count + 1};
            paths.push((new_block, new_heat_loss, hist_clone));
        }
    }

    paths
}

fn get_path_with_min_heat_loss(grid:Grid, start_pos: Pos, end_pos: Pos, part_no: i32)  -> u32 {
    let mut best_losses = HashMap::new();
    let mut best_final_loss = u32::MAX;
    let straight_count = 0;


    let mut paths_to_take = vec![];
    let right_path = BlockTravel { pos: start_pos, dir: Direction::East, straight_count: straight_count};
    let down_path = BlockTravel { pos: start_pos, dir: Direction::South, straight_count: straight_count};
    let mut best_path = vec![];


    let next_block_func = if part_no == 1 {
        get_next_blocks
    }
    else {
        get_next_blocks_two
    };

    paths_to_take.extend(next_block_func(&right_path, 0,vec![], &grid));
    paths_to_take.extend(next_block_func(&down_path, 0,vec![], &grid ));

    while paths_to_take.len() > 0 {
        let (next_block, next_heat_loss, next_path_history) = paths_to_take.pop().unwrap();

        if next_heat_loss > best_final_loss {
            continue;
        }

        if next_block.pos == end_pos {
            if next_heat_loss < best_final_loss {
                best_final_loss = next_heat_loss;
                best_path = next_path_history.clone();
                println!("{:?}, {:?}", next_block, next_heat_loss);
            }
        }

        if let Some(best_loss) = best_losses.get(&next_block) {
            if next_heat_loss < *best_loss {
                // println!("{:?}, {:?}", next_block, heat_loss);
                paths_to_take.extend(next_block_func(&next_block, next_heat_loss,next_path_history, &grid));
                best_losses.insert(next_block, next_heat_loss);
            }
        }
        else {
            // println!("{:?}", heat_loss);
            paths_to_take.extend(next_block_func(&next_block, next_heat_loss,next_path_history, &grid));
            best_losses.insert(next_block, next_heat_loss);
        }
    }

    let mut grid_clone = grid.clone();
    for pos in best_path {
        grid_clone[pos.y][pos.x] = 0;
        // println!("{:?}", pos);
    }

    // _print_grid(&grid_clone);

    best_final_loss

}   




fn part1 () -> u32 {
    let grid = get_grid();

    let start_pos = Pos {x:0, y:0};
    let end_pos = Pos {x:grid[0].len() - 1, y: grid.len() - 1};
    get_path_with_min_heat_loss(grid, start_pos, end_pos, 1)
    // let u = 0 - 1 as usize;



}
fn part2 () -> u32 {
    let grid = get_grid();

    let start_pos = Pos {x:0, y:0};
    let end_pos = Pos {x:grid[0].len() - 1, y: grid.len() - 1};
    get_path_with_min_heat_loss(grid, start_pos, end_pos, 2)
}
