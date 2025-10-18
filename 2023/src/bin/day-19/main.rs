use std::collections::HashMap;
use std::cmp::{min, max};

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

type Workflows = HashMap<String, Vec<Step>>;


static MAX_RATING: i64 = 4000;
static MIN_RATING: i64 = 1;

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64
}

// enum Category {
//     X(i64),
//     M(i64),
//     A(i64),
//     S(i64)
// }

#[derive(Debug, Clone)]
enum Comparator {
    LT,
    GT
}

#[derive(Debug, Clone)]
struct Step {
    category: char,
    comparator: Option<Comparator>,
    condition: i64,
    destination: String
}


fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}


fn get_step_from_str(step_str: &str) -> Step {
    let mut acc = "".to_string();
    let mut comparator = None;
    let mut value:i64 = 0;
    let mut category =  ' '; 
    let destination; 

    for c in step_str.chars() {
        if c == '<' {
            category = acc.chars().next().unwrap();
            acc.clear();
            comparator = Some(Comparator::LT)
        }
        else if c == '>'{
            category = acc.chars().next().unwrap();
            acc.clear();
            comparator = Some(Comparator::GT)
        }
        else if c == ':' {
            value = acc.parse().unwrap();
            acc.clear();
        }
        else {
            acc.push(c);
        }
    }

    destination = acc.to_string();

    
    Step {
        category,
        comparator,
        condition: value,
        destination
    }
}

fn get_parts_and_workflows()  -> (Vec<Part>, Workflows){
    let (workflow_lines, part_lines) = FILE_CONTENTS.split_once("\n\n").unwrap();
    // for 
    // let workflows = vec![];
    let workflows = workflow_lines.split('\n').map(|line| {
        let left_brace = line.find('{').unwrap();
        let label = line[0..left_brace].to_string();
        let steps: Vec<Step> = line[left_brace+1..line.len()-1].split(',').map(|step_str|{
            get_step_from_str(step_str)
        }).collect();
        (label, steps)
        // get_workflow_from_str()
    }).collect();


    let parts = part_lines.split('\n').map(|line| {
        let vals: Vec<&str> = line[1..line.len()-1].split(',').collect();
        Part {
            x: vals[0][2..].parse().unwrap(),
            m: vals[1][2..].parse().unwrap(),
            a: vals[2][2..].parse().unwrap(),
            s: vals[3][2..].parse().unwrap()
        }
    }).collect();

    (parts, workflows)
}


fn apply_workflow(part: &Part, workflows: &Workflows) -> bool {
    let mut outcome = None;
    let mut next_steps = workflows.get("in").unwrap();
    while outcome.is_none() {
        for step in next_steps {
            let val = match step.category {
                'x' => part.x,
                'm' => part.m,
                'a' => part.a,
                's' => part.s,
                ' ' => 0,
                _ => panic!("found other {:?}", step)
            };

            let next_label = match &step.comparator {
                Some(Comparator::LT) => if val < step.condition { Some(step.destination.clone())} else {None},
                Some(Comparator::GT) => if val > step.condition { Some(step.destination.clone())} else {None},
                None => Some(step.destination.clone())
            };

            // println!("{:?}", next_label);

            match next_label.as_deref() {
                Some("A") => outcome = Some(true),
                Some("R") => outcome = Some(false),
                Some(other) => next_steps = workflows.get(other).unwrap(),
                _ => continue
            }

            break;
        }
    }    

    outcome.unwrap()
}


fn part1 () -> i64{
    let (parts, workflows) = get_parts_and_workflows();

    parts.iter().map(|part| if apply_workflow(part, &workflows) {part.x + part.m + part.a + part.s} else {0} ).sum()


}

// struct Step {
//     category: char,
//     comparator: Option<Comparator>,
//     condition: i64,
//     destination: String
// }

