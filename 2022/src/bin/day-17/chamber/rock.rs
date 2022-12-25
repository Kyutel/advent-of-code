use super::{Point, Chamber};


pub struct Rock {
    pub solid_tiles: Vec<Point>,
    pub shape: RockShape
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum RockShape {
    Flat,
    Plus,
    LShape,
    Vertical,
    Square
}

impl Rock {
    pub fn get_next_rock(previous_rock: Option<Rock>, tallest_point: usize)  -> Rock{
        

        let shape = if let Some(x) = previous_rock {
            match x.shape {
                RockShape::Flat => RockShape::Plus,
                RockShape::Plus => RockShape::LShape,
                RockShape::LShape => RockShape::Vertical,
                RockShape::Vertical => RockShape::Square,
                RockShape::Square => RockShape::Flat,
            }
        }
        else {
            RockShape::Flat
        };
        

        Rock::create_rock_from_shape(shape, tallest_point)
    }

    pub fn create_rock_from_shape(shape : RockShape, tallest_point: usize) -> Rock {
        let mut solid_tiles = vec![];
        match shape {
            RockShape::Flat => {
                for x in 2..6 {
                    solid_tiles.push(Point {x, y: tallest_point + 3})
                }
            }
            RockShape::Plus => {
                solid_tiles.push(Point {x:3, y: tallest_point + 5});
                solid_tiles.push(Point {x:2, y: tallest_point + 4});
                solid_tiles.push(Point {x:3, y: tallest_point + 4});
                solid_tiles.push(Point {x:4, y: tallest_point + 4});
                solid_tiles.push(Point {x:3, y: tallest_point + 3});
            }
            RockShape::LShape => {
                solid_tiles.push(Point {x:4, y: tallest_point + 5});
                solid_tiles.push(Point {x:2, y: tallest_point + 3});
                solid_tiles.push(Point {x:3, y: tallest_point + 3});
                solid_tiles.push(Point {x:4, y: tallest_point + 3});
                solid_tiles.push(Point {x:4, y: tallest_point + 4});

            }
            RockShape::Vertical => {
                for y in (tallest_point+3..tallest_point+7).rev() {
                    solid_tiles.push(Point {x:2, y})
                }
            },
            RockShape::Square => {
                solid_tiles.push(Point {x:2, y: tallest_point + 4});
                solid_tiles.push(Point {x:2, y: tallest_point + 3});
                solid_tiles.push(Point {x:3, y: tallest_point + 3});
                solid_tiles.push(Point {x:3, y: tallest_point + 4});
                
            }
        }

        Rock {shape, solid_tiles}
    }

    pub fn push_rock_based_on_jet(&mut self, jet: char, chamber: &Chamber) {
        let mut can_move = true;
        let direction: i32 = if jet == '>' {1} else {-1};

        for tile in self.solid_tiles.iter() {
            let new_x = tile.x as i32 + direction;
            let item_at_new_x = *chamber.items.get(tile.y).unwrap_or(&vec!['.']).get(new_x as usize).unwrap_or(&'.');

            if new_x < 0 || new_x > 6||item_at_new_x != '.' {
                can_move = false;
                break;
            }
        }

        if can_move {
            for tile in self.solid_tiles.iter_mut() {
                let new_x = tile.x as i32 + direction;
                tile.x = new_x as usize;
            }
        }
    }

    pub fn fall(&mut self, chamber: &Chamber) -> bool {
        let mut can_move = true;

        for tile in self.solid_tiles.iter() {
            let new_y = tile.y as i32 -1;
            let item_at_new_y = *chamber.items.get(new_y as usize).unwrap_or(&vec!['.']).get(tile.x).unwrap_or(&'.');

            if new_y < 0 || item_at_new_y != '.' {
                can_move = false;
                break;
            }
        }

        if can_move {
            for tile in self.solid_tiles.iter_mut() {
                let new_y = tile.y as i32 -1;
                tile.y = new_y as usize;
            }
        }

        !can_move
    }

}