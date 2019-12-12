use std::collections::HashSet;
use std::cmp::Ordering;

fn main() {
    let data = get_data();

    // println!("{:?}", data);

    let mut asteroid_co_ordinates = Vec::new();
    //get asteroids
    for y in 0..data.len(){
        for x in 0..data[y].len() {
            if data[y][x] == '#' {
                // println!("{:?}, {:?}", x,y);
                asteroid_co_ordinates.push((x as f64,y as f64))
            }
        }
    }

    // println!("{:?}",asteroid_co_ordinates );
    



    let mut max_number_asteroids_seen = 0;
    let mut grads_from_best = HashSet::new();
    let mut station_location = (-1.0,-1.0);

    for point1 in &asteroid_co_ordinates {
        let mut line_of_sight = HashSet::new();
        for point2 in &asteroid_co_ordinates {
            let gradient = get_gradient(*point1,*point2);
            line_of_sight.insert(gradient.to_string());
        }
        if line_of_sight.len() - 1 > max_number_asteroids_seen {
            station_location = point1.clone();
            grads_from_best = line_of_sight.clone();
        }
        max_number_asteroids_seen = max_number_asteroids_seen.max(line_of_sight.len() - 1)
        
    }

    // println!("gradients: {:?}", grads_from_best);
    println!("best has {:?}",max_number_asteroids_seen);
    println!("station location is: {:?}", station_location);

    let mut grads_as_vec:Vec<String> = grads_from_best.into_iter()
    .filter(|x| x!= "same")
    .collect();

    grads_as_vec.sort_by(|a,b| {
        let a_char = a.chars().next().unwrap();
        let b_char = b.chars().next().unwrap();
        if a_char == '+'  && b_char != '+' {
            return Ordering::Less
        }
        else if a_char != '+'  && b_char == '+'{
            return Ordering::Greater
        }
        else if a_char == '+'  && b_char == '+'{
            let a_number = &a[1..].parse::<f64>().unwrap();  
            let b_number = &b[1..].parse::<f64>().unwrap();
            return b_number.partial_cmp(a_number).unwrap_or(Ordering::Equal)
        }
        else {
            if a == "same" || a == "neg0" {
                return Ordering::Equal;
            }
            else if b == "same" || b == "neg0" {
                return Ordering::Equal;
            }

            let a_number = &a.parse::<f64>().unwrap();  
            let b_number = &b.parse::<f64>().unwrap();
            return b_number.partial_cmp(a_number).unwrap_or(Ordering::Equal)
        }
    });

    // println!("{:?}",grads_as_vec);


    // let test_value:Vec<(f64,f64)> = asteroid_co_ordinates.into_iter()
    // .filter(|x| x!= &station_location)
    // .collect();
    // println!("{:?}", test_value);
    let number = 200 % grads_as_vec.len();
    let number = number - 1;
    println!("{:?}",grads_as_vec.len() );
    println!("number {:?}",number );

    let test = &grads_as_vec[number];
    println!("{:?}",test );

    for point in &asteroid_co_ordinates {
        if &get_gradient(station_location,*point) == test{
            println!("{:?}",point );
            println!("{:?}",point.0 * 100.0 + point.1 );
        }
    }

}


fn get_gradient(point_a:(f64,f64),point_b:(f64,f64)) -> String{
    let change_in_x = point_b.0 - point_a.0;
    let change_in_y = point_a.1 - point_b.1;

    let gradient:f64 = change_in_y/change_in_x;


    if change_in_x == 0.0 && change_in_y == 0.0 {
        return "same".to_string();
    }

    if change_in_x == 0.0 && change_in_y < 0.0 {
       return "-0".to_string();
    }

    // println!("x delta: {:?}", change_in_x);
    // println!("y delta: {:?}", change_in_y);
    // println!("gradient: {:?}", gradient);

    if change_in_x >= 0.0 {
        return "+".to_string() + &gradient.to_string();
    }

    gradient.to_string()
}   


fn get_data() -> Vec<Vec<char>> {
    include_str!("data.txt").split("\r\n")
    .map(|i| i.chars().collect())
    .collect()
}