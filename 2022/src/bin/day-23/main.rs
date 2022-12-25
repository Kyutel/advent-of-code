use std::collections::{HashSet, HashMap};

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
// static FILE_CONTENTS: &str = include_str!("test-input2.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East
}

static DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::West, Direction::East];


#[derive(Debug)]
struct Grove {
    elves: HashSet<Point>,
    smallest_point: Point,
    biggest_point: Point,
}


impl Grove {
    fn _print_grove(&self) {
        for y in self.smallest_point.y..self.biggest_point.y+1 {
            for x in self.smallest_point.x..self.biggest_point.x+1 {
                let current_point = Point { x, y };
                let c = if self.elves.contains(&current_point) {'#'} else {'.'};
                print!("{c}");
            }
            println!();
        }
    }

    fn check_should_move(&self, elf: Point)  -> bool {
        // let mut points_to_check = vec![];
        for x in elf.x-1..elf.x+2 {
            for y in elf.y-1..elf.y+2 {
                let p = Point{x,y};
                if p != elf && self.elves.contains(&p){
                    return true;
                }

            }
        }

        false
    }

    fn check_spot(&self, elf: Point, direction: Direction) -> Option<Point> {
        let spot;
        match direction {
            Direction::North => {
                for x in elf.x-1..elf.x+2 {
                    let p = Point{x, y:elf.y-1};
                    if self.elves.contains(&p) {
                        return None
                    }
                }
                spot = Point{x:elf.x, y:elf.y -1}
            },
            Direction::South => {
                for x in elf.x-1..elf.x+2 {
                    let p = Point{x, y:elf.y+1};
                    if self.elves.contains(&p) {
                        return None
                    }
                }
                spot = Point{x:elf.x, y:elf.y +1}
            },
            Direction::West => {
                for y in elf.y-1..elf.y+2 {
                    let p = Point{x:elf.x-1, y};
                    if self.elves.contains(&p) {
                        return None
                    }
                }
                spot = Point{x:elf.x-1, y:elf.y}
            },
            Direction::East => {
                for y in elf.y-1..elf.y+2 {
                    let p = Point{x:elf.x+1, y};
                    if self.elves.contains(&p) {
                        return None
                    }
                }
                spot = Point{x:elf.x+1, y:elf.y}
            },

        };
        Some(spot)
    }

    fn spread_elves(&mut self, round: i32) -> bool{
        let mut elf_picks: HashMap<Point, Point> = HashMap::new();
        let mut picked_spots: HashMap<Point, i32> = HashMap::new();
        let mut moved = false;
        for x in 0..4 {
            let d = DIRECTIONS[(x + round as usize)%4];
            for elf in self.elves.iter() {
                if self.check_should_move(*elf) && !elf_picks.contains_key(elf) {
                    if let Some(spot) = self.check_spot(*elf, d) {
                        elf_picks.insert(*elf, spot);
                        let count = picked_spots.entry(spot).or_insert(0);
                        *count+=1;
                    }

                }
                // picked_spots.insert(, v)
            }
            // print!("{d:?}, ");
        }

        for (elf, pick) in elf_picks.iter() {
            if *picked_spots.get(pick).unwrap() == 1 {
                self.elves.remove(elf);
                self.elves.insert(*pick);
                self.smallest_point = Point{x: self.smallest_point.x.min(pick.x), y: self.smallest_point.y.min(pick.y)};
                self.biggest_point = Point{x: self.biggest_point.x.max(pick.x), y: self.biggest_point.y.max(pick.y)};
                moved = true;
            }
        }

        moved
        // println!();
    }

    fn get_empty_tile_count(&self) -> i32 {
        let mut count = 0;
        for y in self.smallest_point.y..self.biggest_point.y+1 {
            for x in self.smallest_point.x..self.biggest_point.x+1 {
                let current_point = Point { x, y };
                if !self.elves.contains(&current_point) {
                    count +=1;
                }

            }
        }

        count
    }
}


fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_input() -> Grove {
    let elves: HashSet<Point> = FILE_CONTENTS.split('\n').enumerate()
    .flat_map(
        |(y, row)| row.chars().enumerate()
            .filter(|(_, tile)| *tile == '#')
            .map(move |(x, _)| Point{x:x as i32, y: y as i32})
    )
    .collect();
    let smallest_point = Point {x:0, y:0};
    let mut biggest_point = Point {x:0, y:0};
    
    for elf_location in elves.iter() {
        biggest_point = Point{x: biggest_point.x.max(elf_location.x), y: biggest_point.y.max(elf_location.y)};
    }


    Grove { elves, smallest_point, biggest_point}
} 

fn part1 () -> i32 {
    let mut grove = get_input();

    // println!("{grove:?}")
    for round in 0..10 {
        grove.spread_elves(round);
    }
    // grove.print_grove();
    grove.get_empty_tile_count()
}
fn part2 () -> i32{
    // let count = 0
    let mut round = 0;
    let mut grove = get_input();

    while grove.spread_elves(round) {
        round+=1;
    }

    round + 1
}
