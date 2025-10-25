use std::collections::{HashMap, HashSet};
use std::cmp::max;

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos3D {
    x: usize,
    y: usize,
    z: usize
}

#[derive(Debug, Clone)]
enum Dir{
    X,
    Y,
    Z
}

#[derive(Debug, Clone)]
struct Brick {
    id: usize,
    initial: (Pos3D, Pos3D),
    current: Pos3D,
    length: usize,
    direction: Dir,
    supporting: Vec<usize>,
    supported_by: HashSet<usize>
}


type Bricks =  HashMap<usize, Brick>;
type Space = Vec<Vec<Vec<usize>>>;

fn map_brick_input(index: usize, line: &str) -> (usize, Brick){
    let (start, end)  = line.split_once('~').unwrap();
    let start_vec: Vec<usize> = start.split(',').map(|x| x.parse().unwrap()).collect();
    let end_vec: Vec<usize> = end.split(',').map(|x| x.parse().unwrap()).collect();

    let start_pos = Pos3D { x: start_vec[0], y: start_vec[1], z: start_vec[2] };
    let end_pos = Pos3D { x: end_vec[0], y: end_vec[1], z: end_vec[2] };

    let direction = if start_pos.x != end_pos.x {
        Dir::X
    }
    else if start_pos.y != end_pos.y {
        Dir::Y
      }
    else if  start_pos.z != end_pos.z{
        Dir::Z
    } 
    else {
        Dir::Z //doesnt really matter in this case
        // panic!()
    };

    let length = match direction {
        Dir::X => (end_pos.x - start_pos.x) + 1,
        Dir::Y => (end_pos.y - start_pos.y) + 1,
        Dir::Z => (end_pos.z - start_pos.z) + 1,
    };
    

    let brick = Brick {
        id: index,
        initial: (start_pos, end_pos),
        current: start_pos,
        direction,
        length,
        supporting: vec![],
        supported_by: HashSet::new()
    };

    // if start_pos.x > end_pos.x || start_pos.y > end_pos.y || start_pos.z > end_pos.z   {
    //     panic!("Bricks are upside down")
    // }


    (brick.id, brick)
    
}

fn get_bricks() -> Bricks{
    FILE_CONTENTS.split('\n').enumerate().map(| (i, line) | map_brick_input(i+1, line)).collect()
}

fn get_max_dimensions(bricks: &Bricks) -> Pos3D {
    let mut max_pos = Pos3D {x:0,y:0,z:0};

    for brick in bricks.values() {
        max_pos.x  = max(max_pos.x, brick.initial.1.x);
        max_pos.y  = max(max_pos.y, brick.initial.1.y);
        max_pos.z  = max(max_pos.z, brick.initial.1.z);
    }
    max_pos
}

fn _print_by_z_layer(space: &Space) {
    for z_layer in space {
        for line in z_layer {
            for tile in line {
                print!("{:?} ", tile);
            }
            println!()
        }
        println!()
    }

}

fn find_lowest_point(space: &Space, yx_pos: &Pos3D) -> usize{

    for z_to_check in (1..yx_pos.z).rev() {
        if space[z_to_check][yx_pos.y][yx_pos.x] != 0 {
            return z_to_check+1
        }
    }

    1
}

fn drop_all_blocks(space: &mut Space, bricks: &mut Bricks) {
    let mut brick_ids:Vec<usize> = bricks.keys().cloned().collect();
    brick_ids.sort_by(|a,b|{
        bricks.get(a).unwrap().current.z.cmp(&bricks.get(b).unwrap().current.z)
    });

    for id in brick_ids {
        let brick = bricks.get_mut(&id).unwrap();

        let mut yx_positions = vec![];

        match brick.direction {
            Dir::X => {for l in 0..brick.length {yx_positions.push(Pos3D{z:brick.current.z, y:brick.current.y, x:brick.current.x+l})}},
            Dir::Y => {for l in 0..brick.length {yx_positions.push(Pos3D{z:brick.current.z, y:brick.current.y+l, x:brick.current.x})}},
            Dir::Z => {yx_positions.push(Pos3D{z:brick.current.z, y:brick.current.y, x:brick.current.x})}
        }

        let lowest_z = yx_positions.iter().map(|yx_pos| find_lowest_point(&space, yx_pos)).max();

        match brick.direction {
            Dir::X => {for l in 0..brick.length {(space[brick.current.z][brick.current.y][brick.current.x+l] = 0);}},
            Dir::Y => {for l in 0..brick.length {(space[brick.current.z][brick.current.y+l][brick.current.x] = 0);}},
            Dir::Z => {for l in 0..brick.length {(space[brick.current.z+l][brick.current.y][brick.current.x] = 0);}}
        };

        brick.current.z = lowest_z.unwrap();

        match brick.direction {
            Dir::X => {for l in 0..brick.length {(space[brick.current.z][brick.current.y][brick.current.x+l] = id);}},
            Dir::Y => {for l in 0..brick.length {(space[brick.current.z][brick.current.y+l][brick.current.x] = id);}},
            Dir::Z => {for l in 0..brick.length {(space[brick.current.z+l][brick.current.y][brick.current.x] = id);}}
        };

        let mut supported_by = HashSet::new();

        match brick.direction {
            Dir::X => {for l in 0..brick.length {supported_by.insert(space[brick.current.z - 1][brick.current.y][brick.current.x+l]);}},
            Dir::Y => {for l in 0..brick.length {supported_by.insert(space[brick.current.z - 1][brick.current.y+l][brick.current.x]);}},
            Dir::Z => {supported_by.insert(space[brick.current.z - 1][brick.current.y][brick.current.x]);}
        };

        brick.supported_by = supported_by.clone();
        brick.supported_by.remove(&0);
        
        for supporting in supported_by {
            let supporting_brick = bricks.get_mut(&supporting);
            if let Some(supporting_brick) = supporting_brick {
                supporting_brick.supporting.push(id);
            }
        }
        

    }

    // println!("{brick_ids:?}");
}


