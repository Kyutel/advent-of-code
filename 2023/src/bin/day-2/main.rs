// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}


fn get_valid_games(row :&str) -> bool {
    let colon_index = row.find(":").unwrap() + 2;
    let temp:Vec<&str> = row[colon_index..].split("; ").collect();
    let game:Vec<Vec<&str>> = temp.iter().map(|x| x.split(", ").collect()).collect();
    for rounds in game {
       for pull in rounds {
            let split_index = pull.find(" ").unwrap() + 1;
            let colour = &pull[split_index..];
            let count:u32 = pull[..split_index-1].parse().unwrap();
            if colour == "red"{
                if count > 12 {return false}
            }
            else if colour == "green" {
                if count > 13 {return false}
            }
            else if colour == "blue" {
                if count > 14 {return false}
            }
        }
    }

    return true;

}

fn get_game_power(row :&str) -> u32 {
    let colon_index = row.find(":").unwrap() + 2;
    let temp:Vec<&str> = row[colon_index..].split("; ").collect();
    let game:Vec<Vec<&str>> = temp.iter().map(|x| x.split(", ").collect()).collect();

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;


    for rounds in game {
        for pull in rounds {
             let split_index = pull.find(" ").unwrap() + 1;
             let colour = &pull[split_index..];
             let count:u32 = pull[..split_index-1].parse().unwrap();

            if colour == "red" {
                if count > max_red {max_red = count}
            }
            else if colour == "green" {
                if count > max_green {max_green = count}
            }
            else if colour == "blue" {
                if count > max_blue {max_blue = count}
            }
        }
    }

    return max_red * max_green * max_blue
}

fn part1 () {
    let answer = FILE_CONTENTS.split("\n")
    .map(|row|get_valid_games(row))
    .enumerate().fold(0, |acc: usize, (i, valid)| if valid { acc + i+1} else {acc} );
    
    println!("{:?}", answer)
}

fn part2 () {
    let answer:u32 = FILE_CONTENTS.split("\n")
    .map(|row|get_game_power(row)).sum();

    println!("{:?}", answer)


}
