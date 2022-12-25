use std::collections::HashMap;

use chamber::Chamber;
use chamber::rock::{Rock, RockShape};

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");
mod chamber;


fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn part1 () -> usize{
    let mut chamber = Chamber::new();
    
    let mut next_rock = Rock::get_next_rock(None, chamber.tallest_point);
    let mut next_jet = 0;
    let jet_pattern = FILE_CONTENTS.chars().collect();
    
    let number = 2022;

    for _ in 0..number as i64 {
        next_jet = chamber.drop_rock(&mut next_rock, &jet_pattern, next_jet);
        next_rock = Rock::get_next_rock(Some(next_rock), chamber.tallest_point);
    }

    // println!("{}", chamber);

    println!("");

    
    chamber.tallest_point
}


fn part2 ()  -> i64{
    let mut chamber = Chamber::new();
    
    let mut next_rock = Rock::get_next_rock(None, chamber.tallest_point);
    let mut next_jet = 0;
    let jet_pattern = FILE_CONTENTS.chars().collect();
    let mut history: HashMap<(RockShape, usize, usize, String), (i64, usize)> = HashMap::new();
    
    let number_of_rocks: i64 = 1000000000000;


    let mut first = 0;
    let mut repeating = 0;
    let mut height_per_dup = 0;
    let mut _current = 0;
    for i in 0..number_of_rocks as i64 {
        next_jet = chamber.drop_rock(&mut next_rock, &jet_pattern, next_jet);

        let top_row: String = chamber.items[&chamber.items.len() -1].iter().collect();
        if (next_rock.shape, next_rock.solid_tiles[0].x, next_jet) == (RockShape::Vertical, 2, 1465) {
            // println!("i: {}, {:?}", i, chamber.tallest_point);
        }

        match history.get(&(next_rock.shape, next_rock.solid_tiles[0].x, next_jet, top_row.to_owned())) {
            None => {history.insert((next_rock.shape, next_rock.solid_tiles[0].x, next_jet, top_row.to_owned()), (i, chamber.tallest_point));} ,
            Some(x) => {
                
                first = x.0;
                repeating = i - x.0;
                height_per_dup = (chamber.tallest_point - x.1) as i64;
                _current = i;
                let remaining = (number_of_rocks% repeating) - first;
                if remaining == 0 {
                    break;
                }
            }
        };
        
        next_rock = Rock::get_next_rock(Some(next_rock), chamber.tallest_point);
    }


    let remaining = (number_of_rocks% repeating) - first;
    let dup_multi = height_per_dup * (((number_of_rocks - (first+remaining)) / repeating)-1 );
    
    
    // println!("dup at {}", first);
    // println!("current {}", current);
    // println!("repeat every {}", repeating);
    // println!("multi subtrct by {}", (current-first)/ repeating);
    // println!("{} times", (((number_of_rocks - (first+remaining)) / repeating)-1 ));
    // println!("height every loop {}", height_per_dup);
    // println!("remaining {}", remaining );
    
    // println!("multiplied is {}", dup_multi );

    // println!("{}", chamber);
     

    dup_multi + (chamber.tallest_point as i64) -1

}
