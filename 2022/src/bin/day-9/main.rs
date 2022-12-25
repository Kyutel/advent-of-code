use std::collections::HashSet;

static FILE_CONTENTS: &str = include_str!("input.txt");

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct Position {
    x:i32,
    y:i32
}

enum Direction{
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn from(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!()
        }
    }
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_instructions() -> Vec<(char, i32)> {
    FILE_CONTENTS.split('\n')
    .map(|x| { 
        let y:Vec<&str>=x.split(' ').collect(); 
        (y[0].chars().next().unwrap(), y[1].parse::<i32>().unwrap())
    })
    .collect()
}

fn move_rope(knots: Vec<Position>, direction: Direction, distance: i32) -> (Vec<Position>, HashSet<Position>) {
    let mut visited_positions = HashSet::new();
    let mut x_modifier = 0;
    let mut y_modifier = 0;
    match direction {
        Direction::Up => y_modifier = 1,
        Direction::Down => y_modifier = -1,
        Direction::Left => x_modifier = -1,
        Direction::Right => x_modifier = 1
    }

    let mut knots = knots;
    // let mut head = knots[0];
    // let mut tail = tail;
    for _ in 0..distance  {
        knots[0] = Position { x: knots[0].x + (1*x_modifier), y:  knots[0].y + (1*y_modifier) };
        for x in 1..knots.len(){
            knots[x] = move_towards_next_knot(knots[x], knots[x-1]);
        }
        visited_positions.insert(knots[knots.len()-1]);
        // println!("H: {:?}, T: {:?}", knots[0], knots[1]);
    }

    (knots, visited_positions)
}


fn move_towards_next_knot(tail: Position, head: Position) -> Position {
    let x_diff = head.x - tail.x;
    let y_diff = head.y - tail.y;
    if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
        return tail;
    }
    
    else if x_diff == 0 {
        return Position{x:tail.x, y:tail.y + (y_diff/y_diff.abs()) };
    }
    else if y_diff == 0 {
        return Position{x:tail.x + (x_diff/x_diff.abs()), y:tail.y};
    }
    else {    
        return Position{x:tail.x + (x_diff/x_diff.abs()), y:tail.y + (y_diff/y_diff.abs()) };
    }
} 

fn part1() -> i32 {
    let mut visited_positions = HashSet::new();
    // visited_positions.insert(Position{x:1,y:2});
    
    let head = Position{x:0,y:0};
    let tail = Position{x:0,y:0};
    visited_positions.insert(tail);
    let mut knots = vec![head, tail];

    // let new_head = move_rope(head, Direction::from('U'), 5);
    
    for instruction in get_instructions().into_iter(){
        let new_visited;
        (knots, new_visited) = move_rope(knots, Direction::from(instruction.0), instruction.1);
        visited_positions.extend(&new_visited);
    }



    // println!("{:?}", visited_positions);

    visited_positions.len() as i32
}

fn part2() -> i32 {
    let mut visited_positions = HashSet::new();
    // visited_positions.insert(Position{x:1,y:2});
    
    // let head = Position{x:0,y:0};
    // let tail = Position{x:0,y:0};
    let mut knots = vec![];
    for _ in 0..10 {
        knots.push( Position{x:0,y:0});
    }
    visited_positions.insert(knots[knots.len()-1]);

    // let new_head = move_rope(head, Direction::from('U'), 5);
    
    for instruction in get_instructions().into_iter(){
        let new_visited;
        (knots, new_visited) = move_rope(knots, Direction::from(instruction.0), instruction.1);
        visited_positions.extend(&new_visited);
    }

    visited_positions.len() as i32

}