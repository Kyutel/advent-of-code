use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let filename = "src/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
    	let line = line.unwrap().to_string();
    	let mass = line.parse::<i32>().unwrap();
    	let fuel = calculate_fuel(mass);
    	sum += fuel;
    }

    println!("{}",sum );

}

fn calculate_fuel(mass:i32) -> i32 {
	let fuel = (mass/3) -2;
	if fuel < 0{
		return 0;
	}
	return calculate_fuel(fuel) + fuel;
}
