use std::collections::{HashMap, VecDeque, HashSet};

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
// static FILE_CONTENTS: &str = include_str!("test-input2.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
    // Button
}

#[derive(Debug)]
enum State {
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
    NoState
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High
}

#[derive(Debug)]
struct Module {
    label: String,
    m_type: ModuleType,
    state: State,
    targets: Vec<String>,
    sources: Vec<String>,
    p_for_low: Option<i32>,
    p_for_high: Option<i32>
}


fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_module_from_line(line: &str) -> Module {
    let (source, targets) = line.split_once(" -> ").unwrap();
    let m_type = match source.chars().next().unwrap() {
        '%' => ModuleType::FlipFlop,
        '&' => ModuleType::Conjunction,
        'b' => ModuleType::Broadcast,
        _ => panic!()
    };

    let label;
    let state;
    match m_type  {
        ModuleType::FlipFlop => {label = source[1..].to_string(); state = State::FlipFlop(false)},
        ModuleType::Conjunction => {label = source[1..].to_string(); state = State::Conjunction(HashMap::new())},
        ModuleType::Broadcast => {label = source.to_string(); state = State::NoState}
        // _ => {}
    }


    let targets = targets.split(", ").map(|target| target.to_string()).collect();
    let sources = vec![];
    let p_for_low = None;
    let p_for_high = None;

    Module { label, m_type, state, targets, sources, p_for_low, p_for_high}

}

fn fill_conjuctions_and_sources(modules: &mut HashMap<String, Module>) {

    let mut reverse_relations: HashMap<String, Vec<String>> = HashMap::new();

    for (label, module) in modules.iter() {
        for target in module.targets.iter(){
            if let Some(target_module) = modules.get(target) {
                if let Some(list) = reverse_relations.get_mut(&target_module.label) {
                    list.push(label.clone());
                }
                else {
                    reverse_relations.insert(target_module.label.clone(), vec![label.clone()]);
                }
            }
        }
    }

    for (label, relations) in reverse_relations {
        let module = modules.get_mut(&label).unwrap();
        if let State::Conjunction(state) = &mut module.state {
            for relation in relations.iter() {
                state.insert(relation.clone(), Pulse::Low);
            }
        }
        module.sources = relations;
    }

}

fn get_modules() -> HashMap<String, Module>{
    let mut modules = FILE_CONTENTS.split('\n').map(|line| {let module = get_module_from_line(line); (module.label.clone(), module) }).collect();


    fill_conjuctions_and_sources(&mut modules);
    modules
}


// fn take_action(module, )

fn take_action_for_flip_flop(module: &mut Module, pulse:Pulse, actions: &mut VecDeque<(String, String, Pulse)>) -> Option<Pulse> {
    if pulse == Pulse::Low {
        match module.state {
            State::FlipFlop(true) => {
                module.state = State::FlipFlop(false);
                for target in module.targets.iter() {
                    (*actions).push_back((module.label.clone(), target.clone(), Pulse::Low));
                }
                return Some(Pulse::Low)
            },
            State::FlipFlop(false) => {
                module.state = State::FlipFlop(true);
                for target in module.targets.iter() {
                    (*actions).push_back((module.label.clone(), target.clone(), Pulse::High));
                }
                return Some(Pulse::High)
            }   
            _ => panic!()
        }                        
    }
    return None
}

fn take_action_for_conjuction(module: &mut Module, sender: String, pulse:Pulse, actions: &mut VecDeque<(String, String, Pulse)>) -> Option<Pulse>{
    
    if let State::Conjunction(state) = &mut module.state {
        let pulse_to_send;
        state.insert(sender, pulse);

        if state.values().any(|val| *val == Pulse::Low) {
            pulse_to_send = Pulse::High;
        }
        else {
            pulse_to_send = Pulse::Low;
        }

        for target in module.targets.iter() {
            actions.push_back((module.label.clone(), target.clone(), pulse_to_send));
        }
        return Some(pulse_to_send)
    }

    return  None;
}


