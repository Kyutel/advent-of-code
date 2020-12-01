fn main() {
    let mut program = read_file();
    program.resize(program.len() *  10,0);

    // let mut program_clone = program.clone();

    const GRID_SIZE:usize = 30;
    // const WINDOW_SIZE:usize = 50;
    let mut grid = vec![vec![0;GRID_SIZE];GRID_SIZE];
    // let mut point_count = 0;

    // let x_offset = 489;
    // let y_offset = 657;

    let x_offset = 3;
    let y_offset = 4;

    // for x in 0..GRID_SIZE {
    //     for y in 0..GRID_SIZE {
    //         let location_to_check = ((x+x_offset) as i64,(y+y_offset) as i64);

    //         let output = check_location(location_to_check,program.clone());
    //         grid[y][x] = output;

    //         // if output  == 1{
    //         //     point_count +=1;
    //         // }
    //     }
    // }

    // print_grid(grid);




    let mut answer_location_to_check = (x_offset as i64,y_offset as i64);

    loop  {
        let check = check_100_square(answer_location_to_check,&program);
        if check == 200 {
            break
        }
        else if check == 0 {
            answer_location_to_check = (answer_location_to_check.0 + 1,answer_location_to_check.1)
         }
        else if check == 1 {
            answer_location_to_check = (answer_location_to_check.0,answer_location_to_check.1 + 1)
         }
        else if check == 2 {
            answer_location_to_check = (answer_location_to_check.0 + 1,answer_location_to_check.1)
         }
        else if check == 3 {
            println!("huh");
            answer_location_to_check = (answer_location_to_check.0,answer_location_to_check.1 + 1)
         }
        else {
            println!("SQUAK ERROR");
        }
        // println!("{:?}", answer_location_to_check);

    }

    println!("{:?}", answer_location_to_check);
    let answer = answer_location_to_check.0 * 10000 + answer_location_to_check.1;
    println!("{:?}", answer);

    // for x in 0..GRID_SIZE {
    //     for y in 0..GRID_SIZE {
    //         let location_to_check = (x as i64 +answer_location_to_check.0 - 1,y as i64+answer_location_to_check.1 - 1);

    //         let output = check_location(location_to_check,program.clone());
    //         grid[y][x] = output;

    //         if output  == 1{
    //             point_count +=1;
    //         }
    //     }
    // }

    // print_grid(grid);


    // println!("{:?}", check_100_square(answer_location_to_check,&program));
    // println!("{:?}", point_count);
}

fn check_location(location: (i64,i64),mut program:Vec<i64>) -> i64 {

    // let mut input;
    let mut pc = 0;
    let mut rb = 0;
 
    let input = location.0;
    let output = do_intcode(&mut program,input,pc,rb);
    pc = output.1;
    rb = output.2;

    let input = location.1;
    let output = do_intcode(&mut program,input,pc,rb);
    pc = output.1;
    rb = output.2;
    let output = do_intcode(&mut program,input,pc,rb);

    output.0
}

fn check_100_square(location: (i64,i64),program: &Vec<i64> ) -> i32 {
    let top_left = location;
    let top_right = (location.0 + 99,location.1);
    let bot_left = (location.0,location.1 + 99);
    let bot_right = (location.0 + 99,location.1 + 99);
    let corners = vec![top_left,top_right,bot_left,bot_right];

    for (i,corner) in corners.iter().enumerate() {
        let output = check_location(*corner,program.clone());
        if output == 0 {
            return i as i32;
        }
    }
    
    200
}

fn print_grid(grid : Vec<Vec<i64>>) {
    for y in grid {
        for x in y {
            print!("{}", x);
        }
        println!();
    }
}

fn do_intcode(viktor: &mut Vec<i64>,program_input: i64,pc:usize, rb:usize) -> (i64,usize,usize){
        let mut program_counter = pc;
        let output;
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
                // println!("test");
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
                program_counter += program_increment;
                viktor[address1] = input;
                return (-20,program_counter,relative_base);

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