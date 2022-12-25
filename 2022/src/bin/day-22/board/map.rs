use std::collections::HashMap;

use super::{Point, Person, Direction};
use crate::FACE_SIZE;

#[derive(Debug)]
pub struct Face {
    id: usize,
    start_point: Point,
    pub links: [Option<FaceLink>;4]
}

#[derive(Debug, Clone, Copy)]
pub struct FaceLink {
    pub linked_to: usize,
    pub rotation: usize
}

#[derive(Debug)]
pub struct Map {
    pub tiles: HashMap<Point, char>,
    pub biggest_point: Point,
    pub person_start: Point,
    pub faces: Vec<Face>,
}


impl Map {

    fn get_face_starts(tiles: &HashMap<Point, char>, biggest_point: &Point)  -> Vec<Face> {
        let mut faces = vec![];
        for y in (0..biggest_point.y).step_by(FACE_SIZE) {
            for x in (0..biggest_point.x).step_by(FACE_SIZE) {
                let current_point = Point { x, y };
                if tiles.get(&current_point).is_some() {
                    let links = [None;4];

                    faces.push(Face { id: faces.len(), start_point: current_point, links })
                }
            }
        }

        faces
    }

    pub fn get_original_position_from_face_and_offset(&self, face_id: usize, offset: Point ) -> Point {
        let face_origin = &self.faces[face_id].start_point;

        return Point { x: face_origin.x + offset.x , y:  face_origin.y + offset.y}
    }

    pub fn new(input: &str) -> Map {
        let tiles: HashMap<Point, char> = input.split('\n').enumerate()
        .flat_map(
            |(y, row)| row.chars().enumerate()
                .filter(|(_, tile)| *tile != ' ')
                .map(move |(x, tile)| (Point{x, y}, tile))
        )
        .collect();

        let mut biggest_point = Point { x: 0, y: 0 };
        let mut person_start = Point { x: usize::MAX, y: 0 };
        for point in tiles.keys() {
            if point.y == 0 {
                person_start = Point { x: person_start.x.min(point.x), y: 0 };
            }

            biggest_point = Point{x: biggest_point.x.max(point.x), y: biggest_point.y.max(point.y)};
        }

        let faces = Self::get_face_starts(&tiles, &biggest_point);

        Map { tiles, biggest_point, person_start, faces }
    }

    pub fn connect_faces(&mut self) {
        let mut link_count = 0;
        for id in 0..self.faces.len(){
            link_count += 2 * self.find_direct_connections(id);
        }


        while link_count < 24 {
            for id in 0..self.faces.len(){
                link_count += self.find_indirect_connections(id);
                // for face in map.faces {
                //     println!("{:?}", face.links);
                // }
                // println!("{:?}", link_count);
                
            }
        }
    }

    fn get_face_id_from_start_point(&self, start_point: Point) -> usize{
        for faces in self.faces.iter() {
            if start_point == faces.start_point {
                return faces.id;
            }
        }

        10
    }

    fn set_link(&mut self,face_id: usize, link_point: Point, link_dir: usize, rotation: usize) {
        // let rotation = 0;
        let inverse_link_dir = (link_dir + 2) %4 ;
        let linked_to = self.get_face_id_from_start_point(link_point);

        self.faces[face_id].links[link_dir] = Some(FaceLink {linked_to, rotation});
        self.faces[linked_to].links[inverse_link_dir] = Some(FaceLink {linked_to:face_id, rotation});
    }

    fn find_direct_connections(&mut self, face_id: usize) -> i32 {
        let mut link_count = 0;

        let face =  &self.faces[face_id];
        let east_link = Point {x:face.start_point.x + FACE_SIZE, y: face.start_point.y};
        if face.links[0].is_none() && self.tiles.contains_key(&east_link) {
            self.set_link(face_id, east_link, 0, 0);
            link_count +=1;
        }

        let face =  &self.faces[face_id];
        let south_link = Point {x:face.start_point.x, y: face.start_point.y + FACE_SIZE};
        if face.links[1].is_none() && self.tiles.contains_key(&south_link) {
            self.set_link(face_id, south_link, 1, 0);
            link_count +=1;
        }

        let face = &self.faces[face_id];
        if face.links[2].is_none() && face.start_point.x != 0{
            let west_link = Point {x:face.start_point.x - FACE_SIZE, y: face.start_point.y};
            if self.tiles.contains_key(&west_link) {
                self.set_link(face_id, west_link, 2, 0);
                link_count +=1;
            }
        }

        let face = &self.faces[face_id];
        if face.links[3].is_none() && face.start_point.y != 0{
            let north_link = Point {x:face.start_point.x, y: face.start_point.y - FACE_SIZE};
            if self.tiles.contains_key(&north_link) {
                self.set_link(face_id, north_link, 3, 0);
                link_count +=1;
            }
        }

        link_count
    }

