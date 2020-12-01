use std::collections::HashSet;


fn main() {

    let mut moons = vec![(6,-2,-7),(-6,-7,-4),(-9,11,0),(-3,-4,6)];
    // let mut moons = vec![(-1,0,2),(2,-10,-7),(4,-8,8),(3,5,-1)];
    // let mut moons = vec![(-8,-10,0),(5,5,10),(2,-7,3),(9,-8,-3)];
    let mut velocities = vec![(0,0,0); 4];

    let mut moon_states = HashSet::new(); //HashSet::new(); 
    let mut steps = 0;
    // for _x in 0..1000 {
    loop {
        if moon_states.contains(&(moons.clone(),velocities.clone())) {
            break;
        }

        moon_states.insert((moons.clone(),velocities.clone()));
        let mut moon_number = 0;
        for moon1 in &moons {
            for moon2 in &moons {
                if moon1 == moon2 {
                    continue;
                }

                //velocities[moon_number] = adjust_velocity(*moon1,*moon2,velocities[moon_number]);
                velocities[moon_number].2 = adjust_velocity_v2(moon1.2,moon2.2,velocities[moon_number].2);
            }   
            moon_number+=1;
        }

        moon_number = 0;
        for moon in &mut moons {
            apply_velocity(&mut *moon,velocities[moon_number]);
            moon_number +=1;
        }


        steps += 1;
        // println!("{:?}", steps);
    }
    // println!("Moons {:?}", moons);
    // println!("Velocities {:?}", velocities);
    println!("steps {:?}", steps);
    // println!("total potential energy: {:?}", potential_energy);


}


fn adjust_velocity(moon_a: (i32,i32,i32), moon_b:  (i32,i32,i32), velocity: (i32,i32,i32)) -> (i32,i32,i32) {
    let mut new_velocity = velocity;
    if moon_a.0 < moon_b.0 {
        new_velocity.0 +=1;
    }
    else if moon_a.0 > moon_b.0 {
        new_velocity.0 -=1;
    }

    if moon_a.1 < moon_b.1 {
        new_velocity.1 +=1;
    }
    else if moon_a.1 > moon_b.1 {
        new_velocity.1 -=1;
    }

    if moon_a.2 < moon_b.2 {
        new_velocity.2 +=1;
    }
    else if moon_a.2 > moon_b.2 {
        new_velocity.2 -=1;
    }

    new_velocity
}

fn adjust_velocity_v2(moon_a_x: i32,moon_b_x:i32,velocity:i32) -> i32{
    let mut new_velocity = velocity;
    if moon_a_x < moon_b_x {
        new_velocity +=1;
    }
    else if moon_a_x > moon_b_x {
        new_velocity -=1;
    }

    new_velocity
}

fn apply_velocity(moon: &mut(i32,i32,i32), velocity: (i32,i32,i32)) {
    moon.0 = moon.0 + velocity.0;
    moon.1 = moon.1 + velocity.1;
    moon.2 = moon.2 + velocity.2;
}

fn get_total_energy(moons: &Vec<(i32,i32,i32)>,velocities: &Vec<(i32,i32,i32)>) -> i32 {
    let mut total_energy = 0;
    for i in 0..4{
        total_energy += get_energy(moons[i]) * get_energy(velocities[i]);
    }
    total_energy
}

fn get_energy(point: (i32,i32,i32)) -> i32 {
    point.0.abs() + point.1.abs() + point.2.abs()
}
