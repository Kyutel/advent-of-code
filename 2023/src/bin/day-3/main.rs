use std::{cmp::min, collections::HashMap};

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize
}


#[derive(Debug)]
struct SerialNumber{
    value: u32,
    location: Point,
    length: usize
}


fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}


fn check_if_valid(serial:&SerialNumber, grid: &Vec<Vec<char>>) -> bool {
    
    let start_x = if serial.location.x == 0 {
        0
    }
    else {
        serial.location.x - 1 
    };
    
    let start_y = if serial.location.y == 0 {
        0
    }
    else {
        serial.location.y - 1 
    };
    // let start_x = max(0, serial.location.x - 1);
    // let start_y = max(0, serial.location.y - 1);
    let end_x = min(grid[0].len() - 1, serial.location.x + serial.length);
    let end_y = min(grid.len() - 1, serial.location.y + 1);
    // println!("{:?}, {end_x}, {end_y} ", serial);

    for y in start_y..=end_y{
        for x in start_x..=end_x{
            if !grid[y][x].is_numeric() && grid[y][x] != '.' {
                return true
            }
        }
    }
    return false;
}


fn get_serial_numbers_in_grid(grid: &Vec<Vec<char>>) -> Vec<SerialNumber>{
    let mut serial_numbers: Vec<SerialNumber> = vec![];
    for (y, row) in grid.iter().enumerate() {
        let mut next_serial = "".to_string();

        for (x, symbol) in row.iter().enumerate() {
            if symbol.is_numeric() {
                next_serial.push(*symbol);
            }
            else if next_serial != ""{
                let length = next_serial.len();
               
                serial_numbers.push( SerialNumber{
                    value: next_serial.parse().unwrap(),
                    location: Point {x: x-length, y},
                    length
                });
                next_serial = "".to_string();
            }
        }
        if next_serial != ""{
            let length = next_serial.len();
           
            serial_numbers.push( SerialNumber{
                value: next_serial.parse().unwrap(),
                location: Point {x: grid[0].len()-1-length, y},
                length
            });
            // next_serial = "".to_string(); commenting after finishing because compiler warning 
            //when i returned after long time to work on day 6
            // will probably need to come back and fix 
        }
    }

    return serial_numbers
}

fn check_for_gear(serial:&SerialNumber, grid: &Vec<Vec<char>>)  ->  Option<Point> {
    let start_x = if serial.location.x == 0 {
        0
    }
    else {
        serial.location.x - 1 
    };
    
    let start_y = if serial.location.y == 0 {
        0
    }
    else {
        serial.location.y - 1 
    };

    let end_x = min(grid[0].len() - 1, serial.location.x + serial.length);
    let end_y = min(grid.len() - 1, serial.location.y + 1);

    for y in start_y..=end_y{
        for x in start_x..=end_x{
            if grid[y][x] == '*' {
                return Some(Point {x, y});
            }
        }
    }

    return None
}

fn part1 () -> u32 {
    let grid: Vec<Vec<char>>  = FILE_CONTENTS.split("\n").map(|row| row.chars().collect()).collect();

    let serial_numbers = get_serial_numbers_in_grid(&grid);

    // println!("{:?}", serial_numbers);
    


    // let mut valid_serial_numbers: Vec<u32> = vec![];
    return serial_numbers.into_iter().filter(|serial| check_if_valid(serial, &grid)).map(|serial| serial.value).sum()


    // println!("{:?}", symbol_coords);

    // return valid_serial_numbers.iter().sum()
}


fn part2 () -> u32 {
    let grid: Vec<Vec<char>>  = FILE_CONTENTS.split("\n").map(|row| row.chars().collect()).collect();
    let serial_numbers = get_serial_numbers_in_grid(&grid);

    let mut gear_values:HashMap<Point, Vec<SerialNumber>> = HashMap::new();

    for serial in serial_numbers {   
        if let Some(gear) = check_for_gear(&serial, &grid) {
            match gear_values.get_mut(&gear) {
                Some(gear_serials) => {
                    gear_serials.push(serial)
                }
                None => {
                    gear_values.insert(gear, vec![serial]);
                }
            }
        }
    }

    return gear_values.into_iter().filter(|(_loc, serials)| serials.len() == 2)
                .map( |(_loc, serials)| serials[0].value * serials[1].value).sum()

}
