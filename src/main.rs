#![allow(unused_assignments)]

use rand::prelude::*;
use rand::distributions::Alphanumeric;

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();
    rng = generate_random_numbers(rng);
    rng = generate_with_range(rng, 0, 10);
    generate_random_password(20);
}

fn generate_random_password(size: usize) {
    prints("Random Password");

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect();

    println!("{}", rand_string);

}

fn generate_random_numbers(mut rng: ThreadRng) -> ThreadRng {
    prints("Generate random numbers");

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();

    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());

    rng
}

fn generate_with_range(mut rng:ThreadRng, start: i32, end: i32) -> ThreadRng {
    prints("Generate within range");
    let n1 = rng.gen_range(start..end);
    println!("{}", n1);

    rng
}



fn prints(value: &str) {
    println!("\n-----------------------------------{}-------------------------------------", value);
}
