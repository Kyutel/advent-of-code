static FILE_CONTENTS: &str = include_str!("input.txt");

fn main() {
    println!("{}", part1());
    println!("{}", part2());

}

#[derive(Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

fn get_rps(play: u8) -> RPS {
    match play {
        65 | 88 => RPS::Rock,
        66 | 89 => RPS::Paper,
        67 | 90 => RPS::Scissors,
        _ => panic!()
    }
}

fn get_score(round: &str) -> i32 {


    let indexable = round.as_bytes();
    let opponent = indexable[0];
    let me = indexable[2];
    let mut score:i32 = (1+ me - 'X' as u8) as i32;

    let opponent = get_rps(opponent);
    let me = get_rps(me);

    score += match (me, opponent) {
        (RPS::Rock, RPS::Rock) => 3,
        (RPS::Rock, RPS::Paper) => 0,
        (RPS::Rock, RPS::Scissors) => 6,
        (RPS::Paper, RPS::Rock) => 6,
        (RPS::Paper, RPS::Paper) => 3,
        (RPS::Paper, RPS::Scissors) => 0,
        (RPS::Scissors, RPS::Rock) => 0,
        (RPS::Scissors, RPS::Paper) => 6,
        (RPS::Scissors, RPS::Scissors) => 3
    };

    score
}

fn get_score_2(round: &str) -> i32{
    let indexable = round.as_bytes();
    let opponent = indexable[0];
    let me = indexable[2];

    let opponent = get_rps(opponent);
    let mut score:i32 = match me {
        88 => 0,
        89 => 3,
        90 => 6,
        _ => panic!()
    };

    score += match (opponent, me) {
            (RPS::Rock, 88) => 3,
            (RPS::Rock, 89) => 1,
            (RPS::Rock, 90) => 2,
            (RPS::Paper, 88) => 1,
            (RPS::Paper, 89) => 2,
            (RPS::Paper, 90) => 3,
            (RPS::Scissors, 88) => 2,
            (RPS::Scissors, 89) => 3,
            (RPS::Scissors, 90) => 1,
            _ => panic!()
        };
    score
}

fn part1() -> i32 {
    FILE_CONTENTS.split('\n')
    .filter(|&row| row != "")
    .map(|row| get_score(row))
    .sum()
}


fn part2() -> i32 {
    FILE_CONTENTS.split('\n')
    .filter(|&row| row != "")
    .map(|row| get_score_2(row))
    .sum()
}