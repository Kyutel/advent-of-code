use std::collections::HashMap;

fn main() {

    let mut program = read_file();
    program.resize(program.len() *  10,0);

    let mut pc = vec![0;50];
    let mut rb = vec![0;50];

    let mut computers = vec![program;50];
    let mut network_queue: Vec<Vec<Vec<i64>>> = vec![Vec::new();50];
    let mut comp_idle_status:Vec<bool> = vec![false;50];

    let mut finished = false;
    let mut temp_output:Vec<Vec<i64>> = vec![Vec::new();50];
    let mut all_idle_count =0;

    let mut nat = Vec::new();
    let mut answer_map = HashMap::new();

    for computer_id in 0..50 {
        network_queue[computer_id].push(vec![computer_id as i64]);
    }

    while !finished {

        // println!();
        // println!("start of run");
        for computer_id in 0..50 {
            // println!("ID; {:?}", computer_id);
            let output = do_intcode(&mut computers[computer_id],&mut network_queue[computer_id],pc[computer_id],rb[computer_id]);
            pc[computer_id] = output.1;
            rb[computer_id] = output.2;

            if output.0  == -100 {
                println!("fire fire");
            }
            else if output.0 != -333 && output.0!= -334 && output.0!= -335  {
                temp_output[computer_id].push(output.0);
                // if computer_id == 0 {
                //     println!("{:?}", temp_output[computer_id]);
                // }
                // println!("temp out {:?}", temp_output[43]);
                if temp_output[computer_id].len() == 3 {
                    // println!("{:?}", temp_output[computer_id]);
                    
                    let output_location = temp_output[computer_id].remove(0) as usize;
                    // println!("id; {:?}", computer_id);
                    // println!("{:?}", output_location);
                    // println!("{:?}", network_queue);
                    // println!();
                    if output_location == 255 {
                        // println!("nah nah{:?}", temp_output[computer_id]);
                        nat = temp_output[computer_id].clone();
                        temp_output[computer_id] = Vec::new();
                        // finished = true;
                    }else {
                        // println!("hey {:?}",computer_id);
                        network_queue[output_location].push(temp_output[computer_id].clone());
                        temp_output[computer_id] = Vec::new();
                    }

                    
                }
            }
            else if output.0 == -334 {
                comp_idle_status[computer_id] = true;
                // println!("{:?}", network_queue);
            }
            else if output.0 == -335 {
                comp_idle_status[computer_id] = false;
                all_idle_count = 0;
            }

            // println!();
            let filtered:Vec<bool> = comp_idle_status.iter().filter(|x| **x == false).map(|x| *x).collect();
            // println!("{:?}", comp_idle_status);
            if filtered.len() == 0 {
                // println!("{:?}", comp_idle_status);
               
                // println!("all idle");
                all_idle_count +=1;
            }

            if all_idle_count > 1000 {
                // println!("idle for a while");
                // all_idle_count = 0;
                // println!("{:?}", all_idle_count);
                // println!("{:?}", nat);
                if nat.len() != 0 {
                    network_queue[0].push(nat.clone());
                    println!("nat sent {:?}", nat);
                    if answer_map.get(&nat[1]) != None {
                        finished = true;
                        println!("wik wik {:?}", nat[1]);
                        break;
                    }
                    answer_map.insert(nat[1],true);
                    println!("{:?}", answer_map);
                    // nat = Vec::new();
                    all_idle_count = 0;
                }

            }
            // else if output.0 == -333 {

            // }
            // else {
            //     println!("huh what happen: {:?}", output.0);
            // }
        }
    }


}



fn do_intcode(viktor: &mut Vec<i64>,input_queue: &mut Vec<Vec<i64>>,pc:usize, rb:usize) -> (i64,usize,usize){
        let mut program_counter = pc;
        let output;
        let mut relative_base = rb;
        // let input = program_input;

        // loop {
            let instruction = viktor[program_counter];
            let mut program_increment = 4;

            let instruction = format!("{:05}",instruction);
            let op_code = &instruction[3..];

            if op_code == "99"{
                println!("exit");
                return (-100,program_counter,relative_base);
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
                program_counter += program_increment;
                
                // println!("taking input");
                // println!("queue: {:?}", input_queue);
                let input;
                if input_queue.len() == 0{
                    input = -1;
                }
                else {
                    input = input_queue[0].remove(0);
                    if input_queue[0].len() == 0 {
                        input_queue.remove(0);
                    }
                }
                // println!("input taken: {:?}", input);


                viktor[address1] = input;
                if input == -1 {
                    return (-334,program_counter,relative_base);
                }

                return (-335,program_counter,relative_base);

            }
            else if op_code == "04"
            {
                output = get_value(instruction,viktor.to_vec(),program_counter, relative_base);
                // println!("Output: {:?}", output);
                program_increment = 2;
                program_counter += program_increment;

                return (output,program_counter,relative_base);
            }
            else if op_code == "05"
            {
                let values = get_values(instruction,viktor.to_vec(),program_counter, relative_base);
                if values.0 != 0{
                    program_counter = values.1 as usize;
                    return (-333,program_counter,relative_base);
                    // continue;
                }
                program_increment = 3;
            }
            else if op_code == "06"
            {
                let values = get_values(instruction,viktor.to_vec(),program_counter, relative_base);
                if values.0 == 0{
                    program_counter = values.1 as usize;
                    return (-333,program_counter,relative_base);
                    // continue;
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

            return (-333,program_counter,relative_base);
        // }
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