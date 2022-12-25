pub use self::map::Map;
pub use self::person::Person;


mod person;
mod map;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Point {
    x: usize,
    y: usize
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    East,
    South,
    West,
    North
}

// impl Direction {
//     pub fn iter() -> Iter<'static, Direction> {
//         static DIRECTIONS: [Direction; 4] = [Direction::East, Direction::South, Direction::West, Direction::North];
//         DIRECTIONS.iter()
//     }
// }