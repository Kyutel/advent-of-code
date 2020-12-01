use modinverse::modinverse;
use std::collections::HashMap;

fn main() {

    part1();
    part2();

}

fn part2 () {
    let mut index = 2020i128;
    // let mut index = 2019i128;
    const TOTAL_CARDS:i128 = 119315717514047;
    // const TOTAL_CARDS:i128 = 10007;
    const SHUFFLE_TIMES:i128 = 101741582076661;
    // const SHUFFLE_TIMES:i128 = 44;

    // let mut index = 2020i128;
    // const TOTAL_CARDS:i128 = 119315717514047;
    let mut a = 1i128;
    let mut b = 0i128;

    for shuffle in include_str!("data.txt").split("\r\n"){
        let new_a;
        let new_b;
        if shuffle == "deal into new stack" {
            new_a = -1;
            new_b = -1;
        }
        else if shuffle.starts_with("cut") {
            let n = (&shuffle[4..]).parse::<i128>().unwrap();
            new_a = 1;
            new_b = 0-n;
        }
        else if shuffle.starts_with("deal with increment") {
           let n = (&shuffle[20..]).parse::<i128>().unwrap();
           new_a = n;
           new_b = 0;        }
        else
        { 
            println!("error unrecoginised shuffle");
            new_a = 1;
            new_b = 0;
        }

        a = a*new_a;
        b = (new_a*b) + new_b;
        a = a% TOTAL_CARDS;
        b = b% TOTAL_CARDS;
        // println!("a: {:?}, b: {:?}", a,b);

    }


    // println!("first: a: {:?}, b: {:?}", a,b);
    let mut ab_store = Vec::new();
    let mut times = 1;
    ab_store.push((a,b));
    loop{
        // println!("creating pow 2: {:?}", times);
        if times * 2 > SHUFFLE_TIMES {
            break;
        }
        let new_a = a;
        let new_b = b;

        a = a*new_a;
        b = (new_a*b) + new_b;
        a = a% TOTAL_CARDS;
        b = b% TOTAL_CARDS;
        times *= 2;
        ab_store.push((a,b));
    }
    let mut next_to_use = ab_store.len() -1;

    while times != SHUFFLE_TIMES {
        if times + 2i128.pow(next_to_use as u32) <= SHUFFLE_TIMES {
            let new_a = ab_store[next_to_use].0;
            let new_b = ab_store[next_to_use].1;

            a = a*new_a;
            b = (new_a*b) + new_b;
            a = a% TOTAL_CARDS;
            b = b% TOTAL_CARDS;

            times += 2i128.pow(next_to_use as u32);
            // println!("diff {:?}",SHUFFLE_TIMES - times );
        }else {
            if next_to_use == 0{
                continue;
            }

            next_to_use -=1
        }
    }

    // println!("diff {:?}", SHUFFLE_TIMES - times);
    // println!("second: a: {:?}, b: {:?}", a,b);

    let mod_inv:i128 =  modinverse(a,TOTAL_CARDS).unwrap();

    let mut card_at_index = ((index - b  ) *mod_inv) % TOTAL_CARDS;
    if card_at_index < 0
    {
        card_at_index = TOTAL_CARDS + card_at_index;
    }
    
    println!("in pos 2020: {:?}", card_at_index);
    let mut rev_test =  (a * card_at_index + b) % TOTAL_CARDS;

    if rev_test < 0
    {
        rev_test = TOTAL_CARDS + rev_test;
    }

    println!("reverse test {:?}", rev_test);
}

fn part1() {
    let mut deck:Vec<i64> = (0..10007).collect();

    for shuffle in include_str!("data.txt").split("\r\n"){
        if shuffle == "deal into new stack" {
            deck = deal_into_new_stack(&deck);
        }
        else if shuffle.starts_with("cut") {
            let n = (&shuffle[4..]).parse::<i64>().unwrap();
            deck = cut_deck(&deck,n);
        }
        else if shuffle.starts_with("deal with increment") {
           let n = (&shuffle[20..]).parse::<i64>().unwrap();
           deck = deal_with_increment(&deck,n);
        }
        else
        {
           println!("error unrecoginised shuffle");
        }
    } 
    println!("correct answer {:?}", deck.iter().position(|x| *x == 2019).unwrap());
}


fn deal_into_new_stack(deck:&Vec<i64>) -> Vec<i64>
{ 
    deck.iter().rev().map(|x| *x).collect()
}

fn cut_deck(deck:&Vec<i64>,n:i64) -> Vec<i64> {
    let index =  if n < 0 {
        (deck.len() as i64 + n) as usize
    }
    else {
       n as usize
    };

    let upper_half = &deck[..index];
    let lower_half = &deck[index..];


    [lower_half,upper_half].concat()
}

fn deal_with_increment(deck:&Vec<i64>,n:i64) -> Vec<i64> {
    let mut new_deck = vec![-1;deck.len()];
    for index in 0..deck.len(){
        new_deck[(index * n as usize)%deck.len()] = deck[index];
    }

    new_deck
}