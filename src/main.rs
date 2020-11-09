use std::{thread, time};
use std::sync::{Arc};

// 3 people sit down to eat form the same plate, make sure that they all get to eat in turn
struct DiningPeople { 
    num_bites: i32,
}

impl DiningPeople {
    pub fn new(bites: i32) -> DiningPeople {
        return DiningPeople { 
            num_bites: bites,
        }
    }

    pub fn one_eat(&self) {
        for _ in 0..self.num_bites {
            println!("Person one eats");
            eat()
        }
    }

    pub fn two_eat(&self) {
        for _ in 0..self.num_bites {
            println!("Person two eats");
            eat()
        }
    }

    pub fn three_eat(&self) {
        for _ in 0..self.num_bites {
            println!("Person three eats");
            eat()
        }
    }
}

fn eat() {
    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);
}

fn main() {
    // We have to use are to allow multiple references to the heap instance of people
    let people = Arc::new(DiningPeople::new(5));

    let mut handles = vec![];

    // spawn 3 threads to represent the 3 people eating
    {
        let people = Arc::clone(&people);
        let handle = thread::spawn(move || {
                people.one_eat();
            }
        );
        handles.push(handle);
    }

    {
        let people = Arc::clone(&people);
        let handle = thread::spawn(move || {
                people.two_eat();
            }
        );
        handles.push(handle);
    }

    {
        let people = Arc::clone(&people);
        let handle = thread::spawn(move || {
                people.three_eat();
            }
        );
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
