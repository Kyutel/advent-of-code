fn main() {
    let mut program = read_file();
    do_intcode(&mut program);
}


fn do_intcode(viktor: &mut Vec<i32>) -> () {
        let mut program_counter = 0;

        loop {
            let instruction = viktor[program_counter];
            let mut program_increment = 4;

            let instruction = format!("{:05}",instruction);
            let op_code = &instruction[3..];

            if op_code == "99"{
                break
            }

            let address3 = viktor[program_counter+3] as usize;
            
            if op_code == "01" {
                let values = get_values(instruction,viktor.to_vec(),program_counter);

                viktor[address3] = values.0 + values.1;
            }
            else if op_code == "02" {
                let values = get_values(instruction,viktor.to_vec(),program_counter);

                viktor[address3] = values.0 * values.1;
            }
            else if op_code == "03"
            {
                let input = 5;
                let address1 = viktor[program_counter+1] as usize;
                program_increment = 2;
                viktor[address1] = input;
            }
            else if op_code == "04"
            {
                let output = get_value(instruction,viktor.to_vec(),program_counter);
                println!("Output: {:?}", output);
                program_increment = 2;
            }
            else if op_code == "05"
            {
                let values = get_values(instruction,viktor.to_vec(),program_counter);
                if values.0 != 0{
                    program_counter = values.1 as usize;
                    continue;
                }
                program_increment = 3;
            }
            else if op_code == "06"
            {
                let values = get_values(instruction,viktor.to_vec(),program_counter);
                if values.0 == 0{
                    program_counter = values.1 as usize;
                    continue;
                }
                program_increment = 3;
            }
            else if op_code == "07"
            {
                let values = get_values(instruction,viktor.to_vec(),program_counter);
                if values.0 < values.1 {
                    viktor[address3] = 1;
                }
                else {
                    viktor[address3] = 0;
                }
            }
            else if op_code == "08"
            {
                let values = get_values(instruction,viktor.to_vec(),program_counter);
                if values.0 == values.1 {
                    viktor[address3] = 1;
                }
                else {
                    viktor[address3] = 0;
                }
            }
            else {
                println!("????");
                println!("Something went wrong: Unrecognised op code");
                println!("pc {:?}",program_counter);
                println!("inst {:?}",instruction);
            }

            program_counter += program_increment;
        }
}

//taken from luke smith
//https://github.com/lukesmith-bjss/advent-of-code-2019/blob/master/day_2/src/main.rs
fn read_file() -> Vec<i32> {
    include_str!("data.txt").split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

fn get_value(instruction:String,viktor: Vec<i32>,program_counter: usize) -> i32 {
    let mode1 = &instruction[2..3];

    let value;

    if mode1 == "0" {
        let address1 = viktor[program_counter+1] as usize;
        value = viktor[address1];
    }
    else {
        value = viktor[program_counter+1];
    }

    value
}

fn get_values(instruction:String,viktor: Vec<i32>,program_counter: usize)  -> (i32,i32) {
    let mode1 = &instruction[2..3];
    let mode2 = &instruction[1..2];

    let value1;
    let value2;

    if mode1 == "0" {
        let address1 = viktor[program_counter+1] as usize;
        value1 = viktor[address1];
    }
    else {
        value1 = viktor[program_counter+1];
    }

    if mode2 == "0" {
        let address2 = viktor[program_counter+2] as usize;
        value2 = viktor[address2];
    }
    else {
        value2 = viktor[program_counter+2];
    }

    (value1,value2)
}