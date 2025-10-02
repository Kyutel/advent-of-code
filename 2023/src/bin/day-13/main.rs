use std::cmp::min;

static FILE_CONTENTS: &str = include_str!("test-input.txt");
// static FILE_CONTENTS: &str = include_str!("input.txt");

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_land_patterns() -> Vec<Vec<Vec<char>>> {
    FILE_CONTENTS.split("\n\n")
        .map(|grid| grid.split('\n')
            .map(|line| line.chars().collect() ).collect()
        ).collect()
}

fn get_vertical_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let grid_height = grid[0].len(); 
    let mut vertical_grid = vec![vec![];grid_height];
    for line in grid {
        for (index, char) in line.iter().enumerate() {
            vertical_grid[index].push(*char);
        }
    }

    return vertical_grid

}


fn check_for_mirror(left_side: Vec<Vec<char>>, right_side : Vec<Vec<char>>) -> bool {
    // let reversed_left: Vec<Vec<char>> = left_side.clone().reverse();
    let columns_to_check = min(left_side.len(), right_side.len());

    for x in 0..columns_to_check {
        if left_side[left_side.len() - 1 - x] != right_side[x] {
            return false
        }
    }

    return true

}

fn part1 () -> usize {
    let land_patterns = get_land_patterns();
    let mut column_count = 0;
    let mut row_count = 0;

    // println!("{:?}", land_patterns)
    for grid in land_patterns {
        // println!("{:?}", grid);
        // println!("{:?}", get_vertical_land_pattern(&grid));
        // println!("");
        let vert_grid: Vec<Vec<char>> = get_vertical_grid(&grid);
        let mut found_already = false;

        for col_num in 1..vert_grid.len() {
            if check_for_mirror(vert_grid[0..col_num].to_vec(), vert_grid[col_num..vert_grid.len()].to_vec()) {
                column_count += col_num;
                found_already = true;
                break;
            };
        }


        for row_num in 1..grid.len() {
            if check_for_mirror(grid[0..row_num].to_vec(), grid[row_num..grid.len()].to_vec()) {
                // println!("row {:?}: {:?}", row_num,grid[0..row_num].to_vec());
                if found_already {
                    println!("warning")
                }
                row_count += row_num;
                break;
            };
        }

        

    }
    

    return column_count + (100 * row_count)
}


fn smudge_check (left: &Vec<char>, right: &Vec<char>)  -> bool {
    let mut non_matching = 0;
    for (x,y) in left.iter().zip(right) {
        if x!=y {
            non_matching+=1;
            if non_matching > 1 {
                return false;
            }
        }
        
    }
    return true
}

fn check_for_mirror_with_smudge(left_side: Vec<Vec<char>>, right_side : Vec<Vec<char>>) -> bool {
    // let reversed_left: Vec<Vec<char>> = left_side.clone().reverse();
    let columns_to_check = min(left_side.len(), right_side.len());

    let mut smudge_match = false;

    for x in 0..columns_to_check {
        let left_to_check = &left_side[left_side.len() - 1 - x];
        let right_to_check = &right_side[x];

        if left_to_check != right_to_check {
            if smudge_match {
                return false
            }
            else {
                if smudge_check(left_to_check, right_to_check) {
                    smudge_match = true;
                }
                else {
                    return  false;
                }
            }
        }
    }

    if smudge_match {
        return true
    }
    else {
        return false
    }


}

fn part2 () -> usize{
let land_patterns = get_land_patterns();
    let mut column_count = 0;
    let mut row_count = 0;

    for grid in land_patterns {

        let vert_grid: Vec<Vec<char>> = get_vertical_grid(&grid);
        let mut found_already = false;

        for col_num in 1..vert_grid.len() {
            if check_for_mirror_with_smudge(vert_grid[0..col_num].to_vec(), vert_grid[col_num..vert_grid.len()].to_vec()) {
                column_count += col_num;
                found_already = true;
                break;
            };
        }

        for row_num in 1..grid.len() {
            if check_for_mirror_with_smudge(grid[0..row_num].to_vec(), grid[row_num..grid.len()].to_vec()) {
                if found_already {
                    println!("warning")
                }
                row_count += row_num;
                break;
            };
        }

        

    }

    return column_count + (100 * row_count)



}
