// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
struct NumberPosition {
    value:i64,
    position: usize
}

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

impl From<(usize, &str)> for NumberPosition {
    fn from(number_pos: (usize, &str)) -> Self {
        let position = number_pos.0;
        let value = number_pos.1.parse().unwrap();
        NumberPosition { value, position  }
    }
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_numbers_from_input() -> Vec<NumberPosition>{
    FILE_CONTENTS.split('\n').enumerate().map(|x| NumberPosition::from(x)).collect()
}

fn _print_list_in_order(number_positions: &Vec<NumberPosition>) {

    let mut list = vec![0;number_positions.len()];
    for num_pos in number_positions {
        list[num_pos.position] = num_pos.value
    }
    println!("{:?}", list)
}

fn get_pos_wrapped(pos: i64, shift:i64, list_len: i64)  -> usize{
    

    let mut new_position = pos + shift;

    if new_position >= list_len {
        // println!("p:{}", (new_position/list_len));
        // new_position = (new_position + (new_position/list_len)) % list_len;
        new_position %= list_len-1;
    }
    else if new_position < 0 {
        // new_position +=  (new_position/list_len) - 1;
        new_position %= list_len-1;
        // println!("n:{}", (new_position/list_len));
        new_position = (list_len-1) + new_position;
    }
    // println!("npt = {:?}", new_position);
    new_position as usize
}

fn shift_positions(original_index: usize, full_list: &mut Vec<NumberPosition>) {
    let list_len = full_list.len() as i64;
    let num_pos = full_list[original_index].clone();

    
    let new_position = get_pos_wrapped(num_pos.position as i64, num_pos.value, list_len);
    // println!("np = {:?}", new_position);

    let direction = if new_position < num_pos.position {
        Direction::Left
    }
    else if new_position > num_pos.position{
        Direction::Right
    }
    else {
        return;
    };
    
    for (index, to_change) in full_list.iter_mut().enumerate() {
        if index == original_index {
            to_change.position = new_position;
        }
        else {
            match direction {
                Direction::Left => 
                if to_change.position >= new_position && to_change.position < num_pos.position{
                    // to_change.position +=1
                    to_change.position = get_pos_wrapped(to_change.position as i64, 1, list_len);
                }
                Direction::Right => 
                
                if to_change.position <= new_position && to_change.position > num_pos.position {
                    // to_change.position -=1
                    to_change.position = get_pos_wrapped(to_change.position as i64, -1, list_len);
                }

            }
        }    
    }



}

fn part1 () -> i64{
    let mut number_positions = get_numbers_from_input();
    // print_list_in_order(&number_positions);


    for original_index in 0..number_positions.len() {
        shift_positions(original_index, &mut number_positions);
        
    }
    
    let mut zero_numpos = &NumberPosition{value:0, position:0 };
    // let zero_numpos;
    for numpos in number_positions.iter() {
        if numpos.value == 0 {
            zero_numpos = numpos;
        }
    }

    println!("{:?}", zero_numpos);
    let index_1 = (zero_numpos.position + 1000) % number_positions.len();
    let index_2 = (zero_numpos.position + 2000) % number_positions.len();
    let index_3 = (zero_numpos.position + 3000) % number_positions.len();

    let mut answer = 0;
    for numpos in number_positions.iter() {
        if numpos.position == index_1 {
            answer+= numpos.value;
        }
        if numpos.position == index_2 {
            answer+= numpos.value;
        }
        if numpos.position == index_3 {
            answer+= numpos.value;
        }
    }
 
    answer
}

fn part2 () -> i64{

    let mut number_positions = get_numbers_from_input();
    let decription_key:i64 = 811589153;
    for numpos in number_positions.iter_mut() {
        numpos.value *= decription_key;
    } 
    // print_list_in_order(&number_positions);
    
    for _ in 0..10 {
        for original_index in 0..number_positions.len() {
            shift_positions(original_index, &mut number_positions);
            // print_list_in_order(&number_positions);
        }
        // print_list_in_order(&number_positions);
    }


    let mut zero_numpos = &NumberPosition{value:0, position:0 };
    for numpos in number_positions.iter() {
        if numpos.value == 0 {
            zero_numpos = numpos;
        }
    }

    println!("{:?}", zero_numpos);
    let index_1 = (zero_numpos.position + 1000) % number_positions.len();
    let index_2 = (zero_numpos.position + 2000) % number_positions.len();
    let index_3 = (zero_numpos.position + 3000) % number_positions.len();

    let mut answer = 0;
    for numpos in number_positions.iter() {
        if numpos.position == index_1 {
            // println!("1:{}", numpos.value);
            answer+= numpos.value;
        }
        if numpos.position == index_2 {
            // println!("2:{}", numpos.value);
            answer+= numpos.value;
        }
        if numpos.position == index_3 {
            // println!("3:{}", numpos.value);
            answer+= numpos.value;
        }
    }
 

    answer
}
