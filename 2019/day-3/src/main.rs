use std::collections::HashSet;
use std::cmp;

#[derive(Hash, Copy,Clone,Eq, PartialEq, Debug)]
struct Point {
  x: i64,
  y: i64
}

fn main() {
    // let mut viktor1 = vec![];
    // let mut viktor2 = vec![];

	// wire.insert(create_point(10,5));
	let mut intersect = HashSet::new();

	let mut viktor1 = vec!["R1004","U518","R309","D991","R436","D360","L322","U627","R94","D636","L846","D385","R563","U220","L312","D605","L612","D843","R848","U193","L671","D852","L129","D680","L946","D261","L804","D482","R196","U960","L234","U577","R206","D973","R407","D400","R44","D103","R463","U907","L972","U628","L962","U856","L564","D25","L425","U332","R931","U837","R556","U435","R88","U860","L982","D393","R793","D86","R647","D337","R514","D361","L777","U640","R833","D674","L817","D260","R382","U168","R161","U449","L670","U814","L42","U461","R570","U855","L111","U734","L699","U602","R628","D79","L982","D494","L616","D484","R259","U429","L917","D321","R429","U854","R735","D373","L508","D59","L207","D192","L120","D943","R648","U245","L670","D571","L46","D195","L989","U589","L34","D177","L682","U468","L783","D143","L940","U412","R875","D604","R867","D951","L82","U851","L550","D21","L425","D81","L659","D231","R92","D232","R27","D269","L351","D369","R622","U737","R531","U693","R295","U217","R249","U994","R635","U267","L863","U690","L398","U576","R982","U252","L649","U321","L814","U516","R827","U74","L80","U624","L802","D620","L544","U249","R983","U424","R564","D217","R151","U8","L813","D311","R203","U478","R999","U495","R957","U641","R40","U431","L830","U67","L31","U532","R345","U878","L996","D223","L76","D264","R823","U27","L776","U936","L614","U421","L398","U168","L90","U525","R640","U95","L761","U938","R296","D463","L349","D709","R428","U818","L376","D444","L748","D527","L755","U750","R175","U495","R587","D767","L332","U665","L84","D747","L183","D969","R37","D514","R949","U985","R548","U939","L170","U415","R857","D480","R836","D363","R763","D997","R721","D140","R699","U673","L724","U375","R55","U758","R634","D590","L608","U674","R809","U308","L681","D957","R30","D913","L633","D939","L474","D567","R290","D615","L646","D478","L822","D471","L952","D937","R306","U380","R695","U788","R555","D64","R769","D785","R115","U474","R232","U353","R534","D268","L434","U790","L777","D223","L168","U21","L411","D524","R862","D43","L979","U65","R771","U872","L983","U765","R162"];
    let mut viktor2 = vec!["L998","U952","R204","U266","R353","U227","L209","D718","L28","D989","R535","U517","L934","D711","R878","U268","L895","D766","L423","U543","L636","D808","L176","U493","R22","D222","R956","U347","R953","U468","R657","D907","R464","U875","L162","U225","L410","U704","R76","D985","L711","U176","R496","D720","L395","U907","R223","D144","R292","D523","R514","D942","R838","U551","L487","D518","L159","D880","R53","D519","L173","D449","R525","U645","L65","D568","R327","U667","R790","U131","R402","U869","R287","D411","R576","D265","R639","D783","R629","U107","L571","D247","L61","D548","L916","D397","R715","U138","R399","D159","L523","U2","R794","U699","R854","U731","L234","D135","L98","U702","L179","D364","R123","D900","L548","U880","R560","D648","L701","D928","R256","D970","L396","U201","L47","U156","R723","D759","R663","D306","L436","U508","R371","D494","L147","U131","R946","D207","L516","U514","R992","D592","L356","D869","L299","U10","R744","D13","L52","U749","R400","D146","L193","U720","L226","U973","R971","U691","R657","D604","L984","U652","L378","D811","L325","D714","R131","D428","R418","U750","L706","D855","L947","U557","L985","D688","L615","D114","R202","D746","R987","U353","R268","U14","R709","U595","R982","U332","R84","D620","L75","D885","L269","D544","L137","U124","R361","U502","L290","D710","L108","D254","R278","U47","R74","U293","R237","U83","L80","U661","R550","U886","L201","D527","L351","U668","R366","D384","L937","D768","L906","D388","L604","U515","R632","D486","L404","D980","L652","U404","L224","U957","L197","D496","R690","U407","L448","U953","R391","U446","L964","U372","R351","D786","L187","D643","L911","D557","R254","D135","L150","U833","R876","U114","R688","D654","L991","U717","R649","U464","R551","U886","L780","U293","L656","U681","L532","U184","L903","D42","L417","D917","L8","U910","L600","D872","L632","D221","R980","U438","R183","D973","L321","D652","L540","D163","R796","U404","L507","D495","R707","U322","R16","U59","L421","D255","L463","U462","L524","D703","L702","D904","L597","D385","L374","U411","L702","U804","R706","D56","L288"];

    // let mut viktor1 = vec!["R98","U47","R26","D63","R33","U87","L62","D20","R33","U53","R51"];
    // let mut viktor2 = vec!["U98","R91","D20","R16","D67","R40","U7","R15","U6","R7"];

    // let mut viktor1 = vec!["R8","U5","L5","D3"];
    // let mut viktor2 = vec!["U7","R6","D4","L4"];

    // let mut viktor1 = vec!["R75","D30","R83","U83","L12","D49","R71","U7","L72"];
    // let mut viktor2 = vec!["U62","R66","U55","R34","D71","R55","D58","R83"]; 


	let wire = create_wire(&mut viktor1);
	let wire2 = create_wire(&mut viktor2);

	// let a = create_point(0,2);
	// let b = create_point(5,2);
	// let c = create_point(3,0);
	// let d = create_point(3,15);

	// let z = get_intersection(a,b,c,d);
	// println!("{:?}",z);

	// for point in &wire2 {
	// 	println!("({},{})", point.x,point.y );
	// }


	// println!("", );
	let mut best_steps = std::i64::MAX;
	// let mut steps1 = 0;
	// let mut steps2;

	for w1 in 0..wire.len()-1{

		// steps2 = 0;
		for w2 in 0..wire2.len()-1{


			let a = wire[w1];
			let b = wire[w1+1];
			let c = wire2[w2];
			let d = wire2[w2+1];

			let z = get_intersection(a,b,c,d);

			if z != create_point(-1,-1) && z != create_point(0,0) {
				// println!("z: {:?}", z);
				// println!("b: {:?}", b);
				let mut steps = calculate_steps(w1,w2,&mut viktor1,&mut viktor2);

				if a.x == z.x{
					steps+= (a.y - z.y).abs()
				}
				else {
					steps+= (a.x - z.x).abs()
				}

				if c.x == z.x{
					steps+= (c.y - z.y).abs()
				}
				else {
					steps+= (c.x - z.x).abs()
				}


				best_steps = cmp::min(steps, best_steps);

				intersect.insert(z);
			}
		}
	}

	// println!("{:?}",best_steps );

	// let mut md = std::i64::MAX;

	// for p in intersect
	// {
	// 	// println!("{:?}",p );
	// 	// println!("{:?}",p.x + p.y );
	// 	md = cmp::min(p.x.abs() +p.y.abs(), md);
	// }



	// println!("{:?}", md );
	println!("{:?}", best_steps );


}

