// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");



#[derive(Debug, Clone, Copy,/* Hash, PartialEq, Eq*/)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
struct Plan {
    dir: Direction,
    meters: i64,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_direction_from_string(dir: &str) -> Direction {
    match dir {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!()
    }
}

fn get_direction_from_int(dir_no: i64) -> Direction {
    match dir_no {
        3 => Direction::Up,
        1 => Direction::Down,
        2 => Direction::Left,
        0 => Direction::Right,
        _ => panic!()
    }
}


fn get_plans() -> Vec<Plan>{
    FILE_CONTENTS.split('\n').map(|line| {
        let line: Vec<&str> = line.split_whitespace().collect();
        Plan {  
            dir: get_direction_from_string(line[0]),
            meters: line[1].parse().unwrap(),
        }
    }).collect()
}


fn get_plans_from_hex() -> Vec<Plan>{
    FILE_CONTENTS.split('\n').map(|line| {
        let line: Vec<&str> = line.split_whitespace().collect();
        Plan {  
            dir: get_direction_from_int(line[2][7..8].parse().unwrap()),
            meters: i64::from_str_radix(&line[2][2..7], 16).unwrap()
        }
            // colour: i64::from_str_radix(&line[2][2..line[2].len()-1], 16).unwrap()}
    }).collect()
}

fn get_next_point(current_point: Point, dir: Direction, distance: i64) -> Point {
    match dir {
        Direction::Up => Point { x: current_point.x, y: current_point.y - distance },
        Direction::Down => Point { x: current_point.x, y: current_point.y + distance },
        Direction::Left => Point { x: current_point.x - distance, y: current_point.y},
        Direction::Right => Point { x: current_point.x + distance, y: current_point.y}
    }
}

fn follow_map_plan(plans: Vec<Plan>) -> (Vec<Point>, i64){
    let mut points = vec![];
    let mut current_point = Point{ x:0, y:0};

    let mut perimeter = 0;

    for plan in plans {

        let next_point = get_next_point(current_point, plan.dir, plan.meters);
        perimeter += plan.meters;


        points.push(next_point);
        current_point = next_point;
    }


    (points, perimeter)
}

fn find_area_inside_points(points: Vec<Point>) -> i64 {
    let mut top_sum = 0;

    let mut j = points.len() - 1;
    for i in 0..points.len() {
        top_sum += (points[j].x + points[i].x) * ((points[j].y - points[i].y) );
        j = i;
    }

    return top_sum/2;
}




fn part1 ()  -> i64 {
    let plans = get_plans();
    // println!("{:?}", plans);
    let (points, permiter) = follow_map_plan(plans);


    // use picks theorem or something
    find_area_inside_points(points).abs() - (permiter/2) + 1 + permiter
}

fn part2 () -> i64{
    let plans = get_plans_from_hex();
    // println!("{:?}", plans);
    let (points, permiter) = follow_map_plan(plans);


    // use picks theorem or something
    // find_area_inside_points(points).abs() - (permiter/2) + 1 + permiter
    find_area_inside_points(points).abs() + (permiter/2) + 1
}
