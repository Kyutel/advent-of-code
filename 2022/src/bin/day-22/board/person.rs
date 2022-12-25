use crate::FACE_SIZE;

use super::{Direction, Point, Map};
use super::super::Path;

#[derive(Debug)]
pub struct Person {
    location: Point,
    facing: Direction,
    at_face: usize,
    using_cube_travel: bool,
    pub _path_taken : Vec<(Point, Direction)>
}

impl Person {
    pub fn new(start_point: &Point) -> Person{
        let location = start_point.clone();
        let facing = Direction::East;
        let _path_taken = vec![];

        Person { location, facing, _path_taken, at_face:0, using_cube_travel: false}
    }

    pub fn new_cube() -> Person {
        let location = Point { x: 0, y: 0 };
        let facing = Direction::East;
        let _path_taken = vec![];

        Person { location, facing, _path_taken, at_face:0, using_cube_travel : true}
    }

    fn change_direction(&mut self, rotation: char) {
        match self.facing {
            Direction::East => 
                if rotation == 'R' { self.facing = Direction::South}
                else { self.facing = Direction::North},
            Direction::South => 
                if rotation == 'R' { self.facing = Direction::West}
                else { self.facing = Direction::East},
            Direction::West => 
                if rotation == 'R' { self.facing = Direction::North}
                else { self.facing = Direction::South},
            Direction::North => 
                if rotation == 'R' { self.facing = Direction::East}
                else { self.facing = Direction::West},
        }
    }

    fn get_next_position_if_valid(&self, map: &Map) -> Result<Point, &'static str> {
        let mut new_x = self.location.x as i32;
        let mut new_y = self.location.y as i32;
        let mut something_found = false;
        let mut next_location = self.location.clone();

        while !something_found {
            match self.facing {
                Direction::East => new_x+=1,
                Direction::South => new_y+=1,
                Direction::West => new_x-=1,
                Direction::North => new_y-=1,
            };
            if new_x < 0 {
                new_x = map.biggest_point.x as i32;
            }
            else if new_x > map.biggest_point.x as i32{
                new_x = 0;
            }
            if new_y < 0 {
                new_y = map.biggest_point.y as i32;
            }
            else if new_y > map.biggest_point.y as i32{
                new_y = 0;
            }

            next_location = Point { x:new_x as usize, y:new_y as usize};
            if let Some(tile) =  map.tiles.get(&next_location) {
                if *tile == '.'{
                    something_found = true;
                }else{
                    return Err("Wall");
                }
            }

        }

        return Ok(next_location)
    }

    fn get_new_face_and_pos(&mut self, new_x: i32, new_y: i32, map: &Map) -> (Point, usize, usize) {
        let mut edge = None;
        let mut new_x = new_x;
        let mut new_y = new_y;
        let face_size = FACE_SIZE as i32;
        if new_x < 0 {
            new_x = face_size - 1;
            edge = Some(Direction::West);
        }
        else if new_x >= FACE_SIZE as i32 {
            new_x = 0;
            edge = Some(Direction::East);
        }
        else if new_y < 0 {
            new_y = face_size -1;
            edge = Some(Direction::North);
        }
        else if new_y >= FACE_SIZE as i32 {
            new_y = 0;
            edge = Some(Direction::South);
        }

        if edge.is_none() {
            return(Point { x:new_x as usize, y:new_y as usize}, self.at_face, 0); 
        }

        let edge = edge.unwrap();
        let new_face = map.faces[self.at_face].links[edge as usize].unwrap().linked_to;
        let rotation = map.faces[self.at_face].links[edge as usize].unwrap().rotation;


        if rotation == 1 {
            let temp_x = new_x;
             new_x = new_y;
             new_y = (face_size - 1) - temp_x;
        }
        else if rotation == 3 {
            let temp_x = new_x;
             new_x = (face_size - 1) - new_y;
             new_y = temp_x;
        }
        else if rotation == 2 {
             new_x = (face_size - 1) - new_x;
             new_y = (face_size - 1) - new_y;

        }

        let new_point = Point { x:new_x as usize, y:new_y as usize};

        (new_point, new_face, rotation)
    }

    fn get_next_position_if_valid_cube(&mut self, map: &Map) -> Result<Point,  &'static str> {
        let mut new_x = self.location.x as i32;
        let mut new_y = self.location.y as i32;
        // let mut next_location = self.location.clone();

        match self.facing {
            Direction::East => new_x+=1,
            Direction::South => new_y+=1,
            Direction::West => new_x-=1,
            Direction::North => new_y-=1,
        };

        let (next_location, next_face, rotation) = self.get_new_face_and_pos(new_x, new_y, map);
        let converted_location = map.get_original_position_from_face_and_offset(next_face, next_location.clone());
        if let Some(tile) =  map.tiles.get(&converted_location) {
            if *tile == '.'{
                let t = map.get_original_position_from_face_and_offset(self.at_face, self.location.clone());
                self._path_taken.push((t, self.facing));

                self.at_face = next_face;
                for _  in 0..rotation {
                    self.change_direction('L')
                }
                return Ok(next_location);
            }else{
                return Err("Wall");
            }
        }


        Err("Wall")
    }


    fn move_forward(&mut self, x: i32, map : &Map) {
        for _i  in 0..x {
            // println!("{:?} {:?} {:?} {}", self.location, self.at_face, self.facing, x-_i);
            let location = if self.using_cube_travel {
                self.get_next_position_if_valid_cube(map)
            }
            else {
                self.get_next_position_if_valid(map)
            };

            if let Ok(location) = location{
                if !self.using_cube_travel {
                    self._path_taken.push((self.location.clone(), self.facing.clone()));
                }
                self.location = location;
            }
            else {
                break;
            }
        }
    }

    pub fn follow_path(&mut self, path: Path, map: &Map) {
        match path{
            Path::Move(x) => self.move_forward(x, map),
            Path::Turn(rotation) => self.change_direction(rotation)
        }
    }

    
    pub fn convert_cube_location(&mut self,  map :&Map) {
        self.location = map.get_original_position_from_face_and_offset(self.at_face, self.location.clone());
    }

    pub fn get_password(&self) -> usize {
        
        return 1000*(self.location.y+1) + 4*(self.location.x+1) + (self.facing as usize as usize)
    }
}