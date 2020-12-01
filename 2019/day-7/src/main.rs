fn main() {
    let program = read_file();

    // let mut program_copy = program.clone();

    
    let mut max_output = 0;

    use permutohedron::Heap;

    let mut data = vec![5,6,7,8,9];
    let heap = Heap::new(&mut data);
    
    let mut permutations = Vec::new();
    for data in heap {
        permutations.push(data.clone());
    }

    for permutation in &permutations {
        let mut output = 0;
        let mut program_copies = [program.clone(),program.clone(),program.clone(),program.clone(),program.clone()];
        let mut pcs: [usize;5] = [0;5];
        let mut finished = false;
        let mut phases_been_used = false;
        while finished == false {
            for (i,phase) in permutation.iter().enumerate() {
                let returned_values = do_intcode(&mut program_copies[i], output,*phase,pcs[i],phases_been_used);
                output = returned_values.0;
                pcs[i] = returned_values.1;
                if output == -1{
                    println!("HALT");
                    finished = true;
                }
                use std::cmp;
                max_output = cmp::max(max_output,output);
                // program_copy = program.clone();
            }
            phases_been_used= true;

        }
    }

    // println!("program output {:?}",output);
    println!("Max Output was: {:?}",max_output );
}


fn do_intcode(viktor: &mut Vec<i32>,program_input: i32,phase: i32,
    pc:usize,use_input: bool) -> (i32,usize) {
        let mut program_counter = pc;
        let mut output = 0;
        let mut use_input = use_input;
        loop {
            let instruction = viktor[program_counter];
            let mut program_increment = 4;

            let instruction = format!("{:05}",instruction);
            let op_code = &instruction[3..];

            if op_code == "99"{
                return (-1,0);
            }

            let mut address3 = 0;
            if (program_counter + 3 < viktor.len())
            {
                address3 = viktor[program_counter+3] as usize;
            }
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
                // let input = 3;
                let input;
                if use_input {
                    input = program_input;
                }else {
                    input = phase;
                    use_input = true;
                }


                let address1 = viktor[program_counter+1] as usize;
                program_increment = 2;
                viktor[address1] = input;
            }
            else if op_code == "04"
            {
                output = get_value(instruction,viktor.to_vec(),program_counter);
                println!("Output: {:?}", output);
                program_increment = 2;
                program_counter += program_increment;
                return (output,program_counter);
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
    (output,program_counter)
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