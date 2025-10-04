// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    strength: u32
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_input_strings() -> Vec<String> {
    FILE_CONTENTS.split(',').map(|text| text.to_string()).collect()
}

fn get_hash_value(input_string: &String) -> usize {
    let mut hash_value = 0;
    for c in input_string.chars() {
        hash_value += c as usize;
        hash_value *= 17;
        hash_value %= 256;
    }

    hash_value
}


fn get_focus_power(boxes:  Vec<Vec<Lens>>) -> usize {
    let mut focus_power = 0;

    for (box_no, box_contents ) in boxes.iter().enumerate() {
        for (slot_no, lens) in box_contents.iter().enumerate() {
            focus_power += (box_no+1)  * (slot_no+1) * lens.strength as usize
        }
    }

    focus_power
}

fn part1 () -> usize{
    let input_strings = get_input_strings();

    return input_strings.iter().map(|input_string| get_hash_value(input_string)).sum()
}


fn part2 () -> usize {
    // println!("Box ID {:?}", get_hash_value(&"qp".to_string()))

    let input_strings = get_input_strings();
    let mut boxes: Vec<Vec<Lens>>= vec![vec![];256];
    // let mut label_locations = HashMap::new();

    for input_string in input_strings {
        if input_string.chars().last() == Some('-') {
            let label = input_string[0..input_string.len()-1].to_string();
            let box_id: usize = get_hash_value(&label);
            boxes[box_id].retain(|lens| lens.label!=label);

        }
        else {
            let label = input_string[0..input_string.len()-2].to_string();
            let box_id = get_hash_value(&label);
            let lens_strength = input_string.chars().last().unwrap().to_digit(10).unwrap();
            if let Some(lens_index) =  boxes[box_id].iter().position(|lens|lens.label == label) {
                boxes[box_id][lens_index].strength = lens_strength
            }
            else {
                boxes[box_id].push(Lens { label, strength: lens_strength});
            }
           
        }

        // println!("Box 0: {:?}\nBox 1: {:?}\nBox 3: {:?}", boxes[0], boxes[1], boxes[3]);
        // println!()
    }

    get_focus_power(boxes)

}
