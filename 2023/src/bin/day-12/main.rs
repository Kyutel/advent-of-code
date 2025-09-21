// use log::{debug, trace};
static FILE_CONTENTS: &str = include_str!("test-input.txt");
// static FILE_CONTENTS: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
struct SpringRow {
    springs: Vec<char>,
    layout: Vec<i32>
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}


fn get_spring_conditions() -> Vec<SpringRow> {
    FILE_CONTENTS.split("\n").map(|line|  {
        let split_line: Vec<String> = line.split_whitespace().map(|x| x.to_string()).collect();
        let springs = split_line[0].chars().collect();
        let layout: Vec<i32> = split_line[1].split(',').map(| number| number.parse::<i32>().unwrap()).collect();

        SpringRow { springs, layout }
    }).collect()
}

fn fill_gaps(remaining_springs: &Vec<char>, remaining_layout: &Vec<i32>)  -> i32{
    println!("{:?}, {:?}", remaining_springs, remaining_layout);

    
    if remaining_layout.len() == 0 {
        println!("all gaps filled {:?}, {:?}", remaining_springs, remaining_layout);
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
        // println!("index {:?}, next_slice {:?}",index, next_slice);
        if !next_slice.contains(&'.') {
            // println!("index {:?}, next_slice {:?}",index, next_slice);
            if index+next_layout < remaining_springs.len() && remaining_layout.len() != 1
                && remaining_springs[index+next_layout] != '#' {
                gap = Some(index);
                break;
            } 
            else if remaining_layout.len() == 1 && index+next_layout <= remaining_springs.len() {
                gap = Some(index);
                break;
            }
            
        }
    }

    if gap == None {
        // println!("No gaps remain to fit spring {:?}, {:?}", remaining_springs, remaining_layout);
        return 0;
    }

    let gap = gap.unwrap();
    let mut valid_routes = 0;

    if remaining_springs[gap] ==  '?' {
        // let next_springs = remaining_springs[gap+1..].iter().cloned().collect();
        let mut next_springs = remaining_springs.clone();
        next_springs[gap] = '.';
        println!("entering with w/o filling {:?}, {:?}", next_springs, remaining_layout);
        valid_routes += fill_gaps(&next_springs, &remaining_layout);
        println!("exit back into {:?}, {:?}", remaining_springs, remaining_layout);

    }

    // println!("gap {:?}", gap);

    let mut filled_gap = gap+next_layout;
    // if filled_gap > remaining_springs.len() && remaining_layout.len() > 1 {
    //     return 0
    // }
        // println!("No gaps remain to fit spring v2 {:?}, {:?}", remaining_springs, remaining_layout);
        // return 0
    // }
    let mut filled_gap_springs: Vec<char> = remaining_springs[filled_gap..].iter().cloned().collect();
    if !filled_gap_springs.is_empty() && filled_gap_springs[0] == '?' {
        filled_gap_springs[0] = '.'
    }


    let filled_gap_layout = remaining_layout[1..].iter().cloned().collect();

    // filled_gap_springs[0] = '.';

    println!("filled gap and entering with {:?}, {:?}", filled_gap_springs, filled_gap_layout);
    valid_routes += fill_gaps(&filled_gap_springs, &filled_gap_layout);
    println!("exit back into {:?}, {:?}", remaining_springs, remaining_layout);    

    println!("Valid Routes: {:?}", valid_routes);


    return valid_routes
    
}

fn find_valid_arrangements(spring_row: &SpringRow) -> i32 {
    // println!(" {:?}", spring_row);
    let valid_arrangements = fill_gaps(&spring_row.springs, &spring_row.layout);
    println!("final result: {:?}", valid_arrangements);

    return valid_arrangements
}

fn part1 () -> i32{
    let spring_conditions = get_spring_conditions();
    // println!("{:?}", spring_conditions);
    // for spring_map in spring_conditions {

    // }
    // return spring_conditions.iter().map(|spring_row| {find_valid_arrangements(&spring_row)}).sum();
    return find_valid_arrangements(&spring_conditions[5]);
    // find_valid_arrangements(&spring_conditions[1]);
    // return find_valid_arrangements(&spring_conditions[0]);
    // return 0
}


fn part2 () {}
