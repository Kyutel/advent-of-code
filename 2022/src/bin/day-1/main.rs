
static FILE_CONTENTS: &str = include_str!("test-input.txt");

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());

}


fn part1() -> i32 {
    FILE_CONTENTS.split("\n\n")
    .map(|block| block.split('\n')
        .filter(|&row| row != "")
        .map(|row| row.parse::<i32>().unwrap())
        .sum()
    )
    .max().unwrap()
}

fn part2() -> i32{
    let mut total_calories: Vec<i32> = FILE_CONTENTS.split("\n\n")
    .map(|block| block.split('\n')
        .filter(|&row| row != "")
        .map(|row| row.parse::<i32>().unwrap())
        .sum()
    )
    .collect();

    total_calories.sort();
    total_calories.reverse();
    total_calories[..3].into_iter().sum()
}