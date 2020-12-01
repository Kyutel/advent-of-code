use std::collections::HashMap;

fn main() {
    let mut program = read_file();
    // let mut test_program = vec!(104,1125899906842624,99);
    // test_program.resize(test_program.len() *  10000,0);
    program.resize(program.len() *  10,0);


    let mut robot_location = (0,0);
    let mut robot_direction = "up".to_string();
    let mut hull_color = HashMap::new();

    let mut input = 1;
    let mut pc = 0;
    let mut rb = 0;
    loop {
       

       let output1 = do_intcode(&mut program,input,pc,rb);
       pc = output1.1;
       rb = output1.2;
       hull_color.insert(robot_location,output1.0);
       let output2 = do_intcode(&mut program,input,pc,rb);
       let new_robot_loc_and_dir = get_new_robot_loc_and_dir(robot_location,robot_direction,output2.0);
       robot_location = new_robot_loc_and_dir.0;
       robot_direction = new_robot_loc_and_dir.1; 
       pc = output2.1;
       rb = output2.2;

       if !hull_color.contains_key(&robot_location){
        input = 0;
       }
       else {
        input = *hull_color.get(&robot_location).unwrap();
       }


       println!("next;");
       if output1.0 == -1  || output2.0 == -1{
        break;
       }

    }

    println!("length {:?}",hull_color.len() );
    let mut hull_color:Vec<(i32,i32)> = hull_color.into_iter()
    .filter(|x| x.1 == 1)
    .map(|x| x.0)
    // .filter(|x| x.2 == 0)
    .collect();

    println!("{:?}", hull_color);

}

fn get_new_robot_loc_and_dir(location:(i32,i32),direction:String,input:i64) -> ((i32,i32),String) {
    match direction.as_ref() {
            "up" => {
                let new_location;
                let new_direction;
                if input == 0 {
                    new_location = (location.0 - 1,location.1);
                    new_direction = "left".to_string();
                }
                else {
                    new_location = (location.0 + 1,location.1);
                    new_direction = "right".to_string();
                }
                return (new_location,new_direction);
            }
            "down" => {
                let new_location;
                let new_direction;
                if input == 0 {
                    new_location = (location.0 + 1,location.1);
                    new_direction = "right".to_string();
                }
                else {
                    new_location = (location.0 - 1,location.1);
                    new_direction = "left".to_string();
                }
                return (new_location,new_direction);
            }
            "left" => {
                let new_location;
                let new_direction;
                if input == 0 {
                    new_location = (location.0,location.1 - 1);
                    new_direction = "down".to_string();
                }
                else {
                    new_location = (location.0,location.1 + 1);
                    new_direction = "up".to_string();
                }
                return (new_location,new_direction);
            }
            "right" => {
                let new_location;
                let new_direction;
                if input == 0 {
                    new_location = (location.0,location.1 + 1);
                    new_direction = "up".to_string();
                }
                else {
                    new_location = (location.0,location.1 - 1);
                    new_direction = "down".to_string();
                }
                return (new_location,new_direction);
            } 
            _ => {
                println!("heh error");
                return ((0,0),"error".to_string());
            }
        }

}

