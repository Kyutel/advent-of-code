use std::collections::HashMap;

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
struct Operation{
    operator_1: String,
    operator_2: String,
    operator: char
}

#[derive(Debug, Clone)]
enum Job {
    Number(f64),
    Operation(Operation),
    Human(Human)
}

#[derive(Debug, Clone)]
struct Monkey {
    _id: String,
    action: Job
}

#[derive(Debug, Clone)]
struct Human {
    multipliyer:f64,
    addition:f64
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}


impl Human {
    fn update(&self, val: f64, operator: char, left_side: bool) -> Human {
        let mut new_multiplier = self.multipliyer;
        let new_addition;

        if operator == '+' {
            new_addition = self.addition + val;
        }
        else if operator == '-' {
            if left_side {
                new_addition = self.addition - val;
            }
            else {
                new_addition = val - self.addition;
            }
        }
        else if operator == '*' {
            new_multiplier = self.multipliyer * val;
            new_addition = self.addition * val;
        }
        else if operator == '/' {
            if left_side {
                new_multiplier = self.multipliyer / val;
                new_addition = self.addition / val;
            }
            else {
                new_multiplier = val / self.multipliyer;
                new_addition = val / self.addition;
            }
        }
        else {
            panic!()
        }
        // println!("M:{} A:{}", new_multiplier ,new_addition);

        Human { multipliyer: new_multiplier, addition: new_addition }
    }
}


fn parse_monkey_from_line(line: &str) -> (String, Monkey) {

    
    let action;
    let split_job:Vec<&str> = line[6..].split(' ').collect();
    if split_job.len() == 1 {
        action = Job::Number(split_job[0].parse().unwrap());
    }
    else {
        action = Job::Operation(Operation { 
                operator_1: split_job[0].to_string(), 
                operator_2: split_job[2].to_string(), 
                operator:  split_job[1].chars().next().unwrap()
            })
    }
    
    let id = line[..4].to_string();
    let monkey = Monkey{_id:id.clone(), action};
    
    (id, monkey)
}

fn get_monkeys() -> HashMap<String, Monkey> {
    FILE_CONTENTS.split('\n').map(|x| parse_monkey_from_line(x)).collect()
}

fn commit_operation(val_1: f64, val_2: f64, operator: char) -> f64 {
    if operator == '+' {
        val_1 + val_2
    }
    else if operator == '-' {
        val_1 - val_2
    }
    else if operator == '*' {
        val_1 * val_2
    }
    else if operator == '/' {
        val_1 / val_2
    }
    else {
        panic!()
    }
}

fn get_action_value_for_monkey(monkey_id: &String, all_monkeys: &HashMap<String, Monkey>) -> f64{
    let monkey = all_monkeys.get(monkey_id).unwrap();

    match &monkey.action {
        Job::Number(x) => *x,
        Job::Operation(op) => {
            let val_1 = get_action_value_for_monkey(&op.operator_1, all_monkeys);
            let val_2 = get_action_value_for_monkey(&op.operator_2, all_monkeys);
            let result = commit_operation(val_1, val_2, op.operator);
            // all_monkeys.get_mut(monkey_id).unwrap().action = Job::Number(result);
            result
        },
        &Job::Human(_) => 0.0
    }
}

fn part1 () -> f64{
    let monkeys =  get_monkeys();

    let root_monkey_id = "root".to_string();
    
    get_action_value_for_monkey(&root_monkey_id, &monkeys)
}


fn find_contains_human(monkey_id: &String, all_monkeys: &HashMap<String, Monkey>) -> bool{
    let human_id = "humn".to_string();

    let monkey = all_monkeys.get(monkey_id).unwrap();

    if monkey_id == &human_id {
        return true;
    }

    match &monkey.action {
        Job::Number(_) => false,
        Job::Operation(op) =>  {
            let left_id = &op.operator_1;
            let right_id = &op.operator_2;
            // if left_id == &human_id || right_id  == &human_id { return true}; 
            return find_contains_human(left_id, &all_monkeys) || find_contains_human(right_id, &all_monkeys)
        },
        &Job::Human(_) => true
    }
}


fn get_human_equation(monkey_id: &String, all_monkeys: &HashMap<String, Monkey>) -> Job {
    let monkey = all_monkeys.get(monkey_id).unwrap().clone();

    match &monkey.action {
        Job::Operation(op) => {
            let left = get_human_equation(&op.operator_1, all_monkeys);
            let right = get_human_equation(&op.operator_2, all_monkeys);
            match (left, right) {
                (Job::Number(x), Job::Number(y)) =>  Job::Number(commit_operation(x, y, op.operator)),
                (Job::Human(h), Job::Number(y)) => Job::Human(h.update(y, op.operator, true)),
                (Job::Number(x), Job::Human(h)) => Job::Human(h.update(x, op.operator, false)),
                _ => panic!()
            }
            
        },
        _ => monkey.action
    }
}

fn part2 () -> f64{
    let mut monkeys =  get_monkeys();

    let root_monkey_id = "root".to_string();
    get_action_value_for_monkey(&root_monkey_id, &mut monkeys);
    let root_monkey = monkeys.get(&root_monkey_id).unwrap();

    // println!();

    let human_id = "humn".to_string();
    
    let (left, right) = if let Job::Operation(x) = &root_monkey.action {
        (x.operator_1.clone(), x.operator_2.clone())
    }
    else {
        panic!();
    };
    
    monkeys.get_mut(&human_id).unwrap().action = Job::Human(Human { multipliyer: 1.0, addition: 0.0 });

    let human_left = if find_contains_human(&left, &monkeys) {
        true
    }
    else if find_contains_human(&right, &monkeys) {
        false
    }
    else {
        panic!()
    };

    let other_value;
    let eq;
    if human_left {
        other_value = get_action_value_for_monkey(&right, &monkeys);
        eq = get_human_equation(&left, &monkeys);
    }
    else {
        other_value = get_action_value_for_monkey(&left, &monkeys);
        eq = get_human_equation(&right, &monkeys);
    }


    if let Job::Human(h) = eq {
        ((other_value - h.addition) / h.multipliyer).abs()
    }
    else {
        panic!()
    }



}
