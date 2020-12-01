fn main() {
    part_1();

    part_2();
}

fn part_1() {
    let answer:i32 = include_str!("input.txt").split("\r\n")
    .map(|x| (x.parse::<i32>().unwrap()/3) -2)
    .sum();

    println!("part 1: {:?}", answer);
}

fn part_2() {
    let answer:i32 = include_str!("input.txt").split("\r\n")
    .map(|x| calculate_fuel(x.parse::<i32>().unwrap()))
    .sum();

    println!("part 2: {:?}", answer);
}

fn calculate_fuel(mass:i32) -> i32 {
    let fuel = (mass/3) -2;
    if fuel < 0 {
        return 0;
    }

    return calculate_fuel(fuel) + fuel;
}