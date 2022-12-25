// static FILE_CONTENTS: &str = include_str!("test-input.txt");
// static FILE_CONTENTS: &str = include_str!("test-input2.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blizzard {
    pos: Point,
    facing: Direction
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East
}


impl Blizzard {
    fn new(x:usize, y:usize, dir_char: char) -> Blizzard {
        let pos = Point { x, y };
        let facing = if dir_char == '^'{
            Direction::North
        } else if dir_char == 'v'{
            Direction::South
        }
        else if dir_char == '<'{
            Direction::West
        }
        else if dir_char == '>'{
            Direction::East
        }
        else {panic!()};
        Blizzard { pos, facing }
    }

    fn move_blizzard(&mut self, map_height: usize, map_width: usize) {
        let mut new_x = self.pos.x;
        let mut new_y = self.pos.y;

        match self.facing {
            Direction::North => new_y-=1,
            Direction::South => new_y+=1,
            Direction::West => new_x-=1,
            Direction::East => new_x+=1,
        };

        if new_x == 0 {
            new_x = map_width - 2;
        }
        else if new_x ==  map_width - 1 {
            new_x =  1;
        }

        if new_y == 0 {
            new_y = map_height - 2;
        }
        else if new_y ==  map_height - 1 {
            new_y = 1;
        }

        self.pos = Point {x:new_x, y:new_y};

    }

