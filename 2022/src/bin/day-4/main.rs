static FILE_CONTENTS: &str = include_str!("input.txt");

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_pairs() -> Vec<Vec<Vec<i32>>> {
    FILE_CONTENTS.split("\n")
    .filter(|&row| row != "")
    .map(
        |pair| pair.split(",")
        .map(|ids| ids.split("-").map(|id| id.parse::<i32>().unwrap()).collect()
    ).collect())
    .collect()
}


fn does_contain(ids1: &Vec<i32>, ids2: &Vec<i32>) -> bool{
    return ids1[0] >= ids2[0] && ids1[1] <=ids2[1] 
} 

fn has_overlaps(ids1: &Vec<i32>, ids2: &Vec<i32>) -> bool{
    return ids1[0] >= ids2[0] && ids1[0] <=ids2[1] ||
            ids1[1] >= ids2[0] && ids1[0] <=ids2[1] ||
            ids2[0] >= ids1[0] && ids2[0] <=ids1[1] ||
            ids2[1] >= ids1[0] && ids2[0] <=ids1[1]
}

fn part1() -> i32 {
    let pairs = get_pairs();

    let mut contained_count = 0;
    for pair in pairs {
        if does_contain(&pair[0], &pair[1]) || does_contain(&pair[1], &pair[0]) {
            contained_count += 1;
        }
    }

    contained_count
}
fn part2() -> i32 {
    let pairs = get_pairs();
    let mut overlaps = 0;
    for pair in pairs {
        if has_overlaps(&pair[0], &pair[1]){
            overlaps += 1;
        }
    }

    overlaps
}