    fn find_indirect_connections(&mut self, face_id: usize) -> i32 {
        let mut link_count = 0; //MAKE SURE TO CHANGE THIS TO ZERO

        for i in 0..4{
                let link = &mut self.faces[face_id].links[i];
                if link.is_none() {
                    if i == 0 {
                        let south = self.faces[face_id].links[1];
                        let north = self.faces[face_id].links[3];    
                        if self.set_secondary_link_ew(face_id, south, 3, false) 
                         || self.set_secondary_link_ew(face_id, north, 1, false){
                            link_count +=1;
                        }
                    }
                    else if i == 2 {
                        let south = self.faces[face_id].links[1];
                        let north = self.faces[face_id].links[3];    
                        if self.set_secondary_link_ew(face_id, south, 1, true) 
                         || self.set_secondary_link_ew(face_id, north, 3, true){
                            link_count +=1;
                        }
                    }
                    else if i == 1 {
                        let east = self.faces[face_id].links[0];
                        let west = self.faces[face_id].links[2]; 
                        if self.set_secondary_link_ns(face_id, east, 1, false) 
                         || self.set_secondary_link_ns(face_id, west, 3, false){
                            link_count +=1;
                        }
                    }
                    else if i == 3 {
                        let east = self.faces[face_id].links[0];
                        let west = self.faces[face_id].links[2]; 
                        if self.set_secondary_link_ns(face_id, east, 3, true) 
                         || self.set_secondary_link_ns(face_id, west, 1, true){
                            link_count +=1;
                        }
                    }
                }
        }

        link_count
    }

    fn set_secondary_link_ew(&mut self, face_id: usize,first_link: Option<FaceLink>, rotation_change: usize, west: bool) -> bool {
            if let Some(first_link) = first_link {
                let first_link_rotation = first_link.rotation;
                let secondary_link_id = first_link_rotation + (((first_link_rotation + west as usize) % 2) * 2);
                let secondary_link = self.faces[first_link.linked_to].links[secondary_link_id%4];
    
                if let Some(secondary_link) = secondary_link {
                    let rotation = (first_link_rotation+ secondary_link.rotation + rotation_change) % 4;
                    let link = &mut self.faces[face_id].links[0 + (west as usize * 2)];
                    *link = Some(FaceLink {linked_to: secondary_link.linked_to, rotation});
                    return true
                }
            }
        false
    }

    fn set_secondary_link_ns(&mut self, face_id: usize,first_link: Option<FaceLink>, rotation_change: usize, north:bool) -> bool {
        if let Some(first_link) = first_link {
            let first_link_rotation = first_link.rotation;
            let secondary_link_id ;


            if (first_link_rotation + north as usize) % 2 == 0{
                secondary_link_id = first_link_rotation + 1;
            }
            else {
                secondary_link_id = first_link_rotation + 3;
            }
            let secondary_link = self.faces[first_link.linked_to].links[secondary_link_id%4];

            if let Some(secondary_link) = secondary_link {
                let rotation = (first_link_rotation + secondary_link.rotation +  rotation_change) % 4;
                let link = &mut self.faces[face_id].links[1 + (north as usize * 2)];
                *link = Some(FaceLink {linked_to: secondary_link.linked_to, rotation});
                return true
            }
        }
    false
}

    pub fn _print_map(&self, person: Option<&Person>) {
        let mut tile_clone = self.tiles.clone();
        if let Some(p) = person {
            for (pos, dir) in p._path_taken.iter() {
                let dir_char = match dir {
                    Direction::East => '>',
                    Direction::South => 'V',
                    Direction::West => '<',
                    Direction::North => '^',
                };
                tile_clone.insert(pos.clone(), dir_char);
            }
        }

        for y in 0..self.biggest_point.y+1 {
            for x in 0..self.biggest_point.x+1 {
                let current_point = Point { x, y };
                let c = tile_clone.get(&current_point).unwrap_or(&' ');
                print!("{c}");
            }
            println!();
        }


    }
}