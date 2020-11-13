use std::sync::{Arc, Condvar, Mutex};
use std::{thread, time};

// 3 people sit down to eat form the same plate, make sure that they all get to eat in turn
struct DiningPhilosophers {
    // declare the number of bites each philosopher will take
    pub num_bites: i32,

    // declare locks that determine if a given philosopher can take a bite
    one_can_eat: Arc<(Mutex<bool>, Condvar)>,
    two_can_eat: Arc<(Mutex<bool>, Condvar)>,
    three_can_eat: Arc<(Mutex<bool>, Condvar)>,
}

impl DiningPhilosophers {
    pub fn new(bites: i32) -> DiningPhilosophers {
        DiningPhilosophers {
            // at the beginning only philosopher 1 can eat
            num_bites: bites,
            one_can_eat: Arc::new((Mutex::new(true), Condvar::new())),
            two_can_eat: Arc::new((Mutex::new(false), Condvar::new())),
            three_can_eat: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }

    pub fn philosopher_one_eats(&self) {
        // loop for every bite that will be taken
        for _ in 0..self.num_bites {
            // acquire the lock, guard and condvar for this philosopher
            let (one_lock, one_cvar) = &*self.one_can_eat;
            let mut one_eat = one_lock.lock().unwrap();

            // determine if this philosopher can eat, if not wait for the condvar to change
            if !*one_eat {
                one_eat = one_cvar.wait(one_eat).unwrap();
            }

            // acquire the lock for the next philosopher in sequence to eat
            let (two_lock, two_cvar) = &*self.two_can_eat;
            let mut two_eat = two_lock.lock().unwrap();
            
            // this philosopher eats
            println!("Person one eats");
            eat();

            // update the next philosopher's mutex value
            *two_eat = true;
            // update your value to not be able to eat
            *one_eat = false;
            // notify the next philosopher that they can eat
            two_cvar.notify_one();
        }
    }

    pub fn philosopher_two_eats(&self) {
        for _ in 0..self.num_bites {
            // acquire the lock, guard and condvar for this philosopher
            let (two_lock, two_cvar) = &*self.two_can_eat;
            let mut two_eat = two_lock.lock().unwrap();

            // determine if this philosopher can eat, if not wait for the condvar to change
            if !*two_eat {
                two_eat = two_cvar.wait(two_eat).unwrap();
            }

            // acquire the lock for the next philosopher in sequence to eat
            let (three_lock, three_cvar) = &*self.three_can_eat;
            let mut three_eat = three_lock.lock().unwrap();

            // this philosopher eats
            println!("Person two eats");
            eat();

            // update the next philosopher's mutex value
            *three_eat = true;
            // update your value to not be able to eat
            *two_eat = false;
            // notify the next philosopher that they can eat
            three_cvar.notify_one();
        }
    }

    pub fn three_eat(&self) {
        for _ in 0..self.num_bites {
            // acquire the lock, guard and condvar for this philosopher
            let (three_lock, three_cvar) = &*self.three_can_eat;
            let mut three_eat = three_lock.lock().unwrap();

            // determine if this philosopher can eat, if not wait for the condvar to change
            if !*three_eat {
                three_eat = three_cvar.wait(three_eat).unwrap();
            }

            // acquire the lock for the next philosopher in sequence to eat
            let (one_lock, one_cvar) = &*self.one_can_eat;
            let mut one_eat = one_lock.lock().unwrap();

            // this philosopher eats
            println!("Person three eats");
            eat();

            // update the next philosopher's mutex value
            *one_eat = true;
            // update your value to not be able to eat
            *three_eat = false;
            // notify the next philosopher that they can eat
            one_cvar.notify_one();
        }
    }
}

fn eat() {
    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);
}

fn main() {
    // We have to use Arc to allow multiple references to the heap instance of people
    let people = Arc::new(DiningPhilosophers::new(5));

    let mut handles = vec![];

    // spawn 3 threads to represent the 3 people eating
    // open a thread closure and create a copy (reference) of the Arc<DiningPeople>
    // and give non-mutable ownership to the threads
    {
        let people = Arc::clone(&people);
        let handle = thread::spawn(move || {
            people.philosopher_one_eats();
        });
        handles.push(handle);
    }

    {
        let people = Arc::clone(&people);
        let handle = thread::spawn(move || {
            people.philosopher_two_eats();
        });
        handles.push(handle);
    }

    {
        let people = Arc::clone(&people);
        let handle = thread::spawn(move || {
            people.three_eat();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
