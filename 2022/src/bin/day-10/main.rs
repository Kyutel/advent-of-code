static FILE_CONTENTS: &str = include_str!("input.txt");

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_instructions() -> Vec<(&'static str, i32)> {
    FILE_CONTENTS.split('\n')
    .map(|x| {
        let y:Vec<&str> = x.split(' ').collect();
        let number = match y.get(1) {
            Some(z) => z.parse::<i32>().unwrap(),
            None => 0
        };
        return (y[0], number)
    }).collect()
}

fn part1() -> i32 {
    let mut cycle = 1;
    let mut signal_strength = Vec::new();
    let mut register_x = 1;

    for instruction in get_instructions().iter() {
        // println!("{:?}, {}, {}", instruction, cycle, register_x);
        let operation = instruction.0;
        if cycle > 220 {
            break;
        }

        if cycle % 40 == 19 && operation != "noop"{
            signal_strength.push((cycle+1) * register_x);
        }
        else if cycle % 40 == 20{
            signal_strength.push(cycle* register_x);
        }

        if operation == "addx" {
            register_x += instruction.1;
            cycle+=1
        }
        cycle+=1
    }
    println!("{:?}", signal_strength);
    
    signal_strength.iter().sum()
}

fn apply_pixel(screen: &mut [[char;40]; 6], cycle:i32 ,pixel_position: i32) {
    let row = ((cycle -1)/40) as usize;
    let column = ((cycle-1)%40) as usize;

    println!("{}: {}, {}, p!{}", cycle, row, column, pixel_position);
    let cycle = (cycle-1)%40;

    if (pixel_position-1..pixel_position+2).contains(&cycle) {
        screen[row][column] = '#';
    }
    else {
        screen[row][column] = '.';
    }

    
}

fn part2() {
    let mut cycle = 1;
    let mut register_x = 1;
    let mut screen= [['!';40];6];

    for instruction in get_instructions().iter() {
        // println!("{:?}, {}, {}", instruction, cycle, register_x);

        let operation = instruction.0;
        let value = instruction.1;
        apply_pixel(&mut screen, cycle, register_x);

        if operation == "addx" {
            cycle+=1;
            apply_pixel(&mut screen, cycle, register_x);
            register_x += value;
        }
        cycle+=1
    }

    for row in screen{
        for pixel in row {
            print!("{}", pixel);
        }
        println!();
    }
}