fn main() {
	let OGviktor2 = vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,5,19,23,2,10,23,27,1,27,5,31,2,9,31,35,1,35,5,39,2,6,39,43,1,43,5,47,2,47,10,51,2,51,6,55,1,5,55,59,2,10,59,63,1,63,6,67,2,67,6,71,1,71,5,75,1,13,75,79,1,6,79,83,2,83,13,87,1,87,6,91,1,10,91,95,1,95,9,99,2,99,13,103,1,103,6,107,2,107,6,111,1,111,2,115,1,115,13,0,99,2,0,14,0];

	let mut newViktor = OGviktor2.clone();

    for x in 0..99{
    	for y in 0..99{
    		newViktor[1] = x;
    		newViktor[2] = y;
    		do_intcode(&mut newViktor);
    		newViktor = OGviktor2.clone();
    	}
    }
}


fn do_intcode(viktor: &mut Vec<i32>) -> () {
	let mut viktor1 = viktor.clone();
	let mut viktor2 = viktor.clone();

    for index in (0..viktor.len()).step_by(4) {

    	if viktor[index] == 1{
			viktor1[viktor2[index+3] as usize] = viktor2[viktor2[index+1] as usize] + viktor2[viktor2[index+2] as usize]
		}
		else if viktor[index] == 2{
			viktor1[viktor2[index+3] as usize] = viktor2[viktor2[index+1] as usize] * viktor2[viktor2[index+2] as usize]
		}
		else if viktor[index] == 99{
			break;
		}
		else {
			println!("????");
		}

		viktor2 = viktor1.clone();
    }

    // println!("{:?}",viktor1 );
    if viktor1[0] == 19690720
    {
    	println!("FOUND IT");
    	let number = 100 * viktor1[1] + viktor1[2];
    	println!("{:?}", number);
    }

}