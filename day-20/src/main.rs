use std::collections::HashMap;
type Donut = Vec<Vec<char>>;

#[derive(Debug,Copy, Clone, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

fn main() {
    let donut = get_donut();
    print_donut(&donut);

    let warp_points = get_warp_points(&donut);
    // println!();
    // println!("{:?}", warp_points);
    // print_donut(&new_donut);

    // let test = check_direction(Direction::LEFT,(2,8),warp_points,&donut);
    // println!("{:?}", test);
    let starting_position = warp_points.get(&"AA".to_string()).unwrap();
    let starting_position = starting_position[0];
    let destination = warp_points.get(&"ZZ".to_string()).unwrap();
    let destination = destination[0];

    let mut travel_list = HashMap::new();
    travel_list.insert((starting_position,0),0);
    let shortest_path = find_shortest_path(starting_position,destination,0,&mut travel_list,&warp_points,0,&donut);

    println!("{:?}", shortest_path);
}

fn find_shortest_path(
        position:(usize,usize),
        destination: (usize,usize),
        steps:i32,
        travel_list: &mut HashMap<((usize,usize),i32),i32>,
        warp_points: &HashMap<String,Vec<(usize,usize)>>,
        level:i32,
        donut: &Donut ) -> (bool,i32){

    use Direction::*;
    let direction_list = [UP,DOWN,LEFT,RIGHT];
    // let min_steps_taken = 1000000;

    // println!("{:?}, {:?}", position,level);

    let mut shortest_path = (false,std::i32::MAX);
    for dir in direction_list.iter() {
        let mut new_level = level;
        let next_location = check_direction(*dir,position,&warp_points,&mut new_level,&donut);

        if new_level > warp_points.len() as i32 {
            continue;
        }

        let test_step = travel_list.get(&(next_location.0,new_level));
        if  test_step != None{
            // let step_level = test_step.unwrap();
            continue;
            // else if step_level > new_level
            // if test_step.unwrap() < &(steps + 1){
            //     continue;      
            // }
        }


        if next_location.1 == '.'
        {
            travel_list.insert((next_location.0,new_level),steps + 1);
            if next_location.0 == destination  && new_level == 0 {
                return (true,steps + 1);
            }

            let temp_sp = find_shortest_path(next_location.0,destination,steps + 1,travel_list,&warp_points,new_level,&donut);
            if temp_sp.0 {
                use std::cmp;
                shortest_path = (true,cmp::min(shortest_path.1,temp_sp.1));
            }

        }


    }

    // if shortest_path == (false,std::i32::MAX) {
    //     println!("dead end at {:?}",position);
    // }


    shortest_path
}

fn check_direction(direction:Direction,
        current_location: (usize,usize),
        warp_points: &HashMap<String,Vec<(usize,usize)>>,
        level: &mut i32,
        donut:&Donut) -> ((usize,usize),char) {

    let mut check_location = current_location;

    match direction {
        Direction::UP => check_location.1 -= 1,
        Direction::DOWN => check_location.1 += 1,
        Direction::LEFT => check_location.0 -= 1,
        Direction::RIGHT => check_location.0 += 1,
    };


    if donut[check_location.1][check_location.0].is_ascii_alphabetic() {
        let mut warp_str = String::new();
        warp_str.push(donut[check_location.1][check_location.0]);
        match direction {
            Direction::UP => check_location.1 -= 1,
            Direction::DOWN => check_location.1 += 1,
            Direction::LEFT => check_location.0 -= 1,
            Direction::RIGHT => check_location.0 += 1,
        };

        match direction {
            Direction::UP => warp_str.insert(0,donut[check_location.1][check_location.0]),
            Direction::DOWN => warp_str.push(donut[check_location.1][check_location.0]),
            Direction::LEFT => warp_str.insert(0,donut[check_location.1][check_location.0]),
            Direction::RIGHT => warp_str.push(donut[check_location.1][check_location.0]),
        };

        // println!("{:?}", warp_str);
        let warp_spots = warp_points.get(&warp_str).unwrap();
        if warp_str == "AA".to_string() || warp_str == "ZZ".to_string() {
            warp_spots[0];
        }
        else {
            if current_location == warp_spots[0] 
            {
                if *level > 0 {
                   check_location = warp_spots[1];
                   *level -=1 
                }
                else {
                    check_location = warp_spots[0];
                    // *level +=1 
                }
            }
            else {
                check_location = warp_spots[0];
                *level +=1 
            }
        }


    }

    (check_location,donut[check_location.1][check_location.0])
}

// fn get_warp_location()

