use std::{collections::HashMap, vec};

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
// static FILE_CONTENTS: &str = include_str!("test-input2.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");
// static FILE_CONTENTS_2: &str = include_str!("test-input3.txt");
static FILE_CONTENTS_2: &str = include_str!("input.txt");


#[derive(Debug)]
struct Node {
    location: String,
    left: String,
    right: String
}

fn parse_map() -> (Vec<char>, HashMap<String, Node>) {
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let lines: Vec<&str> = FILE_CONTENTS.split("\n").collect();
    let directions: Vec<char> = lines[0].chars().collect();
    for line in lines[2..].iter() {
        nodes.insert(line[0..3].to_string(), Node { location: line[0..3].to_string(), left: line[7..10].to_string(), right: line[12..15].to_string()});
    }

    (directions, nodes)
}

fn parse_map_two() -> (Vec<char>, HashMap<String, Node>, Vec<String>) {
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let lines: Vec<&str> = FILE_CONTENTS_2.split("\n").collect();
    let directions: Vec<char> = lines[0].chars().collect();
    let mut starting_nodes : Vec<String> = Vec::new();
    for line in lines[2..].iter() {
        let label = line[0..3].to_string();
        nodes.insert(label.clone(), Node { location:label.clone(), left: line[7..10].to_string(), right: line[12..15].to_string()});
        if label.chars().nth(2) == Some('A') {
            starting_nodes.push(label.clone());
        } 
    }


    (directions, nodes, starting_nodes)
}


fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn part1 ()  -> i32{

    let (directions, nodes) = parse_map();
    // println!("{:?}", directions);
    // println!("{:?}", nodes);

    let mut current = nodes.get("AAA").unwrap();
    let mut next_direction = 0;
    let mut answer = 0;

    while current.location != "ZZZ".to_string() {
        let next_d: char = directions[next_direction];
        if next_d == 'L' {
            current = nodes.get(&current.left).unwrap()
        }
        else {
            current =  nodes.get(&current.right).unwrap()
        }

        next_direction = (next_direction + 1) % directions.len();
        answer +=1 ;

    }

    answer
}


fn part2 () -> i64 {
    let (directions, nodes, mut travel_nodes) = parse_map_two();
    // println!("{:?}", nodes);
    // println!("{:?}", travel_nodes);
    let mut finished = false;

    let mut next_direction = 0;
    let mut answer = 0;
    let mut exit_count = vec![];

    while !finished {
        finished = true;
        let next_d: char = directions[next_direction];

        for (_, node_str) in travel_nodes.iter_mut().enumerate() {
            let node = nodes.get(node_str).unwrap();
            if next_d == 'L' {
                *node_str = node.left.clone();
            }
            else {
                *node_str =  node.right.clone();
            }

            if node_str.chars().nth(2) != Some('Z') {
                finished = false;
            }
            else {
                // println!("i: {:?}, str: {:?}, count {:?} ", i, node_str, answer );
                exit_count.push(answer + 1);
                if exit_count.len() == 6 {
                    finished = true;
                    break;
                }
            }
        }

        
        next_direction = (next_direction + 1) % directions.len();
        answer +=1 ;
        // println!("{:?}", travel_nodes);
    }
    println!("{:?}", exit_count);

    lcmm(exit_count)
}



fn gcd(a: i64 , b: i64) -> i64{
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

fn lcm(a: i64, b: i64) -> i64 {
    return a * b / gcd(a, b)
}

fn lcmm(numbers: Vec<i64>)  -> i64{
    let result = numbers.into_iter().reduce(|a,b| lcm(a, b)).unwrap();
    return result
}