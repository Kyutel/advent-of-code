use std::collections::HashSet;

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

    for point1 in &asteroid_co_ordinates {
        let mut line_of_sight = HashSet::new();
        for point2 in &asteroid_co_ordinates {
            let gradient = get_gradient(*point1,*point2);
            line_of_sight.insert(gradient.to_string());
        }
        max_number_asteroids_seen = max_number_asteroids_seen.max(line_of_sight.len() - 1)
    }

    // println!("{:?}",line_of_sight);
        // let mut line_of_sight = HashSet::new();
        // for point in &asteroid_co_ordinates {
        //     let gradient = get_gradient((11.0,13.0),*point);
        //     let b1 = point.0.to_string();
        //     let b2 = point.1.to_string();
            
        //     // if line_of_sight.contains(&gradient){
        //     //     let test = gradient.clone();
        //     //     let another = test + " (" + &b1 + "," + &b2 + ")";
        //     //     println!("already exist, new point at {:?}", another);
        //     // }

        //     line_of_sight.insert(gradient);
        // }

    // let max_number_asteroids_seen = line_of_sight.len() - 1;
    // println!("{:?}", line_of_sight);

    println!("best has {:?}",max_number_asteroids_seen);



}


fn get_gradient(point_a:(f64,f64),point_b:(f64,f64)) -> String{
    let change_in_x = point_a.0 - point_b.0;
    let change_in_y = point_a.1 - point_b.1;

    let gradient:f64 = change_in_x/change_in_y;


    if change_in_x == 0.0 && change_in_y == 0.0 {
        return "same".to_string();
    }

    if change_in_x == 0.0 && change_in_y < 0.0 {
       return "neg0".to_string();
    }

    // println!("x delta: {:?}", change_in_x);
    // println!("y delta: {:?}", change_in_y);
    // println!("gradient: {:?}", gradient);

    // let b2 = point_b
    // if gradient.to_string() == "-4" {
    //     println!("debug");
    //     println!("delta x{:?}",change_in_x);
    //     println!("delta y{:?}",change_in_y);
    //     // println!("{:?}",change_in_x);
    // }

    if change_in_y < 0.0 {
        return "+".to_string() + &gradient.to_string();
    }



    gradient.to_string()
}   


fn get_data() -> Vec<Vec<char>> {
    include_str!("data.txt").split("\r\n")
    .map(|i| i.chars().collect())
    .collect()
}