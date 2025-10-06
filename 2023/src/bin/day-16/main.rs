use std::fmt;

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

// type Grid = Vec<Vec<char>>;
type Grid = Vec<Vec<Tile>>;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West
}


#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    HBeam,
    VBeam,
    PBeam,
    VSplitter(bool),
    HSplitter(bool),
    FSlash(bool),  // "/"
    BSlash(bool)   // "\"

}

#[derive(Debug, Clone, Copy,  PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::HBeam => write!(f, "H"),
            Tile::VBeam => write!(f, "V"),
            Tile::PBeam => write!(f, "+"),
            Tile::VSplitter(_) => write!(f, "|"),
            Tile::HSplitter(_) => write!(f, "-"),
            Tile::FSlash(_) => write!(f, "/"),
            Tile::BSlash(_) => write!(f, "\\")
        }
    }
}


fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn _print_grid(grid : &Grid) {
    for (line_no, line) in grid.iter().enumerate() {
        print!("{:?} \t", line_no);
        for tile in line {
            print!("{}", tile);
        }
        println!();
    }
}

fn _print_energise(grid : &Grid) {
    for (line_no, line) in grid.iter().enumerate() {
        print!("{:?} \t", line_no);
        for tile in line {
            match tile {
                Tile::HBeam => {print!("#")},
                Tile::VBeam => {print!("#")},
                Tile::PBeam => {print!("#")},
                Tile::VSplitter(true) => {print!("#")},
                Tile::HSplitter(true) => {print!("#")},
                Tile::FSlash(true) => {print!("#")},
                Tile::BSlash(true) => {print!("#")},
                _ => {print!(".")}
            }
        }
        println!();
    }
}


fn get_contraption() -> Grid {
    FILE_CONTENTS.split('\n').map(|line| 
        line.chars().map(|tile_char| {
            match tile_char {
                '.' => Tile::Empty,
                '|' => Tile::VSplitter(false),
                '-' => Tile::HSplitter(false),
                '/'=> Tile::FSlash(false),
                '\\'=> Tile::BSlash(false),
                _ => panic!("found unexpected char")
            }
        }).collect()
    ).collect()
}


fn energise_tile(old_tile: &Tile, dir: Direction) -> Tile {
    match (old_tile, dir) {
        (Tile::Empty, Direction::North | Direction::South) =>  Tile::VBeam,
        (Tile::Empty, Direction::East | Direction::West) => Tile::HBeam,
        (Tile::VBeam, Direction::East | Direction::West) => Tile::PBeam,
        (Tile::HBeam, Direction::North | Direction::South) => Tile::PBeam,
        (Tile::VSplitter(_) , _) => Tile::VSplitter(true),
        (Tile::HSplitter(_) , _) => Tile::HSplitter(true),
        (Tile::FSlash(_) , _) => Tile::FSlash(true),
        (Tile::BSlash(_) , _) => Tile::BSlash(true),
        _ => {*old_tile} //panic!("Unexpected Tile and Dir ({:?} : {:?})", old_tile, dir) maybe look into why this happening
                         // intial thoughts is to how the queue is operating
    }
}

fn hits_boundry(loc: Point, dir: Direction, grid_size: usize) -> bool{
    match dir {
        Direction::North => loc.y == 0,
        Direction::East => loc.x == grid_size -1,
        Direction::South => loc.y == grid_size -1,
        Direction::West => loc.x == 0
    }
}

fn is_vertical_dir(dir: Direction) -> bool {
    return dir == Direction::North || dir == Direction::South
}

fn is_horizontal_dir(dir: Direction) -> bool {
    return dir == Direction::East || dir == Direction::West
}

