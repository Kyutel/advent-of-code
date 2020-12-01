use std::collections::HashMap;

fn main() {
  let mut valid_passes =0;

  for number in 264360..746325{
    if check_valid_password(number.to_string()){
      valid_passes+=1;
    }

  }
  
  println!("{:?}",valid_passes);

  // println!("{:?}", check_valid_password(112223.to_string()));


}


fn check_valid_password(password: String) -> bool{
  // let mut test = password.chars().peekable();
  // while test.next() != None {
  //   println!("{:?}, P:{:?}",test,test.peek() );
  //   // println!("",test.peek() );
  // }


  let mut last_char = None;
  let mut has_double = false;
  // let mut double_value = None;
  let mut multiple = HashMap::new();


  for c in password.chars().peekable() {
    if last_char!= None{
      if last_char > c.to_digit(10)
      {
        return false
      }

      if last_char == c.to_digit(10){
        let digit = c.to_digit(10);

        if !multiple.contains_key(&digit)
        {
          multiple.insert(digit,2);
        }
        else{
          let number = multiple.remove(&digit);
          multiple.insert(digit,number.unwrap() + 1);
        }
      }
    }
    last_char = c.to_digit(10)
  }

  for (key,value) in multiple
  {
      // println!("{:?} {:?}",key,value );
      if value == 2{
        has_double = true;
      }
  }

  return has_double;
}