fn do_intcode(viktor: &mut Vec<i64>,program_input: i64,pc:usize, rb:usize) -> (i64,usize,usize){
        let mut program_counter = pc;
        let mut output = -1;
        let mut relative_base = rb;
        let input = program_input;

        loop {
            let instruction = viktor[program_counter];
            let mut program_increment = 4;

            let instruction = format!("{:05}",instruction);
            let op_code = &instruction[3..];

            if op_code == "99"{
                // return (-1,0);
                // return -1;
                // break
                return (-1,0,0);
            }

            let mode3 = &instruction[0..1];
            let mut address3 = 0;
            if program_counter + 3 < viktor.len()
            {
                if mode3 == "0"{
                    address3 = viktor[program_counter+3] as usize;
                }
                else if mode3 == "2" {
                    address3 = (viktor[program_counter+3] + relative_base as i64) as usize;
                }
                else {
                    println!("unrecognised parameter mode p3");
                }
            }



            if op_code == "01" {
                let values = get_values(instruction,viktor.to_vec(),program_counter,relative_base);

                viktor[address3] = values.0 + values.1;
            }
            else if op_code == "02" {
                let values = get_values(instruction,viktor.to_vec(),program_counter,relative_base);

                viktor[address3] = values.0 * values.1;
            }
            else if op_code == "03"
            {
                // let input = 3;
                // let input;
                let mode1 = &instruction[2..3];

                let address1;
                if mode1 == "0" {
                    address1 = viktor[program_counter+1] as usize;
                }
                else if mode1 == "2" {
                    address1 = (viktor[program_counter+1] + relative_base as i64) as usize;
                }
                else {
                    address1 = 0;
                    println!("error unrecognised parameter mode");
                }

                program_increment = 2;
                viktor[address1] = input;
            }
            else if op_code == "04"
            {
                output = get_value(instruction,viktor.to_vec(),program_counter, relative_base);
                println!("Output: {:?}", output);
                program_increment = 2;
                program_counter += program_increment;
                return (output,program_counter,relative_base);
            }
            else if op_code == "05"
            {
                let values = get_values(instruction,viktor.to_vec(),program_counter, relative_base);
                if values.0 != 0{
                    program_counter = values.1 as usize;
                    continue;
                }
                program_increment = 3;
            }
            else if op_code == "06"
            {
                let values = get_values(instruction,viktor.to_vec(),program_counter, relative_base);
                if values.0 == 0{
                    program_counter = values.1 as usize;
                    continue;
                }
                program_increment = 3;
            }
            else if op_code == "07"
            {
                let values = get_values(instruction,viktor.to_vec(),program_counter, relative_base);
                if values.0 < values.1 {
                    viktor[address3] = 1;
                }
                else {
                    viktor[address3] = 0;
                }
            }
            else if op_code == "08"
            {
                let values = get_values(instruction,viktor.to_vec(),program_counter, relative_base);
                if values.0 == values.1 {
                    viktor[address3] = 1;
                }
                else {
                    viktor[address3] = 0;
                }
            }
            else if op_code == "09"
            {
                let value = get_value(instruction,viktor.to_vec(),program_counter, relative_base);
                let temp_base = relative_base as i64 + value;
                relative_base = temp_base as usize;
                program_increment = 2;
            }
            else {
                println!("????");
                println!("Something went wrong: Unrecognised op code");
                println!("pc {:?}",program_counter);
                println!("inst {:?}",instruction);
            }

            program_counter += program_increment;
        }
    // output
}

//taken from luke smith
//https://github.com/lukesmith-bjss/advent-of-code-2019/blob/master/day_2/src/main.rs
fn read_file() -> Vec<i64> {
    include_str!("data.txt").split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect()
}

fn get_value(instruction:String,viktor: Vec<i64>,program_counter: usize,relative_base: usize) -> i64 {
    let mode1 = &instruction[2..3];

    let value;

    if mode1 == "0" {
        let address1 = viktor[program_counter+1] as usize;
        value = viktor[address1];
    }
    else if mode1 == "1" {
        value = viktor[program_counter+1];
    }
    else if mode1 == "2" {
        let address1 = (viktor[program_counter+1] + relative_base as i64) as usize;
        value = viktor[address1];     
    }
    else {
        println!("error unrecognised parameter mode");
        value = -1;
    }

    value
}

fn get_values(instruction:String,viktor: Vec<i64>,program_counter: usize,relative_base: usize)  -> (i64,i64) {
    let mode1 = &instruction[2..3];
    let mode2 = &instruction[1..2];

    let value1;
    let value2;

    if mode1 == "0" {
        let address1 = viktor[program_counter+1] as usize;
        value1 = viktor[address1];
    }
    else if mode1 == "1" {
        value1 = viktor[program_counter+1];
    }
    else if mode1 == "2" {
        let address1 = (viktor[program_counter+1] + relative_base as i64) as usize;

        value1 = viktor[address1];     
    }
    else {
        value1 = -1;
        println!("error unrecognised parameter mode for p1");
    }

    if mode2 == "0" {
        let address2 = viktor[program_counter+2] as usize;
        value2 = viktor[address2];

    }
    else if mode2 == "1" {
        value2 = viktor[program_counter+2];
    }
    else if mode2 == "2" {
        let address2 = (viktor[program_counter+2] + relative_base as i64) as usize;
        value2 = viktor[address2];     
    }
    else {
        println!("error unrecognised parameter mode for p2");
        value2 = -1;
    }

    (value1,value2)
}