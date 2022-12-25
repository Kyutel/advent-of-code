static FILE_CONTENTS: &str = include_str!("input.txt");

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_parsed_input() -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>){
    let split_file:Vec<&str> = FILE_CONTENTS.split("\n\n").collect();

    let mut unparsed_crate_stacks:Vec<&str> = split_file[0].split('\n').collect();
    unparsed_crate_stacks.reverse();
    let stack_numbers:Vec<&str> = unparsed_crate_stacks[0].split(' ').collect();
    let number_of_stacks = stack_numbers[stack_numbers.len()-2].parse::<i32>().unwrap();
    let mut crate_stacks = Vec::new();
    for _ in 0..number_of_stacks {
        crate_stacks.push(Vec::new());
    }

    for line in unparsed_crate_stacks[1..].into_iter() {
        let mut crate_stack_no = 0;

        for crate_id in line.chars().skip(1).step_by(4) {
            if crate_id != ' ' {
                crate_stacks[crate_stack_no].push(crate_id)
            }
            crate_stack_no += 1
        }
    }

    let rearangements= split_file[1].split('\n').map(|x| {
        let t: Vec<&str> = x.split(' ').collect();
        return (t[1].parse::<usize>().unwrap(), t[3].parse::<usize>().unwrap(), t[5].parse::<usize>().unwrap())
    }).collect();
    
    // println!("{:?}", crate_stacks);
    // println!("{:?}", rearangements);
    
    (crate_stacks, rearangements)
}


fn part1() -> String {
    let (mut crate_stacks, rearangements) = get_parsed_input();
    for rearangement in rearangements {
        
        let stack_length = crate_stacks[rearangement.1 - 1].len();
        let index_to_move_from = stack_length-rearangement.0;
        let mut removed:Vec<char> = crate_stacks[rearangement.1 - 1].drain(index_to_move_from..).collect();
        removed.reverse();
        for crate_id in removed{
            crate_stacks[rearangement.2 - 1].push(crate_id)
        }

    }
    let mut answer = String::new();
    for crate_stack in crate_stacks.into_iter() {
        let next = crate_stack[crate_stack.len() - 1];
        answer.push(next);
    }

    answer
}
fn part2() -> String {
    let (mut crate_stacks, rearangements) = get_parsed_input();
    for rearangement in rearangements {
        
        let stack_length = crate_stacks[rearangement.1 - 1].len();
        let index_to_move_from = stack_length-rearangement.0;
        let removed:Vec<char> = crate_stacks[rearangement.1 - 1].drain(index_to_move_from..).collect();
        for crate_id in removed{
            crate_stacks[rearangement.2 - 1].push(crate_id)
        }

    }
    let mut answer = String::new();
    for crate_stack in crate_stacks.into_iter() {
        let next = crate_stack[crate_stack.len() - 1];
        answer.push(next);
    }

    answer
}