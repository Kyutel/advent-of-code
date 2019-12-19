use std::collections::HashMap;

type Vault = Vec<Vec<char>>;

#[derive(Debug,Copy, Clone, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

fn main() {
    let vault = get_vault();
    print_vault(&vault);

    let key_count = get_key_count(&vault);

    let mut key_step_map = HashMap::new();
    let locations = get_starting_locations(&vault);
    // println!("{:?}", locations);
    key_step_map.insert((vec!['@'],locations.clone()),0);


    // println!("{:?}", key_step_map);
    // let mut key_list = vec!['@'];
    // let mut searched_keys = Vec::new();

    loop {
        // println!("{:?}", key_step_map);
        let mut temp_map = HashMap::new();
        for (map_key,val) in &key_step_map {
            
            let key_list = &map_key.0;
            let locations = &map_key.1;

            let mut location_index = 0;
            for location in locations {

                let mut visited = HashMap::new();
                visited.insert(*location,0);
                let keys = get_keys_from_position(*location,&key_list,&mut visited,0,&vault);
                // println!("{:?}: {:?}",key_list, keys);
                for key in keys {
                    let mut new_key_list = key_list.clone();
                    new_key_list.push(key.0);
                    new_key_list.sort();
                    let mut new_locations = locations.clone();
                    new_locations[location_index] = find_position_of(key.0,&vault);
                    let new_map_key = (new_key_list.clone(),new_locations);
                    let steps = temp_map.entry(new_map_key).or_insert(key.1 + val);
                    use std::cmp;
                    *steps = cmp::min(*steps,key.1 + val);
                }

                location_index +=1;
            }
        }

        if temp_map.len() == 0 {
            break;
        }

        key_step_map = temp_map.clone();



    }

    // println!("{:?}", key_step_map);
    


    // let mut searched_keys = Vec::new();

    // loop {
    //     let mut temp_map = key_step_map.clone();
    //     // let mut temp_map = HashMap::new();
    //     for (key_list,val) in &key_step_map {
    //         if searched_keys.contains(&key_list.clone()){
    //             continue;
    //         }
    //         searched_keys.push(key_list.clone());

    //         let location = find_position_of(key_list[key_list.len()-1], &vault);
    //         let mut visited = HashMap::new();
    //         visited.insert(location,0);
    //         let keys = get_keys_from_position(location,&key_list,&mut visited,0,&vault);
    //         // println!("{:?}: {:?}",key_list, keys);
    //         for key in keys {
    //             let mut new_key_list = key_list.clone();
    //             new_key_list.sort();
    //             new_key_list.push(key.0);

    //             let steps = temp_map.entry(new_key_list.clone()).or_insert(key.1 + *val);
    //             use std::cmp;
    //             *steps = cmp::min(*steps,key.1 + *val);
    //         }
    //     }
    //     if temp_map == key_step_map{
    //         break;
    //     }
    //     // if temp_map.len() == 0 {
    //     //     break;
    //     // }
    //     key_step_map = temp_map;
    //     // key_step_map = temp_map.clone();
    // }

    // println!("{:?}", key_step_map);

    let mut answer = std::i32::MAX;
    for (key_list,val) in &key_step_map {
        if key_list.0.len() as i32 == key_count + 1{
            use std::cmp;
            answer = cmp::min(answer,*val);
        } 
    }

    println!("{:?}", answer);
    



}

fn get_keys_from_position(
        position: (usize,usize),
        current_key_list:&Vec<char>,
        travel_list: &mut HashMap<(usize,usize),i32>,
        steps: i32,
        vault: &Vault ) -> Vec<(char,i32)> {
    use Direction::*;
    let direction_list = [UP,DOWN,LEFT,RIGHT];
    let mut new_key_list = Vec::new();

    for dir in direction_list.iter() {

        let next_location = check_direction(*dir,position,&vault);
        let test_step = travel_list.get(&next_location.0);
        if  test_step != None{
            if test_step.unwrap() < &(steps + 1){
                continue;      
            }
        }

        if next_location.1 != '#'
        {
            travel_list.insert(next_location.0,steps + 1);
            // println!("last:{:?}, next: {:?}", position,next_location);
            // println!("{:?}, {:?}",Some(get_oppasite_direction(*dir)), last_dir );
            if next_location.1.is_ascii_lowercase()  && !current_key_list.contains(&next_location.1){
                new_key_list.push((next_location.1,steps + 1));
                // return new_key_list;
            }
            else {
                if next_location.1.is_ascii_uppercase() {
                    if !current_key_list.contains(&next_location.1.to_ascii_lowercase()){
                        continue;
                    }
                }

                new_key_list.append(&mut get_keys_from_position(next_location.0,current_key_list,travel_list,steps + 1,vault));
            }
        }
    }

    new_key_list
}

fn check_direction(direction:Direction, current_location: (usize,usize), vault:&Vault) -> ((usize,usize),char) {
    let mut check_location = current_location;

    match direction {
        Direction::UP => check_location.1 -= 1,
        Direction::DOWN => check_location.1 += 1,
        Direction::LEFT => check_location.0 -= 1,
        Direction::RIGHT => check_location.0 += 1,
    };

    (check_location,vault[check_location.1][check_location.0])
}

// fn get_oppasite_direction(direction:Direction) -> Direction {
//     let new_direction;

//     match direction {
//         Direction::UP => new_direction = Direction::DOWN,
//         Direction::DOWN => new_direction = Direction::UP,
//         Direction::LEFT => new_direction = Direction::RIGHT,
//         Direction::RIGHT => new_direction = Direction::LEFT,
//     };

//     new_direction
// }

fn find_position_of(char_to_find:char,vault: &Vault) -> (usize,usize) {
    for y in 0..vault.len(){
        for x in 0..vault[y].len(){
            if vault[y][x] == char_to_find {
                return (x,y);
            }
        }
    }

    (0,0)
}

fn get_starting_locations(vault: &Vault) -> Vec<(usize,usize)>{
    let mut starting_locations = Vec::new();

    for y in 0..vault.len(){
        for x in 0..vault[y].len(){
            if vault[y][x] == '@' {
                starting_locations.push((x,y));
            }
        }
    }

    starting_locations
}

fn get_key_count(vault: &Vault) -> i32 {
    let mut key_count = 0;

    for y in vault {
        for x in y {
            if x.is_ascii_lowercase() {
                key_count +=1;
            }
        }

    }
    key_count
}

fn print_vault(vault: &Vault) {
    for y in vault {
        for x in y {
            print!("{}", x);
        }
        println!();
    }
}

fn get_vault()  -> Vault{
    include_str!("data2.txt").split("\r\n")
    .map(|row| row.chars().collect())
    .collect()
}