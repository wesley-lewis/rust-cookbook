use rand::prelude::*;
use rand::distributions::Alphanumeric;

pub fn generate_random_password(size: usize) {
    prints("Random Password");

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect();

    println!("{}", rand_string);

}

pub fn random_user_defined_password(mut rng: ThreadRng, charset: &[u8], password_len: usize) -> ThreadRng{
    prints("User defined password");
    let password: String = (0..password_len)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
        
    println!("{}", password);

    rng
}

pub fn generate_random_numbers(mut rng: ThreadRng) -> ThreadRng {
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

pub fn generate_with_range(mut rng:ThreadRng, start: i32, end: i32) -> ThreadRng {
    prints("Generate within range");
    let n1 = rng.gen_range(start..end);
    println!("{}", n1);

    rng
}

pub fn run() {
    let mut rng: ThreadRng = rand::thread_rng();
    rng = generate_random_numbers(rng);
    rng = generate_with_range(rng, 0, 10);
    generate_random_password(20);
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

    rng = random_user_defined_password(rng, CHARSET, 20);
}

fn prints(value: &str) {
    println!("\n-----------------------------------{}-----------------------------------", value);
}