fn push_button(modules: &mut HashMap<String, Module>) -> ( i32, i32) {
    let mut actions: VecDeque<(String, String, Pulse)> = VecDeque::new();
    actions.push_back(("button".to_string(),"broadcaster".to_string(), Pulse::Low));

    let mut low_count = 0;
    let mut high_count = 0;

    while let Some((sender, next_module_label, pulse)) = actions.pop_front() {
        // println!("{actions:?}");
        match pulse {
            Pulse::Low => low_count +=1,
            Pulse::High => high_count +=1
        }

        // let next_action = actions.pop_front();
        if let Some(next_module) = modules.get_mut(&next_module_label) {
            match next_module.m_type {
                ModuleType::FlipFlop => {
                    take_action_for_flip_flop(next_module, pulse, &mut actions);                        
                },
                ModuleType::Conjunction => {
                    take_action_for_conjuction(next_module,sender, pulse, &mut actions);                        
                },
                ModuleType::Broadcast => {
                    for target in next_module.targets.iter() {
                        actions.push_back((next_module_label.clone(),target.clone(), pulse));
                    }
                }
            }
        }
        
    }
    (low_count, high_count)
}


fn _print_modules(modules: &HashMap<String, Module>) {
    for module in modules.iter() {
        println!("{:?}", module);
    }
}

fn part1 () -> i32 {
    let mut modules = get_modules();
    
    let mut lc_sum = 0;
    let mut hc_sum = 0;
    // _print_modules(&modules);
    // let mut new_modules = modules;
    for _ in 0..1000 {
        let (lc, hc) = push_button(&mut modules);
        lc_sum += lc;
        hc_sum += hc;
        // println!("LC {:?}, HC {:?}", lc, hc);
    }

    return lc_sum * hc_sum
}





fn push_button_and_get_cycles(modules: &mut HashMap<String, Module>, last_sources: Vec<String>) -> Vec<i64> {
    // find main multiplex
    


    let mut cycles = vec![0,0,0,0];
    let mut cycle_count = 0;
    while cycles.contains(&0) {
        cycle_count += 1;

        let mut actions: VecDeque<(String, String, Pulse)> = VecDeque::new();
        actions.push_back(("button".to_string(),"broadcaster".to_string(), Pulse::Low));
        while let Some((sender, next_module_label, pulse)) = actions.pop_front() {
            if let Some(next_module) = modules.get_mut(&next_module_label) {
                let pulse_sent;
                match next_module.m_type {
                    ModuleType::FlipFlop => {
                        pulse_sent = take_action_for_flip_flop(next_module, pulse, &mut actions);                        
                    },
                    ModuleType::Conjunction => {
                        pulse_sent = take_action_for_conjuction(next_module,sender, pulse, &mut actions);                        
                    },
                    ModuleType::Broadcast => {
                        for target in next_module.targets.iter() {
                            actions.push_back((next_module_label.clone(),target.clone(), pulse));
                        }
                        pulse_sent = Some(pulse);
                    }
                }

                if let Some(source_i) = last_sources.iter().position(|x| x== &next_module_label ) {

                    match pulse_sent {
                        Some(Pulse::High) => {
                            if cycles[source_i] == 0 {
                                cycles[source_i] = cycle_count;
                            }
                            else {

                            }
                        },
                        _ => ()
                    }
                }
            }
        }
    }

    
    
    cycles
}


fn part2 () -> i64 {
    let mut modules = get_modules();
    
    // making a bunch of assumptions here

    // _print_modules(&modules);

    let mut rx_sender = None;
    let sources;
    for (_label, module) in modules.iter() {
        if module.targets.contains(&"rx".to_string()) {
            rx_sender = Some(module);
            break;
        }
    }


    if let Some(rx_sender) = rx_sender {
        sources =  rx_sender.sources.clone();
        
    }
    else {
        return 0
    }



    let cycles = push_button_and_get_cycles(&mut modules, sources);


    let button_presses = lcmm(cycles);
  
    return  button_presses;

    
 
}

fn gcd(a: i64 , b: i64) -> i64{
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

fn lcm(a: i64, b: i64) -> i64 {
    return a * b / gcd(a, b)
}

fn lcmm(numbers: Vec<i64>)  -> i64{
    let result = numbers.into_iter().reduce(|a,b| lcm(a, b)).unwrap();
    return result
}