use regex::Regex;

static FILE_CONTENTS: &str = include_str!("input.txt");
static LINE_NO:i32 = 2000000;
static MAX_XY:i32 = 4000000;


// static FILE_CONTENTS: &str = include_str!("test-input.txt");
// static LINE_NO:i32 = 10;
// static MAX_XY:i32 = 20;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Pair {
    sensor: Point,
    beacon: Point
}

impl Pair {
    fn get_distance(&self) -> i32 {
        return  (self.sensor.x - self.beacon.x).abs() + (self.sensor.y - self.beacon.y).abs();
    }
}

fn get_pairs_from_input(input_line: &str, regex: &Regex) -> Pair{
    let matches: Vec<i32> = regex.find_iter(input_line)
    .map(|x| x.as_str().parse().unwrap())
    .collect();

    let sensor = Point{x:matches[0],y:matches[1]};
    let beacon = Point{x:matches[2],y:matches[3]};
    Pair { sensor, beacon }
}


fn get_pairs() -> Vec<Pair> {
    let re = Regex::new(r"-?\d+").unwrap();

    FILE_CONTENTS.split("\n")
    .map(|x| get_pairs_from_input(x, &re)).collect()
    
} 

fn add_where_beacon_is_not_at_line(pair: &Pair,line_no: i32) -> Option<(i32, i32)> {
    let distance_to_beacon = pair.get_distance();
    let distance_to_line = (pair.sensor.y - line_no).abs();
    if distance_to_line > distance_to_beacon {
        return None
    }

    let lower = pair.sensor.x - (distance_to_beacon-distance_to_line);
    let upper = pair.sensor.x + (distance_to_beacon-distance_to_line);

    Some((lower, upper))
}


fn merge_intervals(intervals: &mut Vec<(i32,i32)>) -> Vec<(i32,i32)> {
    intervals.sort_by(|a,b| a.0.cmp(&b.0));

    let mut merged = vec![];
    merged.push(intervals[0]);

    for i in 1..intervals.len() {
        let current = intervals[i];
        let j = merged.len() -1;
        if current.0 >= merged[j].0 && current.0 <= merged[j].1 + 1{
            merged[j].1 = i32::max(current.1, merged[j].1);
        }
        else {
            merged.push(current);
        }
    }

    merged
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());

}


fn part1() -> i32{
    let pairs = get_pairs();
    // let mut lowest_range = i32::MAX;
    // let mut biggest_range = i32::MIN;
    let mut intervals = vec![];
    for pair in pairs {
         if let Some((mut low,mut  high)) = add_where_beacon_is_not_at_line(&pair, LINE_NO){
            if pair.beacon.y == LINE_NO  && low == pair.beacon.x {
                low+=1;
            }
            if pair.beacon.y == LINE_NO  && high == pair.beacon.x {
                high-=1;
            }
            intervals.push((low,high))
         };

    }

    // println!("{:?}", intervals);


    let intervals = merge_intervals(&mut intervals);
    
    // println!("{:?}", intervals);

    let mut total = 0;
    for interval in intervals {
        total += (interval.1 - interval.0) + 1
    }

    total
}

fn part2() -> u128 {
    let pairs = get_pairs();
    // let mut intervals = vec![];
    for line_no in 0..MAX_XY +1 {
        let mut intervals = vec![];
        for pair in pairs.iter() {

            if let Some((low,high)) = add_where_beacon_is_not_at_line(&pair, line_no){
                intervals.push((i32::max(0, low), i32::min(MAX_XY,high)))
            };
        }

        let intervals = merge_intervals(&mut intervals);
        // println!("L {}: {:?}",line_no, intervals);
        if intervals.len() != 1 {
            // println!("L {}: {:?}",line_no, intervals);

            let  x_pos = intervals[0].1 + 1;
            return  (x_pos as u128) * 4000000  +  line_no as u128
        }

    }
    // let intervals = merge_intervals(&mut intervals);

    // println!("{:?}", intervals);
    0
}