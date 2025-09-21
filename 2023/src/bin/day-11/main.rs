use std::{cmp::{max, min}, collections::HashSet};

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

type Image = Vec<Vec<char>>;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

fn _print_image(map: &Image) {
    for line in map {
        for tile in line {
            print!("{}", tile)
        }
        println!()
    }
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_image_from_input() -> Image {
    FILE_CONTENTS.split("\n").map(|line| line.chars().collect()).collect()
}

fn get_empty_columns_and_rows(image: &Image) -> (HashSet<usize>, HashSet<usize>) {
    let (populated_columns, populated_rows) = get_non_empty_columns_and_rows(&image);
    let empty_columns = &HashSet::from_iter(0..image[0].len()) - &populated_columns;
    let empty_rows = &HashSet::from_iter(0..image.len()) - &populated_rows;

    (empty_columns, empty_rows)
}

fn get_non_empty_columns_and_rows(image: &Image) -> (HashSet<usize>, HashSet<usize>){
    let mut populated_columns = HashSet::new();
    let mut populated_rows = HashSet::new();

    for (y, line) in image.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if *tile != '.' {
                populated_columns.insert(x);
                populated_rows.insert(y);
            }
        }
    }

    (populated_columns, populated_rows)
}

// fn fill_column(image, )

fn grow_image(image: Image, populated_columns: HashSet<usize>, populated_rows: HashSet<usize>) ->  Image {
    let mut offset = 0;
    let mut new_image = image.clone();

    for column_index in 0..image[0].len() {
        if !populated_columns.contains(&column_index) {
            for line in new_image.iter_mut() {
                line.insert(column_index + offset, '.');
            }
            offset += 1;
        }
    }
    offset = 0;
    for row_index in 0..image.len() {
        if !populated_rows.contains(&row_index) {
            new_image.insert(row_index+offset, vec!['.';new_image[0].len()]); 
            offset +=1;
        }
    }
    return new_image
}

fn find_galaxy_points(image: &Image) -> Vec<Point> {
    let mut points = vec![];
    for (y, line) in image.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if *tile == '#' {
                points.push(Point { x: x as i32, y: y as i32});
            }
        }
    }

    points
}


fn part1 () -> i32{
    let image: Vec<Vec<char>> = get_image_from_input();
    let (populated_columns, populated_rows) = get_non_empty_columns_and_rows(&image);

    let image =grow_image(image, populated_columns, populated_rows);

    // print_image(&image);

    let mut galaxy_points = find_galaxy_points(&image);

    let mut sum = 0;

    // println!("{:?}", galaxy_points);

    while galaxy_points.len() > 0 {
        let next = galaxy_points.pop().unwrap();
        for point in galaxy_points.iter() {
            sum += (next.x - point.x).abs() + (next.y - point.y).abs()
        }
    }

    return sum}

fn get_distance(first: &Point, second: &Point, empty_columns: &HashSet<usize>, empty_rows: &HashSet<usize>, gap_size: usize) -> usize {
    let mut x_gap_count = 0 as usize;
    let mut y_gap_count = 0 as usize;

    let min_x = min(first.x, second.x) as usize;
    let max_x = max(first.x, second.x) as usize;

    let min_y = min(first.y, second.y) as usize;
    let max_y = max(first.y, second.y) as usize;
    
    
    for val in min_x..max_x {
        if empty_columns.contains(&val) {
            x_gap_count +=1;
        }
    }
    
    for val in min_y..max_y {
        if empty_rows.contains(&val) {
            y_gap_count +=1;
        }
    }
    // println!("{:?} {:?}", first, second);
    // println!("{:?} {:?}", x_gap_count, y_gap_count);
    

    return (max_x + (x_gap_count * (gap_size-1)) - min_x ) + (max_y + (y_gap_count * (gap_size-1)) - min_y );

}

fn part2 () -> usize {
    let image: Vec<Vec<char>> = get_image_from_input();
    let (empty_columns, empty_rows) = get_empty_columns_and_rows(&image);

    // let image =grow_image(image, populated_columns, populated_rows);

    // print_image(&image);

    let mut galaxy_points = find_galaxy_points(&image);



    let mut sum = 0;

    // println!("{:?}", galaxy_points);

    while galaxy_points.len() > 0 {
        let next = galaxy_points.pop().unwrap();
        for point in galaxy_points.iter() {
            sum += get_distance(&next, point, &empty_columns, &empty_rows, 1000000 );
        }
    }


    return sum
}