fn check_valid_flow(next_flow: &[Step], workflows: &Workflows, mut flow_path: Vec<(Step, bool)>) -> Vec<Vec<(Step, bool)>> {
    let mut valid_flows= vec![];
    

    for next_step in next_flow {
        if next_step.destination == "A" {
            // println!("found A: {:?}", flow_path);
            let mut flow_path_clone = flow_path.clone() ;
            flow_path_clone.push((next_step.clone(), true));
            valid_flows.push(flow_path_clone)
        }
        else if next_step.destination == "R" {
            // return vec![];
        }
        else {
            // check_valid_flow(&next_flow[1..], workflows, flow_path);
            let mut flow_path_clone = flow_path.clone() ;
            flow_path_clone.push((next_step.clone(), true));
            let next_step_met_condition = workflows.get(&next_step.destination).unwrap();
            // println!("extend by {test:?}");
            valid_flows.extend(check_valid_flow(&next_step_met_condition, workflows, flow_path_clone));
        }
        flow_path.push((next_step.clone(), false));
    }
    // println!("list of flows {:?} ",valid_flows);

    return valid_flows

}

// fn get_valid_workflows(workflows: &Workflows) {
//     // let valid = vec![];
    



// }

fn get_possible_combinations(valid_flow: &Vec<(Step, bool)>) -> i64{
    // println!("flow {:?}", valid_flow);

    let mut min_part = Part {x:MIN_RATING, m:MIN_RATING, a:MIN_RATING, s: MIN_RATING};
    let mut max_part=  Part {x:MAX_RATING, m:MAX_RATING, a:MAX_RATING, s: MAX_RATING};
    for (step, met_condition) in valid_flow {
        if *met_condition {
            match step.comparator {
                Some(Comparator::LT) => match step.category {
                        'x' => max_part.x = min(max_part.x, step.condition - 1),
                        'm' => max_part.m = min(max_part.m, step.condition - 1),
                        'a' => max_part.a = min(max_part.a, step.condition - 1),
                        's' => max_part.s = min(max_part.s, step.condition - 1),
                        ' ' => (),
                        _ => panic!("found other {:?}", step)
                },
                Some(Comparator::GT) => match step.category {
                        'x' => min_part.x = max(min_part.x, step.condition + 1),
                        'm' => min_part.m = max(min_part.m, step.condition + 1),
                        'a' => min_part.a = max(min_part.a, step.condition + 1),
                        's' => min_part.s = max(min_part.s, step.condition + 1),
                        ' ' => (),
                        _ => panic!("found other {:?}", step)
                },
                None => ()
            }
        }
        else {
            match step.comparator {
                Some(Comparator::LT) => match step.category {
                        'x' => min_part.x = max(min_part.x, step.condition),
                        'm' => min_part.m = max(min_part.m, step.condition),
                        'a' => min_part.a = max(min_part.a, step.condition),
                        's' => min_part.s = max(min_part.s, step.condition),
                        ' ' => (),
                        _ => panic!("found other {:?}", step)
                },
                Some(Comparator::GT) => match step.category {
                        'x' => max_part.x = min(max_part.x, step.condition),
                        'm' => max_part.m = min(max_part.m, step.condition),
                        'a' => max_part.a = min(max_part.a, step.condition),
                        's' => max_part.s = min(max_part.s, step.condition),
                        ' ' => (),
                        _ => panic!("found other {:?}", step)
                },
                None => ()
            }
        }
        
    }

    // println!("{:?} : {:?}", min_part, max_part);


    return (1+ max_part.x - min_part.x) * (1+ max_part.m - min_part.m) * (1+ max_part.a - min_part.a) * (1+ max_part.s - min_part.s)

    
}

fn part2 () -> i64 {
    let (_parts, workflows) = get_parts_and_workflows();
    let flow = workflows.get("in").unwrap();
    let valid_flows= check_valid_flow(&flow, &workflows, vec![]);
    // for flow in valid_flows.iter() {
        // println!("{:?}", flow);
    // }
    
    valid_flows.iter().map(|flow| get_possible_combinations(flow)).sum()

}
