use std::{thread, time};
use std::sync::{Arc, Mutex, Condvar};

// 3 people sit down to eat form the same plate, make sure that they all get to eat in turn
struct DiningPeople { 
    num_bites: i32,
    one_can_eat: Arc<(Mutex<bool>, Condvar)>,
    two_can_eat: Arc<(Mutex<bool>, Condvar)>,
    three_can_eat: Arc<(Mutex<bool>, Condvar)>,
}

impl DiningPeople {
    pub fn new(bites: i32) -> DiningPeople {
        DiningPeople { 
            num_bites: bites,
            one_can_eat: Arc::new((Mutex::new(true), Condvar::new())),
            two_can_eat: Arc::new((Mutex::new(false), Condvar::new())),
            three_can_eat: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }

    pub fn one_eat(&self) {
        for _ in 0..self.num_bites {
            // one
            let (one_lock, one_cvar) = &*self.one_can_eat;
            let mut one_eat = one_lock.lock().unwrap();

            while !*one_eat {
                one_eat = one_cvar.wait(one_eat).unwrap();
            } 

            // two
            let (two_lock, two_cvar) = &*self.two_can_eat;
            let mut two_eat = two_lock.lock().unwrap();

            println!("Person one eats");
            eat();
            *two_eat = true;
            *one_eat = false;
            two_cvar.notify_one();
        }
    }

    pub fn two_eat(&self) {
        for _ in 0..self.num_bites {

            // two
            let (two_lock, two_cvar) = &*self.two_can_eat;
            let mut two_eat = two_lock.lock().unwrap();


            while !*two_eat {
                two_eat = two_cvar.wait(two_eat).unwrap();
            }

             // one
             let (one_lock, one_cvar) = &*self.one_can_eat;
             let mut one_eat = one_lock.lock().unwrap();

            println!("Person two eats");
            eat();
            *one_eat = true;
            *two_eat = false;
            one_cvar.notify_one();
            drop(one_lock);
        }
    }

    pub fn three_eat(&self) {
        for _ in 0..self.num_bites {
            println!("Person three eats");
            eat();
        }
    }
}

fn eat() {
    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);
}

fn main() {
    // We have to use Arc to allow multiple references to the heap instance of people
    let people = Arc::new(DiningPeople::new(5));

    let mut handles = vec![];

    // spawn 3 threads to represent the 3 people eating
    // open a thread closure and create a copy (reference) of the Arc<DiningPeople> 
    // and give non-mutable ownership to the threads
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

    // {
    //     let people = Arc::clone(&people);
    //     let handle = thread::spawn(move || {
    //             people.three_eat();
    //         }
    //     );
    //     handles.push(handle);
    // }

    for handle in handles {
        handle.join().unwrap();
    }
}
