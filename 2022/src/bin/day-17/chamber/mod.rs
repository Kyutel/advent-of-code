pub mod rock;

use rock::Rock;

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

#[derive(Debug)]
pub struct Chamber {
    pub items: Vec<Vec<char>>,
    pub tallest_point: usize
}

impl Chamber {
    pub fn new() -> Chamber{
        let items = vec![vec!['.';7]];
        Chamber { items, tallest_point: 0 }
    }

    pub fn drop_rock(&mut self, rock: &mut Rock, jet_pattern: &Vec<char>, next_jet: usize) -> usize {
        let mut next_jet = next_jet;
        let mut rock_stopped = false;
        while !rock_stopped {
            let jet = jet_pattern[next_jet];
            rock.push_rock_based_on_jet(jet, &self);
            rock_stopped = rock.fall(&self);
            next_jet = (next_jet + 1) % jet_pattern.len();
        }
        self.add_fallen_rock(&rock);
        
        next_jet
    }

    fn add_fallen_rock(&mut self, rock: &Rock) {
        // for _ in 0..rock.get_height() {
        //     self.items.push(vec!['.';7]);
        // }

        for tile in rock.solid_tiles.iter() {
            while self.items.get(tile.y).is_none() {
                self.items.push(vec!['.';7]);
            }
            self.items[tile.y][tile.x] = '#'
        }

        self.tallest_point = self.items.len();
    }

}

impl std::fmt::Display for Chamber{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut chamber_string: String = String::new();
        for row in self.items.iter().rev() {
            chamber_string.push('|');
            for item in row {
                chamber_string.push(*item);
            }
            chamber_string.push('|');
            chamber_string.push('\n');
        }
        chamber_string+= "+-------+";

        write!(f, "{}", chamber_string)
    }
}