fn check_safe(supporting: &Vec<usize>, bricks: &Bricks) -> bool{
    let mut safe: bool = true;

    for above in supporting.iter() {
        let above_brick = bricks.get(&above).unwrap();
        if above_brick.supported_by.len() == 1 {
            safe = false
        }
    }

    return safe
}

fn find_safe_to_disintegrate(bricks: &Bricks) -> i32 {
    let mut safe_count = 0;

    for (_id, brick) in bricks {
        if check_safe(&brick.supporting, &bricks) {
            safe_count +=1 ;
        }
    }

    safe_count
}

fn part1 () -> i32 {
    let mut bricks = get_bricks();
    // for brick in bricks.iter() {
    //     println!("{:?}", brick);
    // }

    let max_pos = get_max_dimensions(&bricks);

    let mut space: Space = vec![vec![vec![0;max_pos.x+1];max_pos.y+1];max_pos.z+1];

    // println!("{:?}", space.len());
    // println!("{:?}", space[0].len());
    // println!("{:?}", space[0][0].len());

    // place bricks in space make into function if getting confusing or prevent copypaste
    for (id, brick) in bricks.iter() {
        for l in 0..brick.length {
            // println!("pos {:?}, brick_direction {:?} l:{l}", brick.current, brick.direction);
            match brick.direction {
                Dir::X => space[brick.current.z][brick.current.y][brick.current.x + l] = *id,
                Dir::Y => space[brick.current.z][brick.current.y + l][brick.current.x] = *id,
                Dir::Z => space[brick.current.z + l][brick.current.y][brick.current.x] = *id
            }
        }
    }

    drop_all_blocks(&mut space, &mut bricks);
    find_safe_to_disintegrate(&bricks)


    // _print_by_z_layer(&space);
    // let mut brick_ids:Vec<usize> = bricks.keys().cloned().collect();
    // brick_ids.sort_by(|a,b|{
    //     bricks.get(a).unwrap().current.z.cmp(&bricks.get(b).unwrap().current.z)
    // });
    // for id in brick_ids {
    //     println!("{:?}", bricks.get(&id).unwrap());
    // }

    
}


fn disintegrate(brick: &Brick, bricks: &Bricks) -> usize{

    let mut disintegrated_list = HashSet::new();
    disintegrated_list.insert(brick.id);

    let mut falling = vec![brick.id];
    while let Some(fallen) = falling.pop() {
        let fallen_brick = bricks.get(&fallen).unwrap();
        disintegrated_list.insert(fallen_brick.id);
        for next_to_check in fallen_brick.supporting.iter() {
            let next_brick = bricks.get(next_to_check).unwrap();
            let remaining_supports = &next_brick.supported_by - &disintegrated_list;
            if remaining_supports.len() == 0 {
                falling.push(next_brick.id);
            }
        }
    }

    disintegrated_list.len() - 1
}


fn part2 () -> usize {
    let mut bricks = get_bricks();

    let max_pos = get_max_dimensions(&bricks);

    let mut space: Space = vec![vec![vec![0;max_pos.x+1];max_pos.y+1];max_pos.z+1];


    // place bricks in space make into function if getting confusing or prevent copypaste
    for (id, brick) in bricks.iter() {
        for l in 0..brick.length {
            // println!("pos {:?}, brick_direction {:?} l:{l}", brick.current, brick.direction);
            match brick.direction {
                Dir::X => space[brick.current.z][brick.current.y][brick.current.x + l] = *id,
                Dir::Y => space[brick.current.z][brick.current.y + l][brick.current.x] = *id,
                Dir::Z => space[brick.current.z + l][brick.current.y][brick.current.x] = *id
            }
        }
    }

    drop_all_blocks(&mut space, &mut bricks);

    bricks.values().into_iter().map(|brick| disintegrate(brick, &bricks) ).sum()

    // find_safe_to_disintegrate(&bricks)

}
