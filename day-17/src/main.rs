fn main() {
    let mut program = read_file();
    program.resize(program.len() *  10,0);

    program[0] = 2;

    // let mut input = 'A' as u8 as i64;
    let a = "L,10,R,10,L,10,L,10\n";
    let b = "R,10,R,12,L,12\n";
    let c = "R,12,L,12,R,6\n";

    let main_function = "A,B,A,B,C,C,B,A,B,C\n";

    let mut pc = 0;
    let mut rb = 0;
    // let mut finished = false;
    let mut grid = Vec::new();
    let mut current_line = Vec::new();
    let total_input = main_function.to_string() + &a.to_string() + &b.to_string() + &c.to_string() + &"y\n".to_string();
    println!("{:?}", total_input);

    let mut char_iter = total_input.chars();
    let mut input = char_iter.next().unwrap() as u8 as i64;
    // let mut input = char_iter.next().unwrap() as u8 as i64;
    // let mut input = char_iter.next().unwrap() as u8 as i64;
    println!("{:?}", input);

    loop {

        let output = do_intcode(&mut program,input,pc,rb);
        pc = output.1;
        rb = output.2;
        if output.0  == -100 {
            // finished = true;
            break;
        }
        if output.0 == -20 {
            let next = char_iter.next();
            if next != None {
                input = next.unwrap() as u8 as i64;
            }
        }

        if output.0 > std::u8::MAX as i64 {
            println!("Dust collected {:?}", output.0);
            continue;
        }
        let char_output:char = output.0 as u8 as char;
        print!("{}", char_output);

        if char_output == '\n' {
            grid.push(current_line);
            current_line = Vec::new();
        }
        else {
            current_line.push(char_output);
        }

        
    }


    // let cross_roads = find_cross_roads(&mut grid);
    // print_grid(grid);

    // println!("{:?}", cross_roads[0]);
    // let calibrate_num:usize = cross_roads.iter()
    // .map(|x| x.0 * x.1)
    // .sum();

    // println!("{:?}", calibrate_num);
}

fn find_cross_roads(grid: &mut Vec<Vec<char>>) -> Vec<(usize,usize)> {
    let mut cross_roads = Vec::new();

    for y in 1..grid.len() - 2 {
        for x in 1..grid[y].len() - 1 {
            let cell = grid[y][x];
            let cell_left = grid[y][x-1];
            let cell_right = grid[y][x+1];
            let cell_up = grid[y-1][x];
            let cell_down = grid[y+1][x];
            let cell_cross = [cell,cell_left,cell_right,cell_up,cell_down];

            let mut is_crossroad = true;
            for c in cell_cross.iter() {
                if *c != '#' {
                    is_crossroad = false;
                }
            }

            if is_crossroad {
                cross_roads.push((x,y));
                grid[y][x] = 'O';
            }
        }
    }

    cross_roads
}

fn print_grid(grid : Vec<Vec<char>>) {
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
                println!("program exit");
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