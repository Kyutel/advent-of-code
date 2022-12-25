use std::collections::HashSet;

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl FromIterator<i32> for Position {
    fn from_iter<I: IntoIterator<Item=i32>>(iter: I) -> Self {
        let mut i = iter.into_iter();
        let x = i.next().unwrap();
        let y = i.next().unwrap();
        let z = i.next().unwrap();

        Position {x ,y,z}
    }
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_cube_positions()  -> Vec<Position>{
    FILE_CONTENTS.split('\n').map(|x| x.split(',').map(|y| y.parse().unwrap()).collect()).collect()
}

fn get_new_sides(pos: &Position, grid: &HashSet<&Position>) -> i32 {
    let mut new_sides = 6;
    Position {x:pos.x, y:pos.y, z:pos.z};
    let up = Position {x:pos.x, y:pos.y, z:pos.z + 1};
    let down = Position {x:pos.x, y:pos.y, z:pos.z -1};
    let foward = Position {x:pos.x, y:pos.y+1, z:pos.z};
    let back = Position {x:pos.x, y:pos.y-1, z:pos.z};
    let right = Position {x:pos.x+1, y:pos.y, z:pos.z};
    let left = Position {x:pos.x-1, y:pos.y, z:pos.z};
    let pos_to_check = vec![up, down, left, right, foward, back];
    
    for neighbour in pos_to_check {
        if let Some(_) = grid.get(&neighbour) {
            new_sides-=1;
        }
    }
    
    new_sides
}

fn find_air_pockets(grid: &HashSet<&Position>, smallest:Position, biggest: Position) -> HashSet<Position> {
    let mut air_grid: HashSet<Position> = HashSet::new();
    for x in smallest.x..biggest.x+1 {
        for y in smallest.y..biggest.y+1 {
            for z in smallest.z..biggest.z+1 {
                let pos = Position{x,y,z};
                if let None = grid.get(&pos) {
                    // println!("Air Pocket? {:?}", pos);
                    air_grid.insert(pos);
                }
            }
        }
    }
    air_grid
}

fn part1 () -> i32{
    // let grid = vec![vec![vec![]]];
    let mut grid: HashSet<&Position> = HashSet::new();
    let cubes = get_cube_positions();
    let mut total_sides = 0;
    for cube in cubes.iter() {
        // println!("{:?}", cube);
        // total_sides+= get_new_sides(&cube, &grid);
        grid.insert(cube);
    }

    for cube in cubes.iter() {
        total_sides+= get_new_sides(&cube, &grid);
    }
    
    total_sides
}

fn remove_positions_reachable_by_steam(air_grid: &mut HashSet<Position>) {
    let mut pos_to_check = vec![Position {x:1, y:1, z:1}];

    while pos_to_check.len() > 0 {
        let pos = pos_to_check.pop().unwrap();

        let up = Position {x:pos.x, y:pos.y, z:pos.z + 1};
        let down = Position {x:pos.x, y:pos.y, z:pos.z -1};
        let foward = Position {x:pos.x, y:pos.y+1, z:pos.z};
        let back = Position {x:pos.x, y:pos.y-1, z:pos.z};
        let right = Position {x:pos.x+1, y:pos.y, z:pos.z};
        let left = Position {x:pos.x-1, y:pos.y, z:pos.z};
        let dir_to_check = vec![up, down, left, right, foward, back];
        for neighbour in dir_to_check {
            if air_grid.contains(&neighbour) {
                pos_to_check.push(neighbour);
            }
        }
        air_grid.remove(&pos);
    }

}

fn part2 () -> i32{
    // let grid = vec![vec![vec![]]];
    let mut grid: HashSet<&Position> = HashSet::new();
    let cubes = get_cube_positions();
    let mut total_sides = 0;

    let mut smallest_pos = Position{x:200 ,y:200, z:200};
    let mut biggest_pos = Position{x:0, y:0, z:0};


    for cube in cubes.iter() {
        smallest_pos.x = cube.x.min(smallest_pos.x);
        smallest_pos.y = cube.y.min(smallest_pos.y);
        smallest_pos.z = cube.z.min(smallest_pos.z);

        biggest_pos.x = cube.x.max(biggest_pos.x);
        biggest_pos.y = cube.y.max(biggest_pos.y);
        biggest_pos.z = cube.z.max(biggest_pos.z);

        // total_sides+= get_new_sides(&cube, &grid);
        grid.insert(cube);
    }
    // println!("{:?}", smallest_pos);
    // println!("{:?}", biggest_pos);

    for cube in cubes.iter() {
        total_sides+= get_new_sides(&cube, &grid);
    }

    let mut air_grid = find_air_pockets(&grid, smallest_pos, biggest_pos);
    remove_positions_reachable_by_steam(&mut air_grid);
    // println!("air gird  {:?}", air_grid);
    for air in air_grid {
        total_sides-= 6 - get_new_sides(&air, &grid)
    }
    
    
    total_sides
}
