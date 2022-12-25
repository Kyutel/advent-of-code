static FILE_CONTENTS: &str = include_str!("input.txt");


#[derive(Debug)]
struct Monkey {
    _id: usize,
    items: Vec<i64>,
    operation: (char, i64),
    test_condition: (i64, usize, usize),
    inspected_count:i64
}

impl Monkey {
    fn from(unparsed_monkey: Vec<&str>) -> Monkey {

        let _id = unparsed_monkey[0][7..unparsed_monkey[0].len()-1].parse::<usize>().unwrap();
        
        let items = unparsed_monkey[1][18..].split(", ").map(|x| x.parse::<i64>().unwrap()).collect();
        
        // println!("{}", &unparsed_monkey[2]);
        let operator = unparsed_monkey[2][23..24].chars().next().unwrap();
        let rhs = match (unparsed_monkey[2][25..]).parse::<i64>(){
            Ok(x) => x,
            Err(_) => if (&unparsed_monkey[2][25..]) == "old" {-1} else {panic!()},
        };

        let operation = (operator, rhs);


        let test_condition = (
            unparsed_monkey[3][21..].parse::<i64>().unwrap(),
            unparsed_monkey[4][29..].parse::<usize>().unwrap(),
            unparsed_monkey[5][30..].parse::<usize>().unwrap()
        );

        Monkey {_id, items, operation, test_condition, inspected_count:0}
    }

    fn inspect_items(&mut self, reduce_worry: bool) {
        for item in &mut self.items{
            self.inspected_count += 1;
            let rhs =  if self.operation.1 >= 0 {self.operation.1} else {*item};
            if self.operation.0 == '+' {
                *item+= rhs;
            }
            else if self.operation.0 == '*' {
                *item = *item * rhs;
            }
            else {panic!()}
            
            if reduce_worry {
                *item/= 3;
            }
        }
    }
    
    fn get_items_to_throw(&mut self) -> Vec<(i64, usize)>{
        let mut items_to_throw = vec![];
        
        while self.items.len() > 0{
            let next_item = self.items.pop().unwrap();
            let throw_to_monkey = if next_item % self.test_condition.0 == 0{
                self.test_condition.1
            }else {
                self.test_condition.2
            };

            items_to_throw.push((next_item, throw_to_monkey));
        }

        items_to_throw
    }
}



fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_monkeys () -> Vec<Monkey>{
    FILE_CONTENTS.split("\n\n")
    .map(|x| Monkey::from(x.split('\n').collect())).collect()
}

fn _print_monkeys(monkeys: &Vec<Monkey>) {
    for monkey in monkeys {
        println!("{:?}", monkey);
    }
}

fn calculate_lcm(monkeys: &Vec<Monkey>) -> i64 {
    let values: Vec<i64> = monkeys.iter().map(|x| x.test_condition.0).collect();

    let mut lcm = values[0];
    let origin = lcm;

    let mut found = false;
    while !found {
        found = true;
        lcm+=origin;
        for val in &values{
            if lcm % val != 0 {
                found = false
            }
        }
        // println!("{}", lcm);
    }

    lcm
}

fn part1() -> i64 {
    let mut monkeys = get_monkeys();
    let rounds = 20;
    for _round in 0..rounds {
        for monkey_id in 0..monkeys.len(){
            let next_monkey = &mut monkeys[monkey_id];
            next_monkey.inspect_items(true);
            
            let items_to_throw = next_monkey.get_items_to_throw();
            
            for (item, recieving_monkey) in items_to_throw {
                monkeys[recieving_monkey].items.push(item)
            }
        }        
    }

    monkeys.sort_by(|a,b| a.inspected_count.partial_cmp(&b.inspected_count).unwrap());
    monkeys.reverse();
    // print_monkeys(&monkeys);

    let level_of_monkey_business = monkeys[0].inspected_count * monkeys[1].inspected_count;
    level_of_monkey_business
}

fn part2() -> i64{
    let mut monkeys = get_monkeys();
    let rounds = 10000;

    let lcm = calculate_lcm(&monkeys);

    for _round in 0..rounds {
        for monkey_id in 0..monkeys.len(){
            let next_monkey = &mut monkeys[monkey_id];
            next_monkey.inspect_items(false);
            
            let items_to_throw = next_monkey.get_items_to_throw();
            
            for (item, recieving_monkey) in items_to_throw {
                monkeys[recieving_monkey].items.push(item % lcm)
            }
            
        }
        
        // if round == 20 - 1 {
            // print_monkeys(&monkeys);
        // }
    }

    // print_monkeys(&monkeys);
    monkeys.sort_by(|a,b| a.inspected_count.partial_cmp(&b.inspected_count).unwrap());
    monkeys.reverse();
    

    // let x= 2713310158;
    let level_of_monkey_business = monkeys[0].inspected_count * monkeys[1].inspected_count;
    level_of_monkey_business
}
