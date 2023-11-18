use std::cmp::{PartialEq, PartialOrd};

pub fn run_sort_vector() {
    let mut vec = vec![1.1, 1.15, 5.5];

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("{:?}", vec);

    sort_struct();
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

pub fn sort_struct() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];
    println!("Struct: {:?}", people);

    people.sort();
    println!("Struct: {:?}", people);

    people.sort_by(|a, b| b.age.cmp(&a.age));
    println!("Struct: {:?}", people);
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Self {
            name,
            age,
        }
    }
}
