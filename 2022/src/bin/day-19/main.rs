use core::fmt;
use std::collections::HashMap;

use regex::Regex;
// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

#[derive(Debug, Clone, Copy)]
enum Material {
    Ore(i32),
    Clay(i32),
    Obsidian(i32),
    Geode(i32)
}

impl Material {
    // fn get_index(&self)  -> i32 {
    //     match self {
    //         Material::Ore(_) =>  0,
    //         Material::Clay(_) =>  1,
    //         Material::Obsidian(_) => 2,
    //         Material::Geode(_) =>  3
    //     }
    // }

    fn update_val(&mut self, new_val: i32)  {
        
        match self {
            Material::Ore(x) => *x = *x + new_val,
            Material::Clay(x) => *x = *x + new_val,
            Material::Obsidian(x) =>*x = *x + new_val,
            Material::Geode(x) => *x = *x + new_val
        }
    }

    fn get_val(&self) -> i32 {
        match self {
            Material::Ore(x) => *x,
            Material::Clay(x) => *x,
            Material::Obsidian(x) => *x,
            Material::Geode(x) => *x
        }
    }
}


#[derive(Debug, Clone, Copy)]
struct Robot {
    material_type: Material,
    count: i32,
    cost: [Material;3]
}


#[derive(Debug, Clone, Copy)]
struct  Factory {
    id: i32,
    robots: [Robot;4],
    resource_count: [Material;4],
    create_request: [i32;4],
    max_costs:[i32;3]
}

impl fmt::Display for Factory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::new();
        string += "Robots [";
        for robot in self.robots {
            string += &robot.count.to_string();
            string += ","
        }
        string += "] ";

        string += "Resources [";
        for resource in self.resource_count {
            string  += &resource.get_val().to_string();
            string += ",";
        }
        string += "] ";

        string += "Requests [";
        for req in self.create_request {
            string  += &req.to_string();
            string += ",";
        }
        string += "]";
        
        write!(f, "{}", string)
    }
}

impl Factory {
    fn mine_resources(&mut self) {
        for (index, robot) in self.robots.iter().enumerate() {
            self.resource_count[index].update_val(robot.count);
        }
    }

    fn get_max_costs(robots: [Robot;4]) -> [i32;3]{

        let mut max_costs = [0,0,0];
        for robot in robots{
            for (index, material) in robot.cost.iter().enumerate() {
                if max_costs[index] < material.get_val(){
                    max_costs[index] = material.get_val();
                }
            }
        }
        
        max_costs
    }

    fn get_possible_factories(&self, _time: i32) -> Vec<Factory>{
        let mut futures = vec![];


        for (r_index, robot) in self.robots.iter().enumerate().rev() {

            

            // if r_index < 3 && self.resource_count[r_index].get_val() >= self.max_costs[r_index] * time{
            //     continue;
            // }

            if  r_index < 3 && robot.count >= self.max_costs[r_index] {
                continue;
            }

            let mut can_create = true;
            for (m_index,cost) in robot.cost.iter().enumerate() {
                
                if self.resource_count[m_index].get_val() < cost.get_val() {
                    can_create = false;
                }
            }
            if can_create {
                let mut new_factory = self.clone();

                new_factory.create_request[r_index] += 1;
                for (m_index,cost) in robot.cost.iter().enumerate() {
                    new_factory.resource_count[m_index].update_val(-cost.get_val());
                }
                
                futures.push(new_factory);
                if let Material::Geode(_) = robot.material_type {
                    return futures;
                }
            }
        }

        // let mut too_many = true;
        // for x in self.resource_count[0..2].iter().enumerate() {
        //     if x.1.get_val() <= self.max_costs[x.0] {
        //         too_many = false;
        //     }
        // }

        // if too_many {
        //     return futures
        // }
        futures.push(self.clone());

        futures
    }

    fn process_create_request(&mut self) {
        for (i, req) in self.create_request.iter().enumerate() {
            self.robots[i].count += req;
            
        }
        self.create_request = [0;4];
    }

