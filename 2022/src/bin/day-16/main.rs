use std::collections::HashSet;

use regex::Regex;

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

#[derive(Debug, Hash, PartialEq, Eq)]
struct Valve {
    id: String,
    index: usize,
    flow_rate: i32,
    tunnels: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
struct Opener {
    position: usize,
    state: OpenerState,
    next_position: Option<usize>,
    steps_to_next_position: i32
}

#[derive(Debug, Clone, Copy)]
enum OpenerState {
    Nothing,
    Opening,
    Moving,
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}


impl Opener {
    fn update_self(&mut self)  -> bool{
        match self.state {
            OpenerState::Moving => {
                self.steps_to_next_position -=1;                
                if self.steps_to_next_position == 0 {
                    self.state = OpenerState::Opening;
                    self.position = self.next_position.unwrap();
                    self.next_position = None;
                }

                false
            },
            OpenerState::Opening => {
                self.state = OpenerState::Nothing; 
                true
            },
            OpenerState::Nothing => false,
        }
    }

    fn set_next(&self, valve: usize, distance: i32) -> Opener {
        Opener { position: self.position, state: OpenerState::Moving, next_position: Some(valve), steps_to_next_position: distance }
    }
}

fn parse_valve_from_line(input_line: &str, index: usize, regex: &Regex) -> Valve{
    let re_matches: Vec<String> = regex.find_iter(input_line).map(|x| x.as_str().to_string()).collect();
    
    let id = re_matches[0].to_owned();
    let flow_rate = re_matches[1].parse().unwrap();
    let tunnels = re_matches[2..].to_vec();

    Valve {id, flow_rate, index, tunnels}
}

fn get_valves_from_input() -> Vec<Valve>{
    let re = Regex::new(r"([A-Z]{2})|(\d+)").unwrap();
    
    FILE_CONTENTS.split("\n")
    .enumerate()
    .map(|(i, x)| parse_valve_from_line(x,i, &re)).collect()
}


fn get_index_for_valve(id: &String, valves: &Vec<Valve>) -> usize {
    for (index,valve) in valves.iter().enumerate(){
        if &valve.id == id {
            return index;
        }
    } 
    0
}

fn find_shortest_distances(valves: &Vec<Valve>) -> Vec<Vec<i32>>{
    let mut distances = vec![vec![10000;valves.len()];valves.len()];
    // let mut path = vec![vec![vec![];valves.len()];valves.len()];

    for (index, valve) in valves.iter().enumerate() {
        for neighbour in valve.tunnels.iter() {
            let neighbour_index = get_index_for_valve(&neighbour, &valves);
            distances[index][neighbour_index] = 1;
            // path[index][neighbour_index].push(neighbour.clone());
        }
        distances[index][index] = 0;
    }

    // println!("{:?}", distances);

    for x in 0..valves.len() {
        for y in 0..valves.len() {
            for z in 0..valves.len() {
                if distances[y][z] > distances[y][x] + distances[x][z] {
                    distances[y][z] = distances[y][x] + distances[x][z];
                    // let mut new_path = path[y][x].clone();
                    // new_path.extend(path[x][z].clone());
                    // path[y][z] = new_path; 
                }
            }
        }
    }

    distances
}


fn get_most_pressure_released(current_valve: usize, ppm:i32, total_pressure:i32, time_remaining: i32, distances: &Vec<Vec<i32>>, 
    remaining_valves: HashSet<&Valve>
) -> i32 {
    if time_remaining < 0 {
        panic!();
    }

    let mut most = total_pressure + (ppm * time_remaining);
    for valve in remaining_valves.iter() {

        let time_to_move_and_open = distances[current_valve][valve.index] + 1; 
        let time_calc = time_remaining - time_to_move_and_open;
        if time_calc < 1 {
            continue;
        }

        let pressure_calc = total_pressure + (ppm * time_to_move_and_open);
        let next_ppm = ppm + valve.flow_rate;
        let mut next_remaining_valves = remaining_valves.clone();
        next_remaining_valves.remove(valve);

        let next = get_most_pressure_released(
            valve.index, next_ppm, pressure_calc, time_calc, distances,
            next_remaining_valves
        );

        if next > most {
            most = next;
        }
    }

    return most

}
  


fn get_most_pressure_released_with_elephant(mut me: Opener, mut elephant:Opener, ppm:i32, total_pressure:i32, time_remaining: i32, 
    distances: &Vec<Vec<i32>>, valves: &Vec<Valve>, remaining_valves: HashSet<&Valve>
) -> i32 {
    if time_remaining < 0 {
        panic!();
    }
    // if time_remaining == 20 {
        // if me.position == 2
        // && elephant.position == 4 && elephant.next_position == None {
        //     println!("me: {:?}", me);
        //     println!("e : {:?}", elephant);
        //     println!("{}, {}, {:?}", ppm, total_pressure, remaining_valves.iter().map(|x|x.id.clone()).collect::<Vec<String>>());            
        //     println!();
        // }
        // println!("{}",time_remaining);
    // }
    if time_remaining == 0 {
        return total_pressure;
    }
    
    
    // let next_total = total_pressure + ppm;
    // let next_time = time_remaining - 1;
    
    let most = total_pressure + (ppm * time_remaining);
    let next = match (me.state, elephant.state) {
        (OpenerState::Nothing, OpenerState::Nothing) =>  {
            let mut temp_most = most;

            if remaining_valves.len() == 0{
                return most;
            }

            let remaining_iter: Vec<&Valve> = remaining_valves.iter().cloned().collect();

            if remaining_iter.len() == 1{

                let d1 = distances[me.position][remaining_iter[0].index];
                let d2 =  distances[elephant.position][remaining_iter[0].index];

                if d1 < d2 {
                    let next_me = me.set_next(remaining_iter[0].index, d1);
                    let next_elephant = elephant.clone();
                    let mut next_remaining = remaining_valves.clone();
                    next_remaining.remove(remaining_iter[0]);
                    let other = get_most_pressure_released_with_elephant(next_me, next_elephant, ppm, total_pressure, time_remaining, distances, valves,next_remaining );
                    return i32::max(temp_most, other);
                }
                else {
                    let next_me = me.clone();
                    let next_elephant = elephant.set_next(remaining_iter[0].index, d2);
                    let mut next_remaining = remaining_valves.clone();
                    next_remaining.remove(remaining_iter[0]);
                    let other = get_most_pressure_released_with_elephant(next_me, next_elephant, ppm, total_pressure, time_remaining, distances, valves,next_remaining );
                    return i32::max(temp_most, other);
                }


            }


            for valve1 in remaining_iter.iter() {
                for valve2 in remaining_iter.iter() {
                    if valve1 == valve2 {
                        continue;
                    }

                    let d1 = distances[me.position][valve1.index];
                    let d2 =  distances[elephant.position][valve2.index];

                    let next_me = me.set_next(valve1.index, d1);
                    let next_elephant = elephant.set_next(valve2.index, d2);
                    let mut next_remaining = remaining_valves.clone();
                    next_remaining.remove(valve1);
                    next_remaining.remove(valve2);

                    let next = get_most_pressure_released_with_elephant(next_me, next_elephant, ppm, total_pressure, time_remaining, distances, valves,next_remaining );
                    if next > temp_most {
                        temp_most = next;
                    }
                }
            }
            temp_most
        },
        (OpenerState::Nothing, _) => {
            let mut temp_most = most;
            
                if remaining_valves.len() == 0{
                    let mut next_ppm = ppm;
                    if me.update_self() {
                        next_ppm +=  valves[me.position].flow_rate
                    }
                    
                    if elephant.update_self() {
                        next_ppm += valves[elephant.position].flow_rate
                    }
                    let next_total = total_pressure + next_ppm;
                    let next_time = time_remaining - 1;
                    return get_most_pressure_released_with_elephant(me, elephant, next_ppm, next_total, next_time, distances, valves,remaining_valves )
                }

            for valve in remaining_valves.iter(){
                let d = distances[me.position][valve.index];

                let next_me = me.set_next(valve.index, d);
                let next_elephant = elephant.clone();
                let mut next_remaining = remaining_valves.clone();
                next_remaining.remove(valve);

                let next = get_most_pressure_released_with_elephant(next_me, next_elephant, ppm, total_pressure, time_remaining, distances, valves,next_remaining );
                if next > temp_most {
                    temp_most = next;
                }
            }
            temp_most
        },
        (_, OpenerState::Nothing) => {
            let mut temp_most = most;

            if remaining_valves.len() == 0{
                let mut next_ppm = ppm;
                if me.update_self() {
                    next_ppm +=  valves[me.position].flow_rate
                }
                
                if elephant.update_self() {
                    next_ppm += valves[elephant.position].flow_rate
                }
                let next_total = total_pressure + next_ppm;
                let next_time = time_remaining - 1;
                return get_most_pressure_released_with_elephant(me, elephant, next_ppm, next_total, next_time, distances, valves,remaining_valves )
            }
            
            for valve in remaining_valves.iter(){
                let d = distances[elephant.position][valve.index];
                let next_me = me.clone();
                let next_elephant = elephant.set_next(valve.index, d);
                let mut next_remaining = remaining_valves.clone();
                next_remaining.remove(valve);

                let next = get_most_pressure_released_with_elephant(next_me, next_elephant, ppm, total_pressure, time_remaining, distances, valves,next_remaining );
                if next > temp_most {
                    temp_most = next;
                }
            }
            temp_most

        },
        _ =>  {
            let mut next_ppm = ppm;
            if me.update_self() {
                next_ppm +=  valves[me.position].flow_rate
            }
            
            if elephant.update_self() {
                next_ppm += valves[elephant.position].flow_rate
            }
            let next_total = total_pressure + next_ppm;
            let next_time = time_remaining - 1;
            get_most_pressure_released_with_elephant(me, elephant, next_ppm, next_total, next_time, distances, valves,remaining_valves )
        }
    };
    next
}

fn part1() -> i32 {
    let valves = get_valves_from_input();
    
    let valves_with_pressure = valves.iter().filter(|v| v.flow_rate > 0).collect();
    let distances= find_shortest_distances(&valves);
    let starting_index = get_index_for_valve(&"AA".to_string(), &valves);


    let time_remaining = 30;
    let pressure_released = get_most_pressure_released(
        starting_index, 0, 0, time_remaining, &distances,
        valves_with_pressure
    );
    pressure_released
}

fn part2() -> i32{
    let valves = get_valves_from_input();
    
    let valves_with_pressure:HashSet<&Valve> = valves.iter().filter(|v| v.flow_rate > 0).collect();
    // let valves_with_pressure:Vec<&Valve> = valves_with_pressure.iter().cloned().collect();
    // let valves_with_pressure:HashSet<&Valve> = valves_with_pressure[5..].into_iter().cloned().collect();
    // println!("how many {} ", valves_with_pressure.len());
    let distances = find_shortest_distances(&valves);
    // for distance in distances.iter() {
    //     println!("{:?}", distance)
    // }
    let starting_index = get_index_for_valve(&"AA".to_string(), &valves);

    let time = 25;

    let me = Opener {position:starting_index, state: OpenerState::Nothing, next_position:None, steps_to_next_position: 0};
    let elephant = Opener {position:starting_index, state: OpenerState::Nothing, next_position:None, steps_to_next_position: 0};

    // let pressure = 0;
    get_most_pressure_released_with_elephant(me, elephant, 0, 0, time, &distances, &valves, valves_with_pressure)
}