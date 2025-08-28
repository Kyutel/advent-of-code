use std::collections::HashSet;

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_numbers_from_input(row: &str) -> Vec<HashSet<u32>> {
    let colon_index = row.find(":").unwrap() + 2;
    
    return row[colon_index..].split(" | ").map(|cards| cards.split_whitespace().map(|no|no.parse().unwrap()).collect()).collect();

}

fn part1 () -> u32 {
    let card_sets:  Vec<Vec<HashSet<u32>>> = FILE_CONTENTS.split("\n").map(|row| get_numbers_from_input(row)).collect();
    // let points = 
    
    card_sets.into_iter().map(|sets| sets[0].intersection(&sets[1]).collect::<HashSet<_>>().len() as u32)
    .filter(|matches|matches > &0)
    .map(|matches| 2_u32.pow(matches-1)).sum()

}


fn get_winnings(match_index: usize, card_matches: &Vec<usize>) -> usize {
    let subwinning: usize = (match_index+1..match_index+card_matches[match_index]+1).collect::<Vec<usize>>().iter().map(|next_index| get_winnings(*next_index, &card_matches)).sum();
    return 1 + subwinning

}

fn part2 ()-> usize  {
    let card_sets:  Vec<Vec<HashSet<u32>>> = FILE_CONTENTS.split("\n").map(|row| get_numbers_from_input(row)).collect();
    let matches: Vec<usize> = card_sets.into_iter().map(|sets| sets[0].intersection(&sets[1]).collect::<HashSet<_>>().len()).collect();
    
    return (0..matches.len()).collect::<Vec<usize>>().iter().map(|match_index| get_winnings(*match_index, &matches)).sum()

}
