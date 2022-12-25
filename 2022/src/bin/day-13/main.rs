use std::{str::Chars, cmp::Ordering};

static FILE_CONTENTS: &str = include_str!("input.txt");
// static FILE_CONTENTS: &str = include_str!("test-input.txt");

#[derive(Debug)]
enum Signal {
    List(Vec<Signal>),
    Value(i32)
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn parse_string_list_into_signal(the_string: &mut Chars) -> Signal {
    // println!("{:?}", the_string);
    let mut appended_chars = String::new();
    let mut next = the_string.next();
    let mut signal: Vec<Signal> = vec![];
    while next != None{
        let next_char = next.unwrap();
        if next_char=='[' {
            signal.push(parse_string_list_into_signal(the_string));
        }
        else if next_char == ']' {
            if appended_chars != "" {
                signal.push(Signal::Value(appended_chars.parse::<i32>().unwrap()));
            }

            return Signal::List(signal)
        }
        else if next_char == ',' &&  appended_chars != "" {
            signal.push(Signal::Value(appended_chars.parse::<i32>().unwrap()));
            appended_chars =  String::new();
        }
        else if next_char == ',' {
        }
        else {
            appended_chars.push(next_char);
        }
        next = the_string.next();
    }
    if appended_chars != "" {
        signal.push(Signal::Value(appended_chars.parse::<i32>().unwrap()));
    }

    return Signal::List(signal);
}


fn get_input_pairs() -> Vec<Vec<Signal>> {
    FILE_CONTENTS.split("\n\n")
    .map(|x| x.split('\n')
        .map(|y| parse_string_list_into_signal(&mut y[1..y.len()-1].chars()))
        .collect()
    )
    .collect()
}

fn get_input_total() -> Vec<Signal> {
    FILE_CONTENTS.split("\n")
    .filter(|&x| x!= "")
    .map(|x| parse_string_list_into_signal(&mut x[1..x.len()-1].chars()))
    .collect()
}


fn in_the_right_order (left: &Signal, right: &Signal) -> i32{
    // println!("{:?} {:?}", left, right);

    match (left, right) {
        (Signal::Value(x), Signal::Value(y)) => return if x!=y{if x<y {1} else {-1}} else { 0},
        (Signal::Value(x), Signal::List(_)) => {
            return in_the_right_order( &Signal::List(vec![Signal::Value(*x)]), right)
        },
        (Signal::List(_), Signal::Value(y)) => {
            return in_the_right_order(left,&Signal::List(vec![Signal::Value(*y)]))
        },
        (Signal::List(x), Signal::List(y)) => {
            let mut x_iter = x.iter();
            let mut y_iter = y.iter();
            loop {
                let next_left =  x_iter.next();
                let next_right = y_iter.next();
                let order = match (next_left, next_right) {
                    (None, Some(_)) => 1,
                    (Some(_), None) => -1,
                    (None, None) => return 0,
                    (Some(l_val), Some(r_val)) => in_the_right_order(l_val, r_val),

                };
                if order != 0 {
                    return  order;
                }
            }
        }

    }
}

fn part1() -> usize {
    let pairs = get_input_pairs();
    let mut indexes_not_in_order = vec![];
    for (index, pair) in pairs.iter().enumerate() {
        let left = &pair[0];
        let right = &pair[1];
        // println!("{:?}", in_the_right_order(left, right));

        if in_the_right_order(left, right) == 1 {
            indexes_not_in_order.push(index + 1);
        }

        // println!("{:?} : {:?}", pair[0], pair[1]);
    }

    indexes_not_in_order.iter().sum()
}

fn part2() -> usize{
    let mut packets = get_input_total();
    let divider_1 = parse_string_list_into_signal(&mut "[2]".chars());
    let divider_2 = parse_string_list_into_signal(&mut "[6]".chars());

    packets.push(parse_string_list_into_signal(&mut "[2]".chars()));
    packets.push(parse_string_list_into_signal(&mut "[6]".chars()));



    packets.sort_by(|a,b| {
        let val = in_the_right_order(a, b);
        return if val == 0 { Ordering::Equal}
        else if  val == -1 {Ordering::Greater}
        else {Ordering::Less}            
    });

    let mut index_1 = 0;
    let mut index_2 = 0;

    for (index, packet) in packets.iter().enumerate() {
        // println!("{:?}", packet);
        if in_the_right_order(&packet, &divider_1) == 0 {
            index_1 = index +1;
        }
        if in_the_right_order(&packet, &divider_2) == 0 {
            index_2 = index +1;
        }
    }

    return index_1 * index_2
}