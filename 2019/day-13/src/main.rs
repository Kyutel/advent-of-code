use std::collections::HashMap;
extern crate termion;

fn main() {
    let mut program = read_file();
    program[0] = 2;
    program.resize(program.len() *  10,0);

    let mut input = 0;
    let mut pc = 0;
    let mut rb = 0;
    let mut finished = false;

    let mut game_grid = HashMap::new();
    let mut score = 0;

    let mut max_x = 0;
    let mut max_y = 0;

    let mut game_started = false;
    let mut current_paddle_x = -1;
    let mut current_ball_x  = -1;

    println!("{}{}", termion::clear::All,termion::cursor::Hide );

    while !finished {
        let mut new_tile = [-1,-1,-1];
        for x in 0..3 {
            let output = do_intcode(&mut program,input,pc,rb);
            pc = output.1;
            rb = output.2;
            new_tile[x] = output.0; 


            if x == 2 && output.0 == 4 {
                current_ball_x = new_tile[0];
            }

            if x == 2 && output.0 == 3 {
                current_paddle_x = new_tile[0];
            }


            if output.0 == -100 {
                finished = true;
                break;
            } 
        }

        if new_tile[0] == -1 && new_tile[1]  == 0{
            score = new_tile[2];
            continue;
        }

        if !finished{
            use std::cmp;
            max_x = cmp::max(max_x, new_tile[0]);
            max_y = cmp::max(max_y, new_tile[1]);
            game_grid.insert((new_tile[0],new_tile[1]), new_tile[2]);
            // game_grid.push(new_tile);
        }

        // println!("{:?}",game_grid.len());
        if game_grid.len() >= 989 {
            game_started = true;
            input = calculate_joystick_input(current_paddle_x,current_ball_x);
            print_game_grid(game_grid.clone(),score,max_x + 1,max_y + 1);
        }

        // print_game_grid(game_grid.clone(),score);

    }


    print_game_grid(game_grid.clone(),score,max_x + 1,max_y + 1);


    

    // print_game_grid(game_grid,score,max_x + 1,max_y + 1);

    // let block_tile_count = game_grid.iter().filter(|&tile| tile[2] == 2).count();

    // println!("{:?}", game_grid);
    // println!("{:?}", block_tile_count);
    

}

fn calculate_joystick_input( current_paddle_x:i64,current_ball_x:i64 ) -> i64{
    // println!("paddle: {:?}, ball: {:?}", current_paddle_x,current_ball_x);
    if current_ball_x > current_paddle_x{
        return 1;
    }
    else if current_ball_x < current_paddle_x{
        return -1;
    }
    else {
        return 0;
    }
}


fn print_game_grid(game_grid: HashMap<(i64,i64),i64>,score: i64,max_x:i64,max_y:i64) {

    print!("{}", termion::cursor::Goto(0, 2));

    for y in 0..max_y {
        for x in 0..max_x {
            let tile_id = game_grid.get(&(x,y));
            if tile_id == None {
                continue;
            }
            match tile_id.unwrap() {
                0 => print!(" "),
                1 => print!("#"),
                2 => print!("x"),
                3 => print!("_"),
                4 => print!("O"),
                _ => print!("E"),
            }
        }
        println!();
    }

    println!();
    println!();
    println!("SCORE: {:?}", score);
    println!();
    println!();
}

fn do_intcode(viktor: &mut Vec<i64>,program_input: i64,pc:usize, rb:usize) -> (i64,usize,usize){
        let mut program_counter = pc;
        let mut output = -2;
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
                viktor[address1] = input;
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