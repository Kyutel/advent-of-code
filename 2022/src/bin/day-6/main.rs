use std::collections::HashSet;

static FILE_CONTENTS: &str = include_str!("input.txt");
// static TEST_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
// static TEST_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
// static TEST_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
// static TEST_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
// static TEST_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn part1() -> i32 {
    let input_str = FILE_CONTENTS;
    let markers:Vec<char> = input_str.chars().collect();
    for i in 3..markers.len() {
        let mut window = HashSet::new();
        window.insert(markers[i-3]);
        window.insert(markers[i-2]);
        window.insert(markers[i-1]);
        window.insert(markers[i]);

        if window.len() == 4 {
            return (i+1) as i32;
        }
    }
    -1
}

fn part2() -> i32 {
    let input_str = FILE_CONTENTS;
    let markers:Vec<char> = input_str.chars().collect();
    for i in 13..markers.len() {
        let mut window = HashSet::new();
        for x in 0..14 {
            window.insert(markers[i-x]);
        }


        if window.len() == 14 {
            return (i+1) as i32;
        }
    }
    -1
}