    fn _dir_char(&self) -> char {
        match self.facing {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::West => '<',
            Direction::East => '>'
        }
    }
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_map_and_blizzard_from_input() -> (Vec<Vec<char>>, Vec<Blizzard>){
    let file_split = FILE_CONTENTS.split('\n');
    let mut map:Vec<Vec<char>> = vec![]; 
    let mut blizzards = vec![];
    for (y, line) in file_split.enumerate() {
        let mut next_row = vec![];
        for (x, c) in line.chars().enumerate() {
            if c != '.' && c!= '#' {
                next_row.push('.');
                blizzards.push(Blizzard::new(x ,y ,c));
            } else {
                next_row.push(c);
            }
        }
        map.push(next_row);
    }

    (map, blizzards)
}

fn get_next_blizzards(blizzards: &Vec<Blizzard>,map_height: usize,  map_width: usize, ) -> Vec<Blizzard> {
    let mut blizzards = blizzards.clone();
    for b in blizzards.iter_mut() {
        b.move_blizzard(map_height, map_width)
    }

    blizzards
}

fn advance_time(time_blizzards: &mut Vec<Vec<Blizzard>>, map_height: usize, map_width: usize) {
    let latest_blizz = &time_blizzards[time_blizzards.len()-1];
    let next_blizzard = get_next_blizzards(latest_blizz, map_height, map_width);
    
    time_blizzards.push(next_blizzard);
}

fn check_if_blizzard_in_pos(time_blizzards: &Vec<Vec<Blizzard>>, time: usize, pos_to_check: Point) -> bool {
    let blizzards_at_time = &time_blizzards[time];
    for blizz in blizzards_at_time {
        if blizz.pos == pos_to_check {
            return true
        }
    }
    
   false
}


fn get_next_possible_positions(current_position: Point, current_time: usize, start_time: usize, time_blizzards: &mut Vec<Vec<Blizzard>>, map_height: usize, map_width: usize)  -> Vec<Point>{
    let mut possible_positions = vec![];
    let next_time = current_time + 1;
    let blizzard_time = next_time + start_time; 
    if time_blizzards.len() -1  < blizzard_time {
        advance_time(time_blizzards, map_height, map_width)
    }



    if (current_position.y!= map_height-2 && current_position.y != map_height-1) || (current_position.y==map_height-2 && current_position.x==map_width-2) {
        let south = Point{x: current_position.x, y: current_position.y + 1};
        if !check_if_blizzard_in_pos(time_blizzards, blizzard_time, south) {
            possible_positions.push(south)
        }
    }

    if (current_position.y!= 1 && current_position.y != 0) || (current_position.y==1 && current_position.x==1) {
        let north = Point{x: current_position.x, y: current_position.y - 1};
        if !check_if_blizzard_in_pos(time_blizzards, blizzard_time, north) {
            possible_positions.push(north)
        }
    }

    if current_position.x!= map_width-2 && current_position.y != 0{
        let east = Point{x: current_position.x + 1, y: current_position.y};
        if !check_if_blizzard_in_pos(time_blizzards, blizzard_time, east) {
            possible_positions.push(east)
        }
    }

    if current_position.x!= 1  && current_position.y != map_height-1{
        let west = Point{x: current_position.x - 1, y: current_position.y};
        if !check_if_blizzard_in_pos(time_blizzards, blizzard_time, west) {
            possible_positions.push(west)
        }
    }


    if !check_if_blizzard_in_pos(time_blizzards, blizzard_time, current_position) {
        possible_positions.push(current_position)
    }

    return possible_positions
}

fn insert_into_search_queue(search_queue: &mut Vec<(Point, usize)>, pos_time: (Point,usize), goal: Point) {
    let mut index_to_insert = 0;
    for item in search_queue.iter() {
        if *item == pos_time {
            return;
        }
        // if item.0 == pos_time.0 {
        //     return;
        // }
    }

    for (index, existing) in search_queue.iter().enumerate() {
        let inserting_pos = pos_time.0;
        let inserting_time = pos_time.1;

        let existing_pos = existing.0;
        let existing_time = existing.1;
        let best_possible_inserting = goal.x.abs_diff(inserting_pos.x) + goal.y.abs_diff(inserting_pos.y) + inserting_time;
        let best_possible_existing = goal.x.abs_diff(existing_pos.x) + goal.y.abs_diff(existing_pos.y) + existing_time;

        if best_possible_inserting > best_possible_existing {
            index_to_insert = index;
            break;
        }
    }

    search_queue.insert(index_to_insert, pos_time);
}

fn get_shortest_time(start: Point, goal: Point, time_blizzards: &mut Vec<Vec<Blizzard>>, map_height: usize, map_width: usize, start_time: usize) -> usize{
    let mut search_queue: Vec<(Point, usize)> = vec![(start, 0)];
    // let mut time_map: HashMap<Point, usize> = HashMap::new();
    // println!("STARTO {search_queue:?}");
    // time_map.insert(start, 0);
    while search_queue.len() > 0 {
    // for _t in 0..110 {
        // println!("{search_queue:?}");
        // println!("");
        let next = search_queue.pop().unwrap();
        let current_position = next.0;
        let current_time = next.1;
        if current_position == goal {
            return next.1;
        }
        let next_positions = get_next_possible_positions(current_position, current_time, start_time, time_blizzards, map_height, map_width);
        

        for pos in next_positions {
            let next_pos_time = current_time + 1;
            // let current_known = time_map.entry(position).or_insert(next_position_value);
            // if next_pos_time <= *current_known {
                insert_into_search_queue(&mut search_queue, (pos, next_pos_time), goal)
            // }

        }

    }

    0
}

fn part1 () -> usize{
    let (map, blizzards) = get_map_and_blizzard_from_input();
    
    let map_height = map.len();
    let map_width = map[0].len();


    let me = Point{x: 1, y: 0};
    let goal = Point{x: map_width -2 , y: map_height-1};

    println!("{goal:?}");

    let mut time_blizzards: Vec<Vec<Blizzard>> = vec![blizzards];

    get_shortest_time(me, goal, &mut time_blizzards, map_height, map_width,0)

    // for i in 1..6 {
    //     if time_blizzards.len() -1 < i {
    //         advance_time(&mut time_blizzards, map_height, map_width)
    //     }
    // }

    // let latest_blizz = &time_blizzards[time_blizzards.len()-1];
    // _print_map(&map, Some(latest_blizz));

    // _print_map(&map, None);

    // for blizz in blizzards {
    //     println!("{blizz:?}")
    // }

}

fn part2 () -> usize{

    let (map, blizzards) = get_map_and_blizzard_from_input();
    
    let map_height = map.len();
    let map_width = map[0].len();


    let me = Point{x: 1, y: 0};
    let goal = Point{x: map_width -2 , y: map_height-1};
    let mut time_blizzards: Vec<Vec<Blizzard>> = vec![blizzards];

    let time_1 = get_shortest_time(me, goal, &mut time_blizzards, map_height, map_width, 0);

    
    let time_2 = get_shortest_time(goal, me, &mut time_blizzards, map_height, map_width, time_1);
    let time_3 = get_shortest_time(me, goal, &mut time_blizzards, map_height, map_width, time_1+time_2);

    // println!()

    time_1 + time_2 + time_3
}



fn _print_map(map: &Vec<Vec<char>>, blizzards: Option<&Vec<Blizzard>>) {
    let mut map_clone = map.clone();
    if let Some(blizzards) = blizzards {
        for b in blizzards {
            if map_clone[b.pos.y][b.pos.x] == '.' {
                map_clone[b.pos.y][b.pos.x] = b._dir_char();
            }
            else {
                map_clone[b.pos.y][b.pos.x] = 'M';
            }
        }
    }


    for row in map_clone {
        for c in row {
            print!("{c}");
        }
        println!();
    }
}