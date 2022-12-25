use board::{Person, Map};
mod board;

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
// static FACE_SIZE:usize = 4;

static FILE_CONTENTS: &str = include_str!("input.txt");
static FACE_SIZE:usize = 50;


#[derive(Debug)]
pub enum Path {
    Turn(char),
    Move(i32)
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_path_from_input_line (path_line: &str) -> Vec<Path> {
    let mut path = vec![];
    let mut next = String::new();

    for thing in path_line.chars() {
        if thing.is_alphabetic() {
            path.push(Path::Move(next.parse().unwrap()));
            next = String::new();
            path.push(Path::Turn(thing));
        }
        else {
            next.push(thing);
        }
    }

    if next!= String::new() {
        path.push(Path::Move(next.parse().unwrap()));
    }

    path
}

fn get_things_from_input() -> (Map, Person, Vec<Path>){
    let (map_input, path_input) = FILE_CONTENTS.split_once("\n\n").unwrap();
    let map = Map::new(map_input);
    let person = Person::new(&map.person_start);
    let path_to_take = get_path_from_input_line(path_input);

    (map, person , path_to_take)
}

fn part1 () -> usize {
    let (map, mut person, path_to_take) = get_things_from_input();
    for path in path_to_take {
        person.follow_path(path, &map);
    }
    // map.print_map(Some(&person));


    person.get_password()
}

fn part2 () -> usize{
    let (mut map,  _, path_to_take) = get_things_from_input();
    map.connect_faces();
    for (i, face) in map.faces.iter().enumerate() {
        println!("{i}:{:?}", face.links);
    }

    let mut person = Person::new_cube();
    for path in path_to_take {
        person.follow_path(path, &map)
    }

    
    // for p in person._path_taken.iter() {
    //     println!("{p:?}");
    // }
    // map.print_map(Some(&person));

    

    person.convert_cube_location(&map);
    person.get_password()
}