    // fn print_robot_counts(&self) {
    //     for robot in self.robots {
    //         println!("Type:{:?} Count:{}", robot.material_type, robot.count)
    //     }
    // }

    fn get_robot_count(&self)  -> [i32;4]{
        let mut robot_count = [0;4]; 
        for (i, robot) in self.robots.iter().enumerate() {
            robot_count[i] = robot.count;
        }
        robot_count
    }

}

fn parse_factory_from_line (input_line: &str, regex: &Regex) -> Factory {
    let matches: Vec<i32> = regex.find_iter(input_line)
    .map(|x| x.as_str().parse().unwrap())
    .collect();

    let id = matches[0];
    let cost = [Material::Ore(matches[1]),Material::Clay(0), Material::Obsidian(0)];
    let ore_robot =Robot { material_type: Material::Ore(0), count:1, cost};

    let cost = [Material::Ore(matches[2]),Material::Clay(0), Material::Obsidian(0)];
    let clay_robot =Robot { material_type: Material::Clay(0), count:0, cost};
    
    let cost = [Material::Ore(matches[3]),Material::Clay(matches[4]), Material::Obsidian(0)];
    let obsidian_robot =Robot { material_type: Material::Obsidian(0), count:0, cost};
    
    let cost = [Material::Ore(matches[5]),Material::Clay(0), Material::Obsidian(matches[6])];
    let geode_robot =Robot { material_type: Material::Geode(0), count:0, cost};
    
    let robots = [ore_robot, clay_robot, obsidian_robot, geode_robot];
    let resource_count =  [Material::Ore(0),Material::Clay(0), Material::Obsidian(0), Material::Geode(0)];

    return Factory { id, robots, resource_count, create_request: [0;4], max_costs:Factory::get_max_costs(robots)}
}

fn get_factories() -> Vec<Factory> {
    let re = Regex::new(r"\d+").unwrap();
    FILE_CONTENTS.split('\n').map(|x| parse_factory_from_line(x, &re)).collect()
}

fn get_best_geode_count(factory: &mut Factory, time: i32, history :&mut HashMap<(i32, [i32;4]), [Material;4]>) -> i32 {


    if time == 0 {
        return factory.resource_count[3].get_val()
    }

    factory.process_create_request();

    let rc = factory.get_robot_count();
    if let Some(historic_resource) = history.get(&(time, rc)) {
        let mut end_timeline = true;
        for (i, resource) in historic_resource.iter().enumerate() {
            if factory.resource_count[i].get_val() > resource.get_val() {
                end_timeline = false;
            }
        }
        if end_timeline {
            return 0;
        }
    }
        
    history.insert((time, rc), factory.resource_count);

    let future_factories = factory.get_possible_factories(time);

    let mut best = factory.resource_count[3].get_val();
    for mut factory in future_factories {
        factory.mine_resources();

        let geode_count = get_best_geode_count(&mut factory, time -1, history);
        if geode_count > best {
            best = geode_count
        }
    }

    best
    

}

fn part1 () -> i32 {
    let mut factories = get_factories();


    // let mut temp_factory = factories[1].clone();
    let time = 24;
    let mut total_quality = 0;
    for factory in factories.iter_mut() {
        let mut history: HashMap<(i32, [i32;4]), [Material;4]> = HashMap::new();
        let quality_level = get_best_geode_count(factory, time,&mut history) * factory.id;
        // println!("{}", quality_level);
        total_quality += quality_level
    }

    total_quality
}
fn part2 ()  -> i32{
    let factories = get_factories();

    let mut remaining_factory: Vec<Factory> = factories[0..3].to_vec();

    let time = 32;

    let mut multi = 1;
    for factory in remaining_factory.iter_mut() {
        let mut history: HashMap<(i32, [i32;4]), [Material;4]> = HashMap::new();

        let most_geodes = get_best_geode_count(factory, time, &mut history);
        // println!("{}", most_geodes);
        multi *= most_geodes
        // total_quality += quality_level
    }
    multi
}
