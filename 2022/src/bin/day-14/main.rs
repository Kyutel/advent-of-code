use std::collections::HashMap;

static FILE_CONTENTS: &str = include_str!("input.txt");
// static FILE_CONTENTS: &str = include_str!("test-input.txt");

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_point_from_line(line: Vec<Vec<i32>>) -> Vec<Point> {
    let mut points = vec![];

    let mut previous_point = &line[0];

    for raw_point in line[1..].iter() {
        // println!("{:?}-{:?}",previous_point, raw_point);
        if previous_point[0] == raw_point[0] {
            let iter = if previous_point[1] <= raw_point[1] {
                previous_point[1]..raw_point[1] 
            } 
            else {
                raw_point[1]+1..previous_point[1]+1
            };
            for y in iter {
                // print!("{} ", y);
                points.push(Point {x:previous_point[0], y});
            }
        }
        else {
            let iter = if previous_point[0] <= raw_point[0] {
                previous_point[0]..raw_point[0] 
            } 
            else {
                raw_point[0]+1..previous_point[0]+1
            };

            for x in iter {
                // print!("{} ", x);
                points.push(Point {x, y:previous_point[1]});
            }
        }
        // println!();
        points.push(Point{x:raw_point[0],y:raw_point[1]});
        previous_point = raw_point;
    } 

    return points;
}

fn get_cave_map() -> HashMap<Point, char>{

    FILE_CONTENTS.split('\n')
        .flat_map(|line| get_point_from_line(
            line.split(" -> ").map(
                |raw_point| raw_point.split(',').map(|i| i.parse::<i32>().unwrap()).collect()
            ).collect()
        ))
        .map(|x| (x, '#')).collect()

}

fn get_lowest_y(cave_map: &HashMap<Point, char>) -> i32{
    let mut biggest_y = 0;
    for key in cave_map.keys() {
        biggest_y = if key.y > biggest_y {key.y} else {biggest_y};

    }

    biggest_y
}

fn print_cave_map(cave_map: &HashMap<Point, char>) {
    let mut smallest_x = i32::MAX;
    let mut smallest_y = i32::MAX;
    let mut biggest_x = 0;
    let mut biggest_y = 0;


    for key in cave_map.keys() {
        smallest_x = if key.x < smallest_x {key.x} else {smallest_x};
        smallest_y = if key.y < smallest_y {key.y} else {smallest_y};

        biggest_x = if key.x > biggest_x {key.x} else {biggest_x};
        biggest_y = if key.y > biggest_y {key.y} else {biggest_y};

    }    

    for y in (smallest_y-1)..(biggest_y+2) {
        for x in (smallest_x-1)..(biggest_x+2) {
            let point = Point{x,y};
            print!("{}", cave_map.get(&point).unwrap_or(&'.'));
        }
        println!();
    }
}

fn spawn_sand(cave_map: &mut HashMap<Point, char>, sand_spawn: Point, lowest_y: i32, floor_exists: bool)  -> bool{
    let mut sand_has_settled = false;
    let mut new_sand_position = Point {x:sand_spawn.x, y: sand_spawn.y};
    while !sand_has_settled{
        let down = Point{x:new_sand_position.x, y:new_sand_position.y + 1};
        let down_left = Point{x:new_sand_position.x - 1, y:new_sand_position.y + 1};
        let down_right = Point{x:new_sand_position.x + 1, y:new_sand_position.y + 1};
        
        if new_sand_position.y >= lowest_y && !floor_exists {
            return false
        }
        // if new_sand_position.y >= lowest_y*2 {
        //     // print_cave_map(cave_map);
        //     panic!()
        // }
        else if new_sand_position.y == (lowest_y + 2) -1  {
            sand_has_settled = true;
        }
        else if *cave_map.get(&down).unwrap_or(&'.') == '.' {
            new_sand_position = down;
        }
        else if  *cave_map.get(&down_left).unwrap_or(&'.') == '.' {
            new_sand_position = down_left;
        }
        else if  *cave_map.get(&down_right).unwrap_or(&'.') == '.' {
            new_sand_position = down_right;
        }
        else {
            sand_has_settled = true;
        }
    }
    if new_sand_position == sand_spawn{
        return false;
    }

    cave_map.insert(new_sand_position, 'o');

    true
}

fn part1() -> i32 {
    let mut cave_map = get_cave_map();
    // println!("{:?}", cave_map);
    let sand_spawn = Point {x:500,y:0};
    cave_map.insert(sand_spawn, '+');

    let mut amount_of_sand = 0;
    let lowest_y = get_lowest_y(&cave_map);
    while spawn_sand(&mut cave_map, sand_spawn, lowest_y, false) {
        amount_of_sand +=1;
    }

    print_cave_map(&cave_map);

    amount_of_sand
}

fn part2() -> i32{
    let mut cave_map = get_cave_map();
    // println!("{:?}", cave_map);
    let sand_spawn = Point {x:500,y:0};
    cave_map.insert(sand_spawn, '+');

    let mut amount_of_sand = 0;
    let lowest_y = get_lowest_y(&cave_map);
    while spawn_sand(&mut cave_map, sand_spawn, lowest_y, true) {
        amount_of_sand +=1;
    }

    // print_cave_map(&cave_map);

    amount_of_sand+1
}