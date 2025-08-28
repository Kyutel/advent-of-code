use std::{cmp::Ordering, collections::HashMap};

// static FILE_CONTENTS: &str = include_str!("test-input.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

#[derive(Debug, Clone, Eq, PartialEq)]
struct Hand {
    cards: String,
    bid: i32,
    value: Option<i32>,
}


fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}


fn convert_card_to_value(letter: char) -> i32 {
    if letter.is_numeric(){
        return letter.to_digit(10).unwrap() as i32
    }
    else if letter == 'A'
    {
        return 14;
    }
    else if letter == 'K'
    {
       return 13
    }
    else if letter == 'Q' {
        return 12
    }
    else if letter == 'J' {
        //return 11 //11 for part one
        return 1 //1 for part two
    }
    else if letter == 'T' {
        return 10
    }

    return -1;
}


impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.value != other.value {
            return self.value.unwrap().cmp(&other.value.unwrap());
        }
        else {


            for( self_c,other_c ) in self.cards.chars().zip(other.cards.chars()) {
                if self_c != other_c {
                    return convert_card_to_value(self_c).cmp(&convert_card_to_value(other_c))
                }
            }

            println!("{:?}", self);
            println!("{:?}", other);
            
            return Ordering::Equal
        }
    }
}


//1 high
//2 one pair
//3 two pair
//4 three kind
//5 full house
//6 four of kind
//7 five of kind

fn get_score(hand: &Hand) -> Option<i32> {
    let value;
    let mut card_map = HashMap::new();
    for card in hand.cards.chars() {
        // card_map.get_mut(k)
        match card_map.get(&card) {
            Some(count) => {card_map.insert(card, count + 1);}
            None => {card_map.insert(card, 1);}
        }
    }

    if card_map.len() == 1 {
        value = Some(7);
    }
    else if card_map.len() == 2 {
        let counts: Vec<i32> = card_map.values().cloned().collect();
        if counts[0] == 4 || counts[1] == 4{
            value = Some(6)
        } 
        else  { // else if counts[0] == 3 && counts[1] == 2 ||  counts[1] == 3 && counts[0] == 2{
            value = Some(5)
        }
    }
    else if card_map.len() == 3 {
        let counts: Vec<i32> = card_map.values().cloned().collect();
        if counts[0] == 3 || counts[1] == 3 || counts[2] == 3 {
            value = Some(4)
        }
        else {
            value = Some(3)
        }
    }
    else if card_map.len() == 4 {
        value = Some(2)
    }
    else {
        value = Some(1)
    }
    
    value

}

fn get_score_2(hand: &Hand) -> Option<i32> {
    let value;
    let mut card_map = HashMap::new();
    for card in hand.cards.chars() {
        // card_map.get_mut(k)
        match card_map.get(&card) {
            Some(count) => {card_map.insert(card, count + 1);}
            None => {card_map.insert(card, 1);}
        }
    }

    if !card_map.contains_key(&'J') {
        return get_score(hand)
    }
    if card_map.len() == 1  || card_map.len() == 2 {
        value = Some(7);
    }
    else if card_map.len() == 3 {
        let counts: Vec<i32> = card_map.values().cloned().collect();
        // ABJJJ 4 of kind
        // AABJJ 4 of kind
        // AAABJ 4 of a kind
        // AABBJ full house
        if (card_map.get(&'J') == Some(&1)) &&(counts[0] == 2 || counts[1] == 2 || counts[2] == 2){
            value = Some(5)
        }
        else {
            value = Some(6)
        }
    }
    else if card_map.len() == 4 {
        //ABCJJ THREE OF KIND
        //AABCJ THREE OF KIND
        value = Some(4)
    }
    else {
        value = Some(2)
    }


    // value = None;
    value
}

fn get_hands() -> Vec<Hand> {
    let hands = FILE_CONTENTS.split("\n").map(|line| Hand{
        cards: line[0..5].to_string(),
        bid: line[6..].parse::<i32>().unwrap(),
        value: None,
    }).collect();

    hands
}

fn part1 () -> i32 {
    let mut hands = get_hands();
    for hand in hands.iter_mut(){
        let val = get_score(&hand);
        hand.value = val;
    }

    hands.sort();
    
    // println!("{:?}", hands);


    let mut answer = 0;
    for (index, hand) in hands.into_iter().enumerate() {
        answer += (index + 1) as i32 * hand.bid 
    }
    
    answer
}
fn part2 () -> i32{
let mut hands = get_hands();
    for hand in hands.iter_mut(){
        let val = get_score_2(&hand);
        hand.value = val;
    }

    hands.sort();
    
    println!("{:?}", hands);


    let mut answer = 0;
    for (index, hand) in hands.into_iter().enumerate() {
        answer += (index + 1) as i32 * hand.bid 
    }
    
    answer

}
