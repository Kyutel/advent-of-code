use std::vec;

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn read_numbers() -> Vec<Vec<i32>> {
    FILE_CONTENTS.split('\n')
        .map(|line| line.split(' ').map(|val| val.parse().unwrap()).collect())
        .collect()
}

fn get_next_number(values: Vec<i32>, ) -> i32{
    let mut differences = vec![];
    let mut last_layer = true;
    for i in 1..values.len() {
        let diff = values[i] - values[i - 1];
        differences.push(diff);
        if diff != 0 {
            last_layer = false;
        }
    }

    if !last_layer {
        return differences[differences.len() - 1] + get_next_number(differences);
    }

    0
}

fn get_prev_number(values: Vec<i32>, ) -> i32{
    let mut differences = vec![];
    let mut last_layer = true;
    for i in 1..values.len() {
        let diff = values[i] - values[i - 1];
        differences.push(diff);
        if diff != 0 {
            last_layer = false;
        }
    }

    if !last_layer {
        let first_num = differences[0].clone();
        return  first_num - get_prev_number(differences)
    }

    0
}

fn part1 () -> i32{
    let histories = read_numbers();
    histories.into_iter().map(|history| history[history.len() - 1] + get_next_number(history) ).sum()
}


fn part2 () -> i32 {
    let histories = read_numbers();
    
    // let history = histories[0].clone();
    histories.into_iter().map(|history| {
            let first_num = history[0].clone();
            first_num - get_prev_number(history)
    }).sum()

    // let first_num: i32 = history[0].clone();
    // let t = first_num - get_prev_number(history.clone());
    // println!("testing {:?}, {:?}, {:?}",first_num, t, get_prev_number(history));

    // 0
}
