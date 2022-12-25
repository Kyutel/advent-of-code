use std::collections::HashMap;

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

fn main() {
    println!("{:?}", part1());
    // println!("{:?}", part2());
}

fn get_values_from_input() -> Vec<Vec<char>> {
    FILE_CONTENTS.split('\n').map(|x| x.chars().rev().collect()).collect()
}

fn convert_char_to_val(c: char) -> i64 {
    if c == '=' 
    {
        return -2
    }
    else if c == '-' {
        return -1
    }
    else {return c.to_digit(10).unwrap() as i64}
}

fn convert_val_to_char(i: i64) -> char {
    match i {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => panic!()
    }
}

fn part1 ()  -> String{
    let fuel_requirements = get_values_from_input();
    let mut total_req = 0;
    for req in fuel_requirements {
        for (i,c) in req.iter().enumerate() {
            let c_val = convert_char_to_val(*c);
            total_req +=  c_val * 5_i64.pow(i as u32);
        }
    }

    println!("{total_req}");

    // let temp = 12345;
    // let total_req = temp;
    
    //find_biggest_exponent
    let mut exp = 0;
    let mut next_try = 0;
    // let mut found = false;
    let mut max_map:Vec<i64> = Vec::new();

    loop {
        next_try += 5_i64.pow(exp);
        max_map.push(next_try*2);

        if next_try * 2 > total_req {
            break;
        }
        exp+=1;
    }
    // println!("{max_map:?}");

    let mut remaining = total_req;
    let mut snafu = String::new();
    while exp >= 0 {
    // for i in 0..6 {
        let exp_val = 5_i64.pow(exp);
        // println!("remaining: {remaining}");


        // println!("{}", max_map.get(&(exp-1)).unwrap());
        let calc = (remaining.abs() - exp_val).abs();
        let calc_2 = (remaining.abs() - exp_val*2).abs();

        // println!("calc: {}", calc);
        // println!("calc2: {}", calc_2);
        // println!("exp: {}", exp_val);
        let max_for_next= if exp == 0{
            0
        }
        else { max_map[(exp-1) as usize]};
        if calc <=  max_for_next {
            snafu.push(convert_val_to_char(1 * remaining.signum()));
            remaining = (remaining - exp_val * remaining.signum());
        }
        else if calc_2 <= max_for_next  {
            snafu.push(convert_val_to_char(2 * remaining.signum()));
            remaining = (remaining - exp_val*2 * remaining.signum());
        }
        else  {
            snafu.push(convert_val_to_char(0));

        }

        if exp == 0 {
            break;
        }

        exp-=1;
        // println!("{snafu}");
    }


    

    snafu    


}
// fn part2 () {}
