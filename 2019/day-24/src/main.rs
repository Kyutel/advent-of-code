use std::collections::HashSet;

type State = Vec<Vec<char>>;


fn main() {
    // part_1();
    part_2();
}


fn part_2() {
    let mut state = get_data();
    state[2][2] = '?';
    // print_state(&state);
    let mut eris = vec![state];

    for _i in 0..200 {
        eris = get_new_levels(&eris);
        eris = get_next_eris_state(&eris);
    }

    // print_eris(&eris);
    println!("{:?}", get_total_bug_count(&eris));

}

fn part_1() {
   let mut state = get_data();
   // println!("{:?}", state);
   let mut previous_states = HashSet::new();
   previous_states.insert(state.clone());
   loop {
       state = get_next_state(&state);
       if previous_states.contains(&state){
           break;
       }
       else {
           previous_states.insert(state.clone());
       }
   }

   print_state(&state);
   println!("biodiversity rating: {:?}", get_rating(&state)); 
}

fn get_total_bug_count(eris: &Vec<State>) -> i32 {
    let mut total_bug_count = 0;
    for levels in eris {
        for y in levels {
            for x in y {
                if *x == '#' {
                    total_bug_count += 1;
                }
            }
        }
    }


    total_bug_count
}

fn get_next_eris_state(eris: &Vec<State>) -> Vec<State> {
    let mut next_eris_state = eris.clone();

    for (level,level_state) in eris.iter().enumerate() {
        for y in 0..5 {
            for x in 0..5 {
                if y == 2 && x ==2 {
                    continue;
                }
                let bug_count = get_adjacent_bug_count_v2((x,y),level,eris);
                next_eris_state[level][y][x] = get_new_cell(eris[level][y][x],bug_count);
            }
        }
    }

    next_eris_state
}

fn get_new_levels(eris: &Vec<State>) -> Vec<State> {
    let mut new_eris = eris.clone();

    for (level,level_state) in eris.iter().enumerate() {
        let mut is_lower_level = false;
        let mut is_upper_level = false;

        for y in 0..5 {
            for x in 0..5 {
                if x == 0 || y == 0 || x == 4 || y==4 {
                    if level_state[y][x] == '#' {
                        is_lower_level = true;
                    }
                }
            }
        }

        if level_state[1][2] == '#' || level_state[2][1] == '#' || level_state[2][3] == '#' || level_state[3][2] == '#' {
            is_upper_level = true;
        }

        let mut blank_level = vec![vec!['.';5];5];
        blank_level[2][2] = '?';


        if level == 0 {
            if is_lower_level {
                new_eris.insert(0,blank_level.clone());
            }
        }

        if level == eris.len() - 1{
            if is_upper_level {
                // let offset = if is_upper_level {2} else {1};
                new_eris.insert(new_eris.len(),blank_level.clone());
            }   
        }

    }

    // print_eris(&new_eris);

    new_eris
}


fn get_rating(state: &State) -> i32 {
    let mut rating = 0;

    for y in 0..state.len() {
        for x in 0..state[y].len() {
            if state[y][x] == '#' {
                let pow = (y*state[y].len() + x) as u32;
                rating += 2i32.pow(pow);
            }
        }
    }

    rating
}

fn get_new_cell(curent_cell:char, bug_count:i32) -> char{
    if curent_cell == '.' {
        if bug_count == 1 || bug_count ==2 {
            '#'
        }
        else {
            '.'
        }  
    }else {
        if bug_count == 1 {
            '#'
        }
        else {
            '.'
        }  
    }
}

fn get_next_state(state: &State) -> State {
    let mut next_state = vec![vec!['x';state[0].len()];state.len()];

    for y in 0..state.len() {
        for x in 0..state[y].len() {
            let bug_count = get_adjacent_bug_count((x,y),state);
            next_state[y][x] = get_new_cell(state[y][x],bug_count);
        }
    }

    next_state
}

fn get_adjacent_bug_count(cell:(usize,usize), state:&State) -> i32 {
    let mut count = 0;
    let x = cell.0;
    let y = cell.1;
    if x!=0 {
        if state[y][x-1] == '#' {
            count+=1;
        }
    }

    if y!=0 {
        if state[y-1][x] == '#' {
            count+=1;
        }
    }

    if x != state[y].len() -1 {
        if state[y][x+1] == '#' {
            count+=1;
        }
    }

    if y != state.len() -1 {
        if state[y+1][x] == '#' {
            count+=1;
        }
    }

    count
}


fn get_adjacent_bug_count_v2(cell:(usize,usize),level:usize, eris:&Vec<State>) -> i32{
    let mut count = 0;
    let x = cell.0;
    let y = cell.1;

    //check left
    if x == 0 {
        if level!= 0 {
            if eris[level - 1][2][1] == '#'{
                count+=1;
            }
        }
    }
    else if x == 3 && y ==2 {
        if level != eris.len() - 1 {
            for upper_y in 0..5 {
                if eris[level + 1][upper_y][4] == '#' {
                    count+=1
                }
            }
        }
    }
    else {
        if eris[level][y][x-1] == '#' {
            count+=1;
        }
    }

    //check right
    if x == 4 {
        if level!= 0 {
            if eris[level - 1][2][3] == '#'{
                count+=1;
            }
        }
    }
    else if x == 1 && y == 2 {
        if level!= eris.len() - 1 {
            for upper_y in 0..5 {
                if eris[level + 1][upper_y][0] == '#' {
                    count+=1
                }
            }
        }
    }
    else {
        if eris[level][y][x+1] == '#' {
            count+=1;
        }
    }

    //check up
    if y == 0{
        if level!= 0 {
            if eris[level - 1][1][2] == '#'{
                count+=1;
            }
        }
    }
    else if y == 3 && x == 2{
        if level!= eris.len() - 1 {
            for upper_x in 0..5 {
                if eris[level + 1][4][upper_x] == '#' {
                    count+=1
                }
            }
        }
    }
    else {
        if eris[level][y-1][x] == '#' {
            count+=1;
        }
    }

    //check down
    if y == 4{
        if level!= 0 {
            if eris[level - 1][3][2] == '#'{
                count+=1;
            }
        }
    }
    else if y == 1 && x == 2{
        if level!= eris.len() - 1 {
            for upper_x in 0..5 {
                if eris[level + 1][0][upper_x] == '#' {
                    count+=1
                }
            }
        }
    }
    else {
        if eris[level][y+1][x] == '#' {
            count+=1;
        }
    }


    count
}

fn print_state(state: &State) {
    for y in state {
        for x in y {
            print!("{}", x);
        }
        println!();
    }
}

fn print_eris(eris: &Vec<State>) {
    let mut level:i32 = 0 - (eris.len()/ 2) as i32;

    for level_state in eris {
        println!("Level {:?}", level);
        print_state(level_state);
        println!();
        level+=1;
    }
}

fn get_data()  -> State{
    include_str!("data.txt").split("\r\n")
    .map(|row| row.chars().collect())
    .collect()
}