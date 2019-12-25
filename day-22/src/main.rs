fn main() {

    part1();

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