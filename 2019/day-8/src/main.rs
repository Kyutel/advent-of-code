fn main() {
    // println!("Hello, world!");
    let data = read_file();
    // let data = vec![1,2,3,4,5,6,7,8,9,0,1,2];
    // let data = vec![0,2,2,2,1,1,2,2,2,2,1,2,0,0,0,0];

    // let img_w = 2;
    // let img_h = 2;

    let img_w = 25;
    let img_h = 6;
    let img_layer_size = img_w * img_h;

    let mut img = Vec::new();
    let number_of_layers = data.len() / img_layer_size;

    for i in 0..number_of_layers {
        let mut j = 0;
        let mut temp_vec = Vec::new();
        while j < img_layer_size {
            temp_vec.push(data[(i*img_layer_size)+j]);
            j+=1;
        }
        img.push(temp_vec);
    }

    // println!("{:?}", img);
    println!("{:?}, {:?}",img_w,img_h);
    println!("{:?}",img_layer_size);
    println!("{:?}",data.len() );
    println!("{:?}",number_of_layers);

    let mut layered_image = Vec::new();
    for pixel in 0..img_layer_size {
        let mut next_pixel = 2;
        for layer in 0..number_of_layers {
            if img[layer][pixel] != 2{
                next_pixel = img[layer][pixel];
                break;
            }
            
        }
        // println!("");
        layered_image.push(next_pixel);
    }

    // println!("{:?}", layered_image);

    for i in 0..img_h {
        for j in 0..img_w {
            // println!("{:?}", i*img_w + j);
            // if layered_image[i*img_w + j] == 0 {
            //     print!("#");
            // }
            // else
            // {
            //     print!(" ");
            // }
            // println!("");
            print!("{:?}",layered_image[i*img_w + j]);
        } 
        println!("");
    }


    /*
        println!("{:?}", img);
        let mut layer_with_least_0_digit = 0;
        let mut least_0_digit_count = core::i32::MAX;

        for (layer_no,layer) in img.iter().enumerate() {
            let mut zero_count = 0;
            for pixel in layer {
                if *pixel == 0 {
                    zero_count+=1;
                }
            }
            if zero_count < least_0_digit_count {
                layer_with_least_0_digit = layer_no;
                least_0_digit_count = zero_count;
            }
        }

        println!("{:?}", layer_with_least_0_digit);
        let mut digit_1_count = 0;
        let mut digit_2_count = 0;
        let layer = &img[layer_with_least_0_digit];
        for pixel in layer.iter() {
            if *pixel == 1 {
                digit_1_count += 1;
            }
            if *pixel == 2 {
                digit_2_count += 1;
            }
        }

        let answer = digit_1_count * digit_2_count;
        println!("Answer: {:?}", answer);
    */

}

fn read_file() -> Vec<u32> {
    include_str!("data.txt").chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}