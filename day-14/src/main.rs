use std::collections::HashMap;

type Recipies = HashMap<&'static str, (&'static str,Vec<Vec<&'static str>>)>;

fn main() {
    let recipies = get_recipies();

    // println!("{:?}", recipies);
    let mut leftovers = HashMap::new();
    let mut amount_of_ore = 1000000000000i64;
    let mut amount_made = 0;
    loop {
        let ore_req = get_ore_requirement("FUEL",1,&recipies,&mut leftovers);
        amount_of_ore -= ore_req as i64;
        println!("{:?}", amount_of_ore);
        if amount_of_ore < 0 {
            break;
        }
        amount_made +=1;
    }
    
    // println!("{:?} ores needed", ore_req);
    println!("{:?}", amount_made);
    // println!("{:?}", leftovers);


}


fn get_ore_requirement(chem_to_make: &'static str, amount_to_make: i32,recipies: &Recipies,mut leftovers: &mut HashMap<&'static str,i32>) -> i32{
    let recipe = recipies.get(chem_to_make).unwrap();
    let mut ore_req = 0;
    let mut amount_made = 0;
    // println!("{:?}", recipe);

    let previous_leftover:i32;
    match leftovers.get(chem_to_make) {
        Some(amount) => previous_leftover = *amount,
        None => previous_leftover = 0
    };

    amount_made += previous_leftover;
    leftovers.insert(chem_to_make,0);

    while amount_made < amount_to_make {
        for ingredient in &recipe.1 {
            if ingredient[1] == "ORE" {
                ore_req += ingredient[0].parse::<i32>().unwrap();
            }
            else
            {
                let ingredient_amount = ingredient[0].parse::<i32>().unwrap();
                ore_req += get_ore_requirement(ingredient[1], ingredient_amount, &recipies,&mut leftovers);
            }
        }
        amount_made+= recipe.0.parse::<i32>().unwrap();
    }

    let leftover:i32 = amount_made - amount_to_make ;
    let previous_leftover:i32;
    match leftovers.get(chem_to_make) {
        Some(amount) => previous_leftover = *amount,
        None => previous_leftover = 0
    };

    leftovers.insert(chem_to_make,previous_leftover + leftover);
    
    ore_req
}

fn get_data()  -> Vec<&'static str>{
    include_str!("data.txt").split("\r\n")
    .collect()
}

fn get_recipies() -> Recipies{
    let data = get_data();
    let mut recipies = HashMap::new();

    for recipe in data{

        let arrow_location = recipe.find('>').unwrap();
        let left: Vec< Vec<&'static str>> = {
            let temp = &recipe[..arrow_location-2];
            temp.split(", ")
                .map(|x| x.split(' ').collect())
                    .collect()
        };
        let right:  Vec<&'static str> = (&recipe [arrow_location+2..]).split(' ').collect();

        recipies.insert(right[1],(right[0],left));
    }

    recipies
}