fn get_warp_points(donut: &Donut) -> HashMap<String,Vec<(usize,usize)>> {
    let mut warp_points = HashMap::new();

    //search top and bot row
    for x in 0..donut[0].len() {
        if donut[0][x] != ' ' {
            let mut warp_str = String::new();
            warp_str.push(donut[0][x]);
            warp_str.push(donut[1][x]);
            warp_points.insert(warp_str.clone(),vec![(x,2)]);
            // println!("{:?}",warp_str);
        }

        let second_to_last_row = donut.len() - 2;

        if donut[second_to_last_row][x] != ' ' {
            let mut warp_str = String::new();
            warp_str.push(donut[second_to_last_row][x]);
            warp_str.push(donut[second_to_last_row + 1][x]);
            warp_points.insert(warp_str.clone(),vec![(x,second_to_last_row -1)]);
            // println!("{:?}", warp_str);
        }
    }
    println!();

    //search left and right side
    for y in 0..donut.len() {
        if donut[y][0] != ' '{
            let mut warp_str = String::new();
            warp_str.push(donut[y][0]);
            warp_str.push(donut[y][1]);
            warp_points.insert(warp_str.clone(),vec![(2,y)]);
            // println!("{:?}", warp_str);
        }
        let second_to_last_column = donut[y].len() - 2;
        if donut[y][second_to_last_column] != ' '{
            let mut warp_str = String::new();
            warp_str.push(donut[y][second_to_last_column]);
            warp_str.push(donut[y][second_to_last_column + 1]);
            warp_points.insert(warp_str.clone(),vec![(second_to_last_column - 1,y)]);
            // println!("{:?}", warp_str);  
        }
    }

    let inner_ring_locations = find_inner_ring(&donut);
    // println!("{:?}", inner_ring_locations);

    for x in inner_ring_locations[0].0..inner_ring_locations[1].0 {
        if donut[inner_ring_locations[0].1][x] != ' ' {
            let mut warp_str = String::new();
            warp_str.push(donut[inner_ring_locations[0].1][x]);
            warp_str.push(donut[inner_ring_locations[0].1 + 1][x]);
            warp_points.entry(warp_str.clone())
                .and_modify(|loc| { loc.push((x,inner_ring_locations[0].1 - 1)) });
            // println!("{:?}",warp_str);
        }

        let last_row = inner_ring_locations[2].1;

        if donut[last_row][x] != ' ' {
            let mut warp_str = String::new();
            warp_str.push(donut[last_row -1][x]);
            warp_str.push(donut[last_row][x]);
            warp_points.entry(warp_str.clone())
                .and_modify(|loc| { loc.push((x,last_row + 1)) });
            // println!("{:?}", warp_str);
        }
    }

    for y in inner_ring_locations[0].1..inner_ring_locations[2].1 {
        if donut[y][inner_ring_locations[0].0] != ' '{
            let mut warp_str = String::new();
            warp_str.push(donut[y][inner_ring_locations[0].0]);
            warp_str.push(donut[y][inner_ring_locations[0].0 + 1]);
            warp_points.entry(warp_str.clone())
                .and_modify(|loc| { loc.push((inner_ring_locations[0].0 - 1,y)) });
            // println!("{:?}", warp_str);
        }
        let second_to_last_column = inner_ring_locations[1].0 - 2;
        if donut[y][second_to_last_column] != ' '{
            let mut warp_str = String::new();
            warp_str.push(donut[y][second_to_last_column]);
            warp_str.push(donut[y][second_to_last_column + 1]);
            warp_points.entry(warp_str.clone())
                .and_modify(|loc| { loc.push((second_to_last_column + 2,y)) });
            // println!("{:?}", warp_str);
        }
    }

    warp_points
}


fn find_inner_ring(donut: &Donut) -> Vec<(usize,usize)> {
    let mut inner_ring_locations = Vec::new();
    let mut top_left = (0,0);
    let mut top_left_found = false;

    for y in 2..donut.len() -2{
        for x in 2..donut[y].len() -2 {
            if donut[y][x] == ' ' {
                top_left = (x,y);
                top_left_found = true;
                break;
            }
        }
        if top_left_found {
            break;
        }
    }

    inner_ring_locations.push(top_left);
    inner_ring_locations.push((donut[0].len() - top_left.0,top_left.1));
    inner_ring_locations.push((top_left.0,donut.len() - top_left.1 - 1));


    inner_ring_locations
}

fn print_donut(donut: &Donut) {
    for y in donut {
        for x in y {
            print!("{}", x);
        }
        println!();
    }
}

fn get_donut()  -> Donut{
    include_str!("data.txt").split("\r\n")
    .map(|row| row.chars().collect())
    .collect()
}