fn calculate_steps(wire1:usize,wire2:usize, viktor1: &mut Vec<&str>,viktor2: &mut Vec<&str>) -> i64{
	let mut step1 = 0;
	let mut step2 = 0;

	for x in 0..wire1{
		let direction = viktor1[x];
		let number = &direction[1..].parse::<i64>().unwrap();
		
		step1+= number;
	}

	for x in 0..wire2{
		let direction = viktor2[x];
		let number = &direction[1..].parse::<i64>().unwrap();
		// println!("{:?}",number );
		step2+= number;
	}

	let calulcated = step1 + step2;
	calulcated
}

fn create_point(x:i64,y:i64) -> Point {
	Point { x: x, y: y}
}

fn create_wire(viktor: &mut Vec<&str>) -> Vec::<Point> {
	let mut wire = Vec::new();

	let mut lastx = 0;
	let mut lasty = 0;
	wire.push(create_point(0,0));

	for directions in viktor {
		let mut newx = lastx;
		let mut newy = lasty;

		let letter = &directions[..1];
		let number = &directions[1..].parse::<i64>().unwrap();
		match letter {
			"R" => {
				// println!("right");
				newx+= number;
			}
			"L" => {
				// println!("left");
				newx-= number;
			}
			"U" => {
				// println!("up");
				newy+= number;
			}
			"D" => {
				newy-= number;
				// println!("down");
			} 
			_ => {
				println!("heh");
			}
		}

		wire.push(create_point(newx,newy));

		lastx = newx;
		lasty = newy;
	}

	wire
}

fn det(a:Point,b:Point) -> i64 {
	let dib = a.x * b.y - a.y * b.x;
	dib
}

fn get_intersection(a:Point,b:Point,c:Point,d:Point) -> Point {
	let xdiff = create_point(a.x - b.x, c.x - d.x);
	let ydiff = create_point(a.y - b.y, c.y - d.y);

	let div = det(xdiff,ydiff);
	if div == 0
	{
		return create_point(-1,-1);
	}
	
	let again = create_point(det(a,b),det(c,d));
	let x = det(again,xdiff)/div;
	let y = det(again,ydiff)/div;

	let answer = create_point(x,y);
	let mut small;
	let mut big;

	let mut small2;
	let mut big2;

	
	if a.x > b.x{
		small = b.x;
		big = a.x;
	}
	else {
		big = b.x;
		small = a.x;
	}

	if c.x > d.x{
		small2 = d.x;
		big2 = c.x;
	}
	else {
		big2 = d.x;
		small2 = c.x;
	}


	if answer.x >= small && answer.x <= big && answer.x >= small2 && answer.x <= big2  {

		if a.y > b.y{
			small = b.y;
			big = a.y;
		}
		else {
			big = b.y;
			small = a.y;
		}

		if c.y > d.y{
			small2 = d.y;
			big2 = c.y;
		}
		else {
			big2 = d.y;
			small2 = c.y;
		}

		if answer.y >= small && answer.y <= big && answer.y >=small2 && answer.y <= big2  {
			return answer;
		}
	}

	return create_point(-1,-1);
}