fn main() {
    // let input = "12345678".to_string();
    // let input = "80871224585914546619083218645595".to_string();
    // let input = "03036732577212944063491565474664".to_string();
    let input = include_str!("data.txt").to_string();
    let number_of_phases = 100;
    // println!("input: {:?}", input);
    // let mut next_phase = input.clone();
    let mut next_phase = repeat_input(&input);
    let offset = (&input[0..7]).parse::<usize>().unwrap();
    let test = &next_phase[offset..];
    next_phase = test.to_string();

    // println!("{:?}", test);

    for phase in 0..number_of_phases {
        println!("{:?}", phase);
        next_phase = calculate_next_phase(next_phase.clone());
        // println!("{:?}", next_phase);
        // println!("{:?}", next_phase);
        // println!("{:?}", phase);
    }

    // let digits = &next_phase[offset..offset+8];
    // println!("output digits: {:?}", next_phase);
    println!("{:?}", &next_phase[..8]);
}

fn repeat_input(input: &String) -> String{
    let mut repeated = "".to_string();

    for _i in 0..10000 {
        repeated += &input.clone();
    }

    repeated
}

fn create_repeating_pattern(output_index:usize,output_length:usize) -> Vec<i32>{
    let base_pattern = [0,1,0,-1];
    let mut repeating_pattern = Vec::new();
    let mut base_digit = 0;

    while repeating_pattern.len() < output_length + 1{
        for _repeat in 0..output_index {
            repeating_pattern.push(base_pattern[base_digit]);
        }
        base_digit = (base_digit+1)% base_pattern.len(); 
    }

    repeating_pattern.remove(0);
    repeating_pattern.truncate(output_length);

    repeating_pattern
}

fn create_repeating_pattern_v2(output_index:usize,output_length:usize) -> Vec<i32>{
    let base_pattern = [1,0,-1,0];
    let mut repeating_pattern = Vec::new();
    let mut base_digit = 0;

    while repeating_pattern.len() < output_length + 1{
        for _repeat in 0..output_index {
            repeating_pattern.push(base_pattern[base_digit]);
            if repeating_pattern.len() > output_length {
                break;
            }
        }
        base_digit = (base_digit+1)% base_pattern.len(); 
    }

    repeating_pattern.remove(0);
    repeating_pattern.truncate(output_length);

    repeating_pattern
}

fn calculate_next_phase(input: String) -> String {
    let mut output_digits = "".to_string();
    let first_sum:i32 = input.chars()
        .map(|d| d.to_digit(10).unwrap() as i32)
        .sum();

    let next_digit = first_sum.abs()  % 10;
    output_digits += &next_digit.to_string().clone();

    let mut test_str = input.clone();
    let mut last_sum = first_sum;

    for digit in 2..input.len() + 1{

        // println!("faster");
        // let repeating_pattern = create_repeating_pattern_v2(digit + offset,input.len());
        // let next_digit:i32 = input.chars().enumerate().skip(digit - 1)
        // let next_digit:i32 = input.chars().skip(digit - 1)
        // // .map(|(i,d)| d.to_digit(10).unwrap() as i32 * repeating_pattern[i])
        // .map(|d| d.to_digit(10).unwrap() as i32)
        // .sum();
        let next_sum = last_sum - &test_str[..1].parse::<i32>().unwrap();
        test_str = (&test_str[1..]).to_string();
        last_sum = next_sum;

        let next_digit = next_sum.abs()  % 10;
        output_digits += &next_digit.to_string().clone();

        // println!("{:?}: {:?}, {:?}",next_digit, digit, input.len());
        // println!("{:?}", offset + digit -1);
        // println!("{:?}", next_digit);
    }

    output_digits
}