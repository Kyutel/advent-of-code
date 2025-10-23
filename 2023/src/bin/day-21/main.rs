use std::collections::HashSet;

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
// static  STEPS: usize = 100;
static FILE_CONTENTS: &str = include_str!("input.txt");
static STEPS: usize = 64;


type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MapPos {
    x: i64,
    y: i64
}



fn main() {
    println!("{:?}", part1());
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

fn get_position_by_direction(pos: &Position, direction: Direction, map: &Grid) -> Option<Position> {
    let x = pos.x;
    let y = pos.y;

    let map_size = map.len();

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


    if let Some(position) = position {

        if map[position.y][position.x] != '.' {
            return None
        }
    }

    position
}

fn get_map() -> Grid {
    FILE_CONTENTS.split('\n').map(|line| line.chars().collect()).collect()
}

fn get_reachable_plots(grid: &Grid, steps_to_take: usize, starting_position: Position) -> HashSet<Position> {

    let mut reachable_plots = HashSet::new();
    reachable_plots.insert(starting_position);
    let mut walk_from =  HashSet::from_iter(vec![starting_position]);

    for steps in 0..steps_to_take {
        let mut new_to_walk = HashSet::new();
        let next_positions = vec![
            Direction::UP,
            Direction::DOWN,
            Direction::LEFT,
            Direction::RIGHT,
        ];

        for pos_to_walk_from in walk_from {
            for dir in next_positions.iter() {
                if let Some(new_pos) = get_position_by_direction(&pos_to_walk_from, *dir, &grid) {
                    if steps == steps_to_take - 1 {
                        reachable_plots.insert(new_pos);
                    }
                    new_to_walk.insert(new_pos);
                }
            }
        }

        walk_from = new_to_walk;

    }


    reachable_plots

}

fn part1 ()  -> usize {
    let map = get_map();
    
    // _print_grid(&map);

    let mut starting_pos = None;
    for (y, line) in map.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if tile == &'S'{ 
                starting_pos = Some(Position {x ,y});
            }
        }
    }

    get_reachable_plots(&map, STEPS, starting_pos.unwrap()).len()

}


// fn generate_points_to_check(steps_taken:i64, base_pos: Pos) -> Vec<Pos> {
    
// }

fn get_position_by_direction_inf(pos: &Position, direction: Direction, map: &Grid, map_pos: MapPos) -> (Option<Position>, MapPos){
    let x = pos.x;
    let y = pos.y;

    let map_size = map.len();

    let position;
    let mut new_map_pos = map_pos;

    match direction {
        Direction::UP => {
            if y != 0 {
                position =  Some(Position{ x, y:y-1})
            }
            else {
                position = Some(Position {x, y:map_size-1});
                new_map_pos.y -= 1;
            }
        },
        Direction::DOWN => {
            if y != map_size - 1 {
                position = Some(Position{ x, y:y + 1} )
            }
            else {
                position = Some(Position{ x, y: 0} );
                new_map_pos.y += 1;
            }
        },
        Direction::LEFT => {
            if x != 0 {
                position = Some(Position{ x:x - 1, y })
            }
            else {
                position = Some(Position{ x:map_size-1, y });
                new_map_pos.x -= 1;
            }
        },
        Direction::RIGHT => {
            if x != map_size - 1 {
                position = Some(Position{ x:x+1 , y })
            }
            else {
                position = Some(Position{ x:0 , y });
                new_map_pos.x += 1;
            }
        }
    };


    if let Some(position) = position {

        if map[position.y][position.x] == '#' {
            return (None, new_map_pos)
        }
    }

    (position, new_map_pos)
}

fn get_reachable_plots_inf(grid: &Grid, steps_to_take: usize, starting_position: Position) -> usize {

    let mut reachable_plots = HashSet::new();
    let mut map_pos  = MapPos { x: 0, y: 0};
    reachable_plots.insert((starting_position, map_pos));
    let mut walk_from =  HashSet::from_iter(vec![(starting_position, map_pos)]);

    let mut walked = vec![(walk_from.clone())];

    for steps in 0..steps_to_take {
        let mut new_to_walk = HashSet::new();
        let walk_from= &walked[walked.len()-1];
        let next_positions = vec![
            Direction::UP,
            Direction::DOWN,
            Direction::LEFT,
            Direction::RIGHT,
        ];

        for (pos_to_walk_from, map_pos_from) in walk_from {
            for dir in next_positions.iter() {
                if let (Some(new_pos), new_map_pos) = get_position_by_direction_inf(pos_to_walk_from, *dir, &grid, *map_pos_from) {
                    new_to_walk.insert((new_pos, new_map_pos));
                    map_pos = new_map_pos;
                }
            }
        }

        if walked.len() > 1{
            let new_set = &new_to_walk - &walked[walked.len()-2];
            walked.push(new_set);
        }
        else {
            walked.push(new_to_walk.clone());
        }



        // println!("Steps left to take {:?}", steps_to_take - steps)
    }

    // for (i, row) in walked.iter().enumerate() {
    //     println!("{:?}: Len: {:?} {:?}", i,row.len(), row);
    // }

    if steps_to_take % 2 == 0 {
        let t:Vec<HashSet<(Position, MapPos)>> =  walked.iter().step_by(2).map(|t|t.clone()).collect();
        return t.iter().map(|x | x.len()).sum()
    }  
    else {
        let t:Vec<HashSet<(Position, MapPos)>> =  walked.iter().skip(1).step_by(2).map(|t|t.clone()).collect();
        let a:usize = t.iter().map(|x |x.len()).sum();
        return a;
    }

}

fn part2 () -> usize {
    let map = get_map();

    // making assumption start point is middle of the map
    let mid_point = map.len() / 2 ;
    let starting_pos = Position {x:mid_point, y:mid_point};
    let one = get_reachable_plots_inf(&map, 65, starting_pos);
    let two = get_reachable_plots_inf(&map, 196, starting_pos);
    let three = get_reachable_plots_inf(&map, 327, starting_pos);
    

    // idk lmao I looked up this formula
    let a = (three - (2*two) + one) / 2;
    let b = two - one - a;
    let c = one;
    let n = (26_501_365 -65) / 131;

    return (a * (n*n)) + (b*n) + c

}
