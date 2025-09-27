use std::collections::HashMap;

// use log::{debug, trace};
// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
struct SpringRow {
    springs: Vec<char>,
    layout: Vec<i64>
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}


fn get_spring_conditions() -> Vec<SpringRow> {
    FILE_CONTENTS.split("\n").map(|line|  {
        let split_line: Vec<String> = line.split_whitespace().map(|x| x.to_string()).collect();
        let springs = split_line[0].chars().collect();
        let layout: Vec<i64> = split_line[1].split(',').map(| number| number.parse::<i64>().unwrap()).collect();

        SpringRow { springs, layout }
    }).collect()
}

fn fill_gaps(remaining_springs: &Vec<char>, remaining_layout: &Vec<i64>)  -> i64{
    if remaining_layout.len() == 0 {
        if remaining_springs.contains(&'#') {
            return  0;
        }
        return 1;
    }

    if remaining_springs.len() == 0  {
        return 0;
    }


    let next_layout = remaining_layout[0] as usize;
    if next_layout > remaining_springs.len() {
        return 0;
    }

    let mut gap = None;

    for index in  0..(remaining_springs.len() - next_layout + 1) {
        let next_slice = &remaining_springs[index..index+next_layout];
        if next_slice[0]=='#' && next_slice.contains(&'.') {
            return  0;
        }
        if !next_slice.contains(&'.') {
            if !next_slice.contains(&'?') && index+next_layout < remaining_springs.len() && remaining_springs[index+next_layout] == '#' {
                return 0
            }

            if index+next_layout < remaining_springs.len() 
                && remaining_springs[index+next_layout] != '#' {
                gap = Some(index);
                break;
            } 
            else if remaining_layout.len() == 1 && index+next_layout == remaining_springs.len() {
                gap = Some(index);
                break;
            }

            if next_slice[0]=='#' && remaining_springs[index+next_layout..].contains(&'#') {
                return 0
            }
        }
    }

    if gap == None {
        return 0;
    }

    let gap = gap.unwrap();
    let mut valid_routes = 0;

    if remaining_springs[gap] ==  '?' {
        // let next_springs = remaining_springs[gap+1..].iter().cloned().collect();
        let mut next_springs = remaining_springs.clone();
        next_springs[gap] = '.';
        valid_routes += fill_gaps(&next_springs, &remaining_layout);
    }


    let filled_gap = gap+next_layout;
    let mut filled_gap_springs: Vec<char> = remaining_springs[filled_gap..].iter().cloned().collect();
    if !filled_gap_springs.is_empty() && filled_gap_springs[0] == '?' {
        filled_gap_springs[0] = '.'
    }

    let filled_gap_layout = remaining_layout[1..].iter().cloned().collect();

    valid_routes += fill_gaps(&filled_gap_springs, &filled_gap_layout);

    return valid_routes
    
}


fn fill_gaps_with_memory(remaining_springs: &Vec<char>, remaining_layout: &Vec<i64>, memory: &mut HashMap<(Vec<char>, Vec<i64>), i64> )  -> i64{
    
    if let Some(value) = memory.get(&(remaining_springs.to_vec(), remaining_layout.to_vec())) {
        // println!("used memory");
        return *value;
    }
    

    if remaining_layout.len() == 0 {
        if remaining_springs.contains(&'#') {
            return  0;
        }
        return 1;
    }

    if remaining_springs.len() == 0  {
        return 0;
    }


    let next_layout = remaining_layout[0] as usize;
    if next_layout > remaining_springs.len() {
        return 0;
    }

    let mut gap = None;

    for index in  0..(remaining_springs.len() - next_layout + 1) {
        let next_slice = &remaining_springs[index..index+next_layout];
        if next_slice[0]=='#' && next_slice.contains(&'.') {
            return  0;
        }
        if !next_slice.contains(&'.') {
            if !next_slice.contains(&'?') && index+next_layout < remaining_springs.len() && remaining_springs[index+next_layout] == '#' {
                return 0
            }

            if index+next_layout < remaining_springs.len() 
                && remaining_springs[index+next_layout] != '#' {
                gap = Some(index);
                break;
            } 
            else if remaining_layout.len() == 1 && index+next_layout == remaining_springs.len() {
                gap = Some(index);
                break;
            }

            if next_slice[0]=='#' && remaining_springs[index+next_layout..].contains(&'#') {
                return 0
            }
        }
    }

    if gap == None {
        return 0;
    }

    let gap = gap.unwrap();
    let mut valid_routes = 0;

    if remaining_springs[gap] ==  '?' {
        // let next_springs = remaining_springs[gap+1..].iter().cloned().collect();
        let mut next_springs = remaining_springs.clone();
        next_springs[gap] = '.';
        valid_routes += fill_gaps_with_memory(&next_springs, &remaining_layout, memory);
    }


    let filled_gap = gap+next_layout;
    let mut filled_gap_springs: Vec<char> = remaining_springs[filled_gap..].iter().cloned().collect();
    if !filled_gap_springs.is_empty() && filled_gap_springs[0] == '?' {
        filled_gap_springs[0] = '.'
    }

    let filled_gap_layout = remaining_layout[1..].iter().cloned().collect();

    valid_routes += fill_gaps_with_memory(&filled_gap_springs, &filled_gap_layout, memory);

    memory.insert((remaining_springs.to_vec(), remaining_layout.to_vec()), valid_routes);

    return valid_routes
    
}

fn find_valid_arrangements(spring_row: &SpringRow) -> i64 {
    // println!(" {:?}", spring_row);
    let valid_arrangements = fill_gaps(&spring_row.springs, &spring_row.layout);
    // println!("final result: {:?}", valid_arrangements);

    return valid_arrangements
}

fn find_valid_arrangements_2(spring_row: &SpringRow) -> i64 {
    let mut modified_spring_row = spring_row.springs.clone();
    for _i in 0..4 {
        modified_spring_row.push('?');
        modified_spring_row.extend(spring_row.springs.clone());
    }
    let mut modifified_layout = spring_row.layout.clone();
    for _i in 0..4 {
        modifified_layout.extend(spring_row.layout.clone());
    }
    let mut memory = HashMap::new();
    let valid_arrangements = fill_gaps_with_memory(&modified_spring_row, &modifified_layout, &mut memory);
    return valid_arrangements
}

fn part1 () -> i64{
    let spring_conditions = get_spring_conditions();
    // println!("{:?}", spring_conditions);
    // for spring_map in spring_conditions {

    // }
    return spring_conditions.iter().map(|spring_row| {find_valid_arrangements(&spring_row)}).sum();
    // return find_valid_arrangements(&SpringRow { springs: vec!['.', '.', '#', '.', '#', '#'], layout: vec![2] })     // [#.#..##] 2
    // return find_valid_arrangements(&SpringRow { springs: vec!['#', '#', '#', '.', '#', '#'], layout: vec![2] }) 
    // return find_valid_arrangements(&SpringRow { springs: vec!['#', '?', '#', '.', '.', '.'], layout: vec![2] }) 
    // return find_valid_arrangements(&spring_conditions[0]);
    // find_valid_arrangements(&spring_conditions[1]);
    // return find_valid_arrangements(&spring_conditions[0]);
    // return 0
}


fn part2 () -> i64{
    let spring_conditions = get_spring_conditions();
    return spring_conditions.iter().enumerate().map(|(index,spring_row)| {
        println!("{:?}", index);
        find_valid_arrangements_2(&spring_row)
    }).sum();

}