fn get_next_beams(loc: Point, dir: Direction, tile: Tile, grid_size: usize)  -> Vec<(Point, Direction)>{
    let mut next_beams = vec![];

    match tile {
        Tile::Empty => { if !hits_boundry(loc, dir, grid_size) { next_beams.push((move_beam(loc, dir), dir))} }
        Tile::HBeam => { 
            if is_vertical_dir(dir) && !hits_boundry(loc, dir, grid_size) { 
                next_beams.push((move_beam(loc, dir), dir))
            }
        }
        Tile::VBeam => { 
            if is_horizontal_dir(dir) && !hits_boundry(loc, dir, grid_size) { 
                next_beams.push((move_beam(loc, dir), dir))
            }
        }
        Tile::VSplitter(false) => {
            if is_vertical_dir(dir) && !hits_boundry(loc, dir, grid_size) {
                next_beams.push((move_beam(loc, dir), dir))
            }
            else if is_horizontal_dir(dir) {
                if !hits_boundry(loc, Direction::North, grid_size) {
                    next_beams.push((move_beam(loc, Direction::North), Direction::North))
                }
                if !hits_boundry(loc, Direction::South, grid_size) {
                    next_beams.push((move_beam(loc, Direction::South), Direction::South))
                }
            }
        } 
        Tile::HSplitter(false) => {
            if is_horizontal_dir(dir) && !hits_boundry(loc, dir, grid_size) {
                next_beams.push((move_beam(loc, dir), dir))
            }
            else if is_vertical_dir(dir) {
                if !hits_boundry(loc, Direction::East, grid_size) {
                    next_beams.push((move_beam(loc, Direction::East), Direction::East))
                }
                if !hits_boundry(loc, Direction::West, grid_size) {
                    next_beams.push((move_beam(loc, Direction::West), Direction::West))
                }
            }
        }
        Tile::FSlash(_) => {
            match dir {
                Direction::North => {
                    if !hits_boundry(loc, Direction::East, grid_size) {
                        next_beams.push((move_beam(loc, Direction::East), Direction::East))
                    }
                }
                Direction::East => {
                    if !hits_boundry(loc, Direction::North, grid_size) {
                        next_beams.push((move_beam(loc, Direction::North), Direction::North))
                    }
                }
                Direction::South => {
                    if !hits_boundry(loc, Direction::West, grid_size) {
                        next_beams.push((move_beam(loc, Direction::West), Direction::West))
                    }
                }
                Direction::West => {
                    if !hits_boundry(loc, Direction::South, grid_size) {
                        next_beams.push((move_beam(loc, Direction::South), Direction::South))
                    }
                }
            }
        }
        Tile::BSlash(_) => {
            match dir {
                Direction::North => {
                    if !hits_boundry(loc, Direction::West, grid_size) {
                        next_beams.push((move_beam(loc, Direction::West), Direction::West))
                    }
                }
                Direction::East => {
                    if !hits_boundry(loc, Direction::South, grid_size) {
                        next_beams.push((move_beam(loc, Direction::South), Direction::South))
                    }
                }
                Direction::South => {
                    if !hits_boundry(loc, Direction::East, grid_size) {
                        next_beams.push((move_beam(loc, Direction::East), Direction::East))
                    }
                }
                Direction::West => {
                    if !hits_boundry(loc, Direction::North, grid_size) {
                        next_beams.push((move_beam(loc, Direction::North), Direction::North))
                    }
                }
            }
        }
        _ => {}
    }


    return next_beams;
}

fn move_beam(loc: Point, dir: Direction) -> Point {
    match dir {
        Direction::North => Point { x: loc.x, y: loc.y - 1 },
        Direction::East => Point { x: loc.x + 1, y: loc.y },
        Direction::South => Point { x: loc.x, y: loc.y + 1},
        Direction::West => Point { x: loc.x - 1, y: loc.y },
    }
}

fn fire_beam(contraption: &mut Grid, starting_location: Point, starting_direction: Direction) {
    let grid_size = contraption.len();

    let mut beam_queue = vec![(starting_location, starting_direction)];
    while beam_queue.len() > 0 {
        let (next_loc,next_direction) = beam_queue.pop().unwrap();

        
        let current_tile = &mut contraption[next_loc.y][next_loc.x];
        beam_queue.extend(get_next_beams(next_loc, next_direction, *current_tile, grid_size));
       
        *current_tile = energise_tile(current_tile, next_direction);
    }
}

fn get_energy_count(grid: &Grid) -> i32 {
    let mut energy_count = 0;

    for line in grid {
        for tile in line {
            match tile {
                // Tile::Empty => {energy_count += 1}
                Tile::HBeam => {energy_count += 1},
                Tile::VBeam => {energy_count += 1},
                Tile::PBeam => {energy_count += 1},
                Tile::VSplitter(true) => {energy_count += 1},
                Tile::HSplitter(true) => {energy_count += 1},
                Tile::FSlash(true) => {energy_count += 1},
                Tile::BSlash(true) => {energy_count += 1},
                _ => {}
            }
        }
    }
    energy_count
}

fn part1 ()  -> i32{
    let mut contraption = get_contraption();

    // _print_grid(&contraption);
    
    fire_beam(&mut contraption, Point { x: 0, y: 0 }, Direction::East);

    println!();
    // _print_grid(&contraption);

    // println!();
    // _print_energise(&contraption);

    get_energy_count(&contraption)
}



fn get_energy_count_from_config(contraption: &Grid, starting_location: Point, starting_direction: Direction) -> i32 {
    let mut contraption_copy = contraption.clone();
    fire_beam(&mut contraption_copy, starting_location, starting_direction);
    get_energy_count(&contraption_copy)
}

fn get_all_possible_starting_configs(grid_size: usize) -> Vec<(Point, Direction)> {
    let mut configs = vec![];
    for i in 0..grid_size {
        configs.push((Point{x: i, y:0}, Direction::South));
        configs.push((Point{x: i, y:grid_size-1}, Direction::North));
        configs.push((Point{x: 0, y:i}, Direction::East));
        configs.push((Point{x: grid_size-1, y:i}, Direction::West));
    }

    configs
}

fn part2 () -> i32 {
    let contraption = get_contraption();

    //  replace this entire function with a max of a map if possible
    let mut current_max = 0;
    let grid_size = contraption.len();
    get_all_possible_starting_configs(grid_size).iter().map(|config| get_energy_count_from_config(&contraption, config.0, config.1)).max().unwrap()

    // for 
    // let mut contraption_copy = contraption.clone();
    // fire_beam(&mut contraption_copy, starting_location, starting_direction);
    // get_energy_count(&contraption_copy)
    
}
