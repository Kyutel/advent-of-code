
// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}


fn get_races(input: &str) -> Vec<Race> {
    let lines:Vec<&str> = input.split("\n").collect();
    let times:Vec<&str> = lines[0].split_whitespace().collect();
    let distance:Vec<&str> = lines[1].split_whitespace().collect();

    let races:Vec<Race> = times[1..].iter().zip(distance[1..].iter())
        .map(|(time, distance)| Race {
            distance:distance.parse::<usize>().unwrap(),
            time: time.parse::<usize>().unwrap()}
        ).collect();

    return races
}

fn get_races_part_2(input: &str) -> Race {
    let lines:Vec<&str> = input.split("\n").collect();
    let times:Vec<&str> = lines[0].split(":").collect();
    let distances:Vec<&str> = lines[1].split(":").collect();

    let time: String = times[1].split_whitespace().collect();
    let distance: String = distances[1].split_whitespace().collect();

    return Race {
        time: time.parse::<usize>().unwrap(),
        distance: distance.parse::<usize>().unwrap()
    };
}


fn part1() -> i32 {
    let races = get_races(FILE_CONTENTS);

    let mut answer = 1;

    for race in races {
        let mut winning_count = 0;
        for t in 0..race.time+1 {
            let score =  t*(race.time - t);
            if score > race.distance {
                winning_count += 1;
            }
        }
        answer *= winning_count;
    }

    return answer 
}

fn part2 () -> i32 {
    let race = get_races_part_2(FILE_CONTENTS);
    let mut winning_count = 0;
    for t in 0..race.time+1 {
        let score =  t*(race.time - t);
        if score > race.distance {
            winning_count += 1;
        }
    }

    return winning_count
    // answer *= winning_count;
}


