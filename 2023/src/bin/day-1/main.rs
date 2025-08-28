// static FILE_CONTENTS: &str = include_str!("test-input2.txt");
static FILE_CONTENTS: &str = include_str!("input.txt");

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn get_value(row: &str) -> u32 {
    let mut first_num= 0;
    let mut last_num = 0;
    for c in row.chars() {
        if c.is_numeric() {
            first_num = c.to_digit(10).unwrap();
            break;
        }
    };
    for c in row.chars().rev() {
        if c.is_numeric() {
            last_num = c.to_digit(10).unwrap();
            break;
        }
    };

    return first_num * 10 + last_num;
}


fn get_value_p2(row: &str) -> u32 {
    let mut first_num= 0;
    let mut last_num = 0;

    let mut temp: String = "".to_string();
    for c in row.chars() {
        if c.is_numeric() {
            first_num = c.to_digit(10).unwrap();
            break;
        }
        else {
            temp.push(c);
            if temp.find("one").is_some(){
                first_num = 1;
                break;
            }
            if temp.find("two").is_some(){
                first_num = 2;
                break;
            }
            if temp.find("three").is_some(){
                first_num = 3;
                break;
            }
            if temp.find("four").is_some(){
                first_num = 4;
                break;
            }
            if temp.find("five").is_some(){
                first_num = 5;
                break;
            }
            if temp.find("six").is_some(){
                first_num = 6;
                break;
            }
            if temp.find("seven").is_some(){
                first_num = 7;
                break;
            }
            if temp.find("eight").is_some(){
                first_num = 8;
                break;
            }
            if temp.find("nine").is_some(){
                first_num = 9;
                break;
            }
        }
    };

    let mut temp = "".to_string();

    for c in row.chars().rev() {
        if c.is_numeric() {
            last_num = c.to_digit(10).unwrap();
            break;
        }
        else {
            let char_as_str = c.to_string();
            temp = char_as_str + &temp;
            if temp.find("one").is_some(){
                last_num = 1;
                break;
            }
            if temp.find("two").is_some(){
                last_num = 2;
                break;
            }
            if temp.find("three").is_some(){
                last_num = 3;
                break;
            }
            if temp.find("four").is_some(){
                last_num = 4;
                break;
            }
            if temp.find("five").is_some(){
                last_num = 5;
                break;
            }
            if temp.find("six").is_some(){
                last_num = 6;
                break;
            }
            if temp.find("seven").is_some(){
                last_num = 7;
                break;
            }
            if temp.find("eight").is_some(){
                last_num = 8;
                break;
            }
            if temp.find("nine").is_some(){
                last_num = 9;
                break;
            }
        }
    };

    return first_num * 10 + last_num;
}

fn part1 () -> u32 {
    FILE_CONTENTS.split("\n")
    .map(|row| get_value(row))
    .sum()
}


fn part2() -> u32 {
    FILE_CONTENTS.split("\n")
    .map(|row| get_value_p2(row))
    .sum()
}
