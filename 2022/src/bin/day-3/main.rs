use std::collections::HashSet;

static FILE_CONTENTS: &str = include_str!("input.txt");

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());

}

fn get_value_of_item(item: char) -> i32{

    let u8_val = item as u8;
    if u8_val >= 'A' as u8 && u8_val <= 'Z' as u8 {
        return (u8_val - ('A' as u8) + 1 + 26) as i32; 
    }
    else {
        return (u8_val - ('a' as u8) + 1) as i32
    }
}

fn part1() -> i32 {

    let backpacks: Vec<&str> = FILE_CONTENTS.split("\n")
    .filter(|&row| row != "")
    .collect();
    
    let mut total_value = 0;

    for backpack in backpacks {
        // println!("{}", backpack);
        let mut item_set = HashSet::new();
        let pocket_1 = &backpack[..backpack.len()/2];
        let pocket_2 = &backpack[backpack.len()/2..];
        for item in pocket_1.chars(){
            item_set.insert(item);
        }
        for item in pocket_2.chars() {
            if item_set.contains(&item) {
                total_value += get_value_of_item(item);
                break;
            }
        }
    }

    total_value
}

fn get_set_of_backpack(backpack: &str) -> HashSet<char>{
    let mut set = HashSet::new();
    for item in backpack.chars(){
        set.insert(item);
    }
    set
}

fn part2() -> i32{
    let backpacks: Vec<&str> = FILE_CONTENTS.split("\n")
    .filter(|&row| row != "")
    .collect();

    let mut total_value = 0;
    for i in  (0..backpacks.len()).step_by(3){
        let b1 = get_set_of_backpack(backpacks[i]);
        let b2 = get_set_of_backpack(backpacks[i+1]);
        let b3 = get_set_of_backpack(backpacks[i+2]);

        let mut sets = [b1, b2, b3];
        let (intersection, others) = sets.split_at_mut(1);
        let intersection = &mut intersection[0];
        for other in others {
            intersection.retain(|e| other.contains(e));
        }

        let badge = intersection.iter().nth(0).unwrap();
        total_value += get_value_of_item(*badge);
    }

    total_value
}