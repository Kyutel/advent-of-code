use std::collections::HashSet;

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

type Platform = Vec<Vec<char>>;

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_platform() -> Platform {
    FILE_CONTENTS.split('\n').map(|line|line.chars().collect()).collect()
}

fn _print_platform(platform: &Platform) {
    for line in platform {
        for tile in line {
            print!("{}", tile)
        }
        println!()
    }
}

fn get_load(platform: &Platform) -> usize {
    let mut load = 0;

    for (row_num, line) in platform.iter().enumerate() {
        for tile in line {
            // print!("{}", tile)
            if tile == &'O' {
                load += platform.len() - row_num;
            }
        }
    }

    load
}



fn part1 () -> usize{
    let mut platform = get_platform();


    roll_north(&mut platform);
    

    // _print_platform(&platform);
    return get_load(&platform);

}


fn roll_north(platform: &mut Platform) {
    let mut top_line = vec![0 as usize;platform[0].len()];

    for y in 0..platform.len() {
        for x in 0..platform[y].len() {
            if platform[y][x] == 'O' {
                platform[y][x] = '.';
                platform[top_line[x]][x] = 'O';
                top_line[x] = top_line[x]+1
            }
            else if platform[y][x] == '#' {
                top_line[x] = y+1
            }
        }
    }

}

fn roll_west(platform: &mut Platform) {
    let mut left_line = vec![0 as usize;platform.len()];

    for x in 0..platform[0].len() {
        for y in 0..platform.len() {
            if platform[y][x] == 'O' {
                platform[y][x] = '.';
                platform[y][left_line[y]] = 'O';
                left_line[y] = left_line[y]+1
            }
            else if platform[y][x] == '#' {
                left_line[y] = x+1
            }
        }
    }
}

fn roll_south(platform: &mut Platform) {
    let platform_width = platform[0].len();
    let mut bottom_line = vec![platform.len() - 1 as usize ;platform_width];


    for y in 0..platform.len() {
        for x in 0..platform[y].len() {
            if platform[(platform_width-1) - y][x] == 'O' {
                platform[(platform_width-1) - y][x] = '.';
                platform[bottom_line[x]][x] = 'O';
                
                if bottom_line[x] != 0 {
                    bottom_line[x] = bottom_line[x]-1
                }
            }
            else if platform[(platform_width-1) - y][x] == '#' {
                if (platform_width-1) - y != 0 {
                    bottom_line[x] = (platform_width-1) - y- 1
                }
            }
        }
    }
}


fn roll_east(platform: &mut Platform) {
    let mut right_line = vec![platform[0].len() -1 as usize;platform.len()];
    let platform_height = platform.len();

    for x in 0..platform[0].len() {
        for y in 0..platform.len() {
            if platform[y][(platform_height - 1) - x] == 'O' {
                platform[y][(platform_height - 1) - x] = '.';
                platform[y][right_line[y]] = 'O';
                
                if right_line[y] != 0 {
                    right_line[y] = right_line[y]-1
                }
            }
            else if platform[y][(platform_height - 1) - x] == '#' {
                if (platform_height - 1) - x != 0 {
                    right_line[y] = (platform_height - 1) - x-1
                }
            }
        }
    }

}

fn part2 () -> usize{
    // let test =1000000000 as usize;
    // println!("{:?}", test);
    let mut platform = get_platform();
    let mut memory = HashSet::new();

    // let platform_width = platform.len();

    // memory.insert(platform.clone());
    let mut duplicate_cycles = vec![];

    // for x in 0..1000000000 {
    for x in 0..1000000000 {
        // print!("{x} ;");
        roll_north(&mut platform);
        // _print_platform(&platform);
        // println!();
        roll_west(&mut platform);
        roll_south(&mut platform);
        roll_east(&mut platform);

        if memory.contains(&platform) {
            duplicate_cycles.push(x+1);
            println!("x: {x}");
            memory = HashSet::new();
            // break;
            if duplicate_cycles.len() == 3 {
                break;
            }
        }
        
        memory.insert(platform.clone());
        
    }

    // _print_platform(&platform);
    // println!("{duplicate_cycles:?}");

    // let testing = duplicate_cycles[2] - duplicate_cycles[1];
    let remaining_cycles = (1000000000 - duplicate_cycles[2]) % (duplicate_cycles[2] - duplicate_cycles[1]) ;
    // let remaining_cycles = (100 - duplicate_cycles[2]) % (duplicate_cycles[2] - duplicate_cycles[1]) ;
    // println!("test: {testing}");
    println!("remaining_cycles: {remaining_cycles}");


    for _ in 0..remaining_cycles {
        roll_north(&mut platform);
        roll_west(&mut platform);
        roll_south(&mut platform);
        roll_east(&mut platform);
    }

    return get_load(&platform);
    // 0

}
