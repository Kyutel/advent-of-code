static FILE_CONTENTS: &str = include_str!("input.txt");

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_trees_from_input() -> Vec<Vec<u32>> {
    FILE_CONTENTS.split("\n")
    .map(|x| x.chars().map(|t| t.to_digit(10).unwrap()).collect())
    .collect()
}

fn check_visible(x:usize, y:usize, trees: &Vec<Vec<u32>>) -> bool{
    let height = trees[x][y];
    let mut east = true;
    let mut west = true;
    let mut north = true;
    let mut south = true;

    for x_to_check in 0..x {
        if trees[x_to_check][y] >= height{
            west = false;
        } 
    }
    for x_to_check in x+1..trees.len() {
        if trees[x_to_check][y] >= height{
            east = false;
        } 
    }

    for y_to_check in 0..y {
        if trees[x][y_to_check] >= height{
            north = false;
        }
    }

    for y_to_check in y+1..trees[x].len() {
        if trees[x][y_to_check] >= height{
            south = false;
        }
    }

    return north  || east || south || west;
}

fn part1() -> i32{
    let mut visible_trees = 0;
    let trees = get_trees_from_input();

    visible_trees += (trees.len()*2 + trees[0].len()*2 - 4) as i32;

    for x in 1..trees.len() -1 {
        for y in 1..trees[x].len() -1 {
            if check_visible(x, y, &trees) {
                visible_trees+=1;
            }
            // println!("{:?}", trees[x][y]);
        }
    }


    // for tree_row in trees {
    //     println!("{:?}", tree_row);
    // }

    visible_trees
}

fn get_scenic_score(x:usize, y:usize, trees: &Vec<Vec<u32>>) -> i32{
    let height = trees[x][y];

    let mut east = 0;
    let mut west = 0;
    let mut north = 0;
    let mut south = 0;

    for x_to_check in (0..x).rev() {
        east+=1;
        if trees[x_to_check][y] >= height{
            break;
        } 
    }
    for x_to_check in x+1..trees.len() {
        west+=1;
        if trees[x_to_check][y] >= height{
            break;
        } 
    }

    for y_to_check in (0..y).rev() {
        north +=1;
        if trees[x][y_to_check] >= height{
            break;
        }
    }

    for y_to_check in y+1..trees[x].len() {
        south +=1;
        if trees[x][y_to_check] >= height{
            break;
        }
    }

    // println!("{},{},{},{}", east, west, north, south);

    east*west*north*south
}

fn part2() -> i32 {
    let trees = get_trees_from_input();
    let mut best_scenic_score = 0;

    // println!("y:{}", trees[0][2]);

    // println!("KK:{}", get_scenic_score(1,2,&trees));

    for x in 0..trees.len() {
        for y in 0..trees[x].len() {
            let scenic_score = get_scenic_score(x, y, &trees);
            best_scenic_score = std::cmp::max(scenic_score, best_scenic_score);
        }

    }

    best_scenic_score
}