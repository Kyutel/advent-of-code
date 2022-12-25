use std::collections::HashMap;

static FILE_CONTENTS: &str = include_str!("input.txt");

#[derive(Debug)]
struct Directory {
    // parent: Option<Directory>,
    files: Vec<File>,
    children: Vec<String>,
    _name: String,
    size: i32,
}

#[derive(Debug)]
struct File {
    _name: String,
    size: i32
}

impl Directory {
    fn new(name: &str) -> Directory {
        Directory { files: vec![], children: vec![], _name:String::from(name), size:-1}
    }

    fn get_size(&self, directories: &HashMap<String, Directory>) -> i32 {
        if self.size == -1{
            let mut total = 0;
            for file in self.files.iter() {
                total+= file.size;
            }
            for dir_name in self.children.iter() {
                let dir = directories.get(dir_name).unwrap();
                total+= dir.get_size(directories)
            }
            
            return total
        }
        return self.size;
    }

}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn part1() -> i32 {
    let terminal_output: Vec<&str>= FILE_CONTENTS.split("\n").collect();
    let mut current_dir = "/".to_string();
    
    let mut previous_dirs = Vec::new();
    
    let mut directories: HashMap<String, Directory> = HashMap::new();
    
    let root_dir = Directory::new("/");
    directories.insert(current_dir.clone(), root_dir);
    
    
    let mut last_command = "";
    for line in terminal_output {
        // println!("{:?}:{:?} ", current_dir, line);
        let split_line:Vec<&str> = line.split(' ').collect();
        if split_line[0] == "$"{
            match split_line[1] {
                "cd" => {
                    if split_line[2] == "/" {
                        continue;
                    }
                    else if  split_line[2] == ".."{
                        current_dir = previous_dirs.pop().unwrap();
                    }
                    else {

                        previous_dirs.push(current_dir.clone());
                        current_dir = current_dir + "/"+ split_line[2];

                        if directories.get(&current_dir).is_none(){
                            directories.insert(current_dir.clone(), Directory::new(split_line[2]));
                            
                        }
                        else {
                            print!("THIS SHOULD NEVER OCCUR, {:?}", current_dir);
                        }
                    }
                }
                "ls" => {

                }
                _ => panic!()
            }
            last_command = split_line[1]
        }
        else if last_command == "ls" {
            let dir = directories.get_mut(&current_dir.to_string()).unwrap();
            if split_line[0] == "dir" {
                let dir_name = current_dir.clone() + "/" + split_line[1];
                dir.children.push(dir_name.to_string());
            }
            else {
                dir.files.push(File { _name: split_line[1].to_string(), size: split_line[0].parse::<i32>().unwrap() })
            }
        }
    }

    let mut answer = 0;
    for (_name,dir) in directories.iter() {
        let size = dir.get_size(&directories);
        if size <= 100000 {
            answer += size
        }
        // println!("{:?} {:?}",name, size);
    }

    answer
}

fn part2() -> i32 {
    let terminal_output: Vec<&str>= FILE_CONTENTS.split("\n").collect();
    let mut current_dir = "/".to_string();
    
    let mut previous_dirs = Vec::new();
    
    let mut directories: HashMap<String, Directory> = HashMap::new();
    
    let root_dir = Directory::new("/");
    directories.insert(current_dir.clone(), root_dir);
    
    
    let mut last_command = "";
    for line in terminal_output {
        // println!("{:?}:{:?} ", current_dir, line);
        let split_line:Vec<&str> = line.split(' ').collect();
        if split_line[0] == "$"{
            match split_line[1] {
                "cd" => {
                    if split_line[2] == "/" {
                        continue;
                    }
                    else if  split_line[2] == ".."{
                        current_dir = previous_dirs.pop().unwrap();
                    }
                    else {

                        previous_dirs.push(current_dir.clone());
                        current_dir = current_dir + "/"+ split_line[2];

                        if directories.get(&current_dir).is_none(){
                            directories.insert(current_dir.clone(), Directory::new(split_line[2]));
                            
                        }
                        else {
                            print!("THIS SHOULD NEVER OCCUR, {:?}", current_dir);
                        }
                    }
                }
                "ls" => {

                }
                _ => panic!()
            }
            last_command = split_line[1]
        }
        else if last_command == "ls" {
            let dir = directories.get_mut(&current_dir.to_string()).unwrap();
            if split_line[0] == "dir" {
                let dir_name = current_dir.clone() + "/" + split_line[1];
                dir.children.push(dir_name.to_string());
            }
            else {
                dir.files.push(File { _name: split_line[1].to_string(), size: split_line[0].parse::<i32>().unwrap() })
            }
        }
    }

    let total_size = 70000000;
    let unused_space = total_size - directories.get("/").unwrap().get_size(&directories);
    println!("Unused Space {:?}", unused_space);
    
    let mut valid_sizes_to_delete = vec![];

    for (_name,dir) in directories.iter() {
        let size = dir.get_size(&directories);
        if unused_space+size >= 30000000 {
            valid_sizes_to_delete.push(size);
        }
        // println!("{:?} {:?}",name, size);
    }

    return *valid_sizes_to_delete.iter().min().unwrap();

}