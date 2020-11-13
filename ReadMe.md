# Dining Philosophers (with a twist)

This problem describes a situation where 3 threads are running in a program and executing some functions, those threads do not have any order or sequence to how they are running and there needs to be a way to enforce order and guarantee that they will always run in that order. There also needs to be a way to make sure that when the threads run in order they don't ever run more than once before the next thread in the sequence gets to run.

## Problem Implementation

3 Philosophers sit down at a table to share a single order of food, represented by this struct with a single property that represents number of bites each person in the group will eat.

```rust
struct DiningPhilosophers { 
    num_bites: i32,
}
```

There is an implementation  of these Philosophers eating with a constructor accepting `num_bites`, and then a function that represents each Philosopher eating.

```rust

impl DiningPhilosophers {
    pub fn new(bites: i32) -> DiningPhilosophers {
        return DiningPhilosophers { 
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
```

The `eat()` function is just a function to represent some delay for whomever is eating.

```rust
fn eat() {
    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);
}
```

Then there is some code using this struct and implementation representing the Philosophers eating together. 
 - Rust Note: We have to use closures (for spawning threads) and `Arc` to allow multiple non-mutable references to a heap allocation, both of these are because of how Rust handles ownership and concurrency.

```rust
fn main() {
    // We have to use Arc to allow multiple references to the heap instance of people
    let people = Arc::new(DiningPhilosophers::new(5));

    let mut handles = vec![];

    // spawn 3 threads to represent the 3 people eating
    // open a thread closure and create a copy (reference) of the Arc<DiningPhilosophers> 
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
```

Output will be slightly different every time since no order is enforced: 
```
Person one eats
Person two eats
Person three eats
Person one eats
Person three eats
Person two eats
Person two eats
Person one eats
Person three eats
Person two eats
Person three eats
Person one eats
Person two eats
Person three eats
Person one eats
```

# The Problem

The issue with the output is that there is no order to which Philosopher will get to eat, nor any guarantee that a Philosopher will get to once before the others have eaten twice. 
Correct the `DiningPhilosophers` implementation to guarantee the same order of philosophers eating every time, and guarantee no philosopher will get to eat twice before another has eaten once.

- Note: You are not allowed to change the code inside `main()` for how the `DiningPhilosophers` struct is referenced, only change code inside `DiningPhilosophers` impl/struct. 

# Solution

In order to protect the threads and establish some sort of order give the `DiningPhilosophers` struct three new properties of type `Arc<Mutex<bool>, Condvar>`. Each of these properties represents a shared memory `Mutex` that can hold the boolean value specifying if a given philosopher can eat, as well as a Condvar to notify threads when the `Mutex` value has changed.

```rust
// 3 people sit down to eat form the same plate, make sure that they all get to eat in turn
struct DiningPhilosophers {
    // declare the number of bites each philosopher will take
    num_bites: i32,

    // declare locks that determine if a given philosopher can take a bite
    one_can_eat: Arc<(Mutex<bool>, Condvar)>,
    two_can_eat: Arc<(Mutex<bool>, Condvar)>,
    three_can_eat: Arc<(Mutex<bool>, Condvar)>,
}
```

Next, initialize the struct and prevent any philosopher from eating except the first one. This will guarantee that that Philosopher 1 will eat first every time.
```rust
pub fn new(bites: i32) -> DiningPhilosophers {
    DiningPhilosophers {
        // at the beginning only philosopher 1 can eat
        num_bites: bites,
        one_can_eat: Arc::new((Mutex::new(true), Condvar::new())),
        two_can_eat: Arc::new((Mutex::new(false), Condvar::new())),
        three_can_eat: Arc::new((Mutex::new(false), Condvar::new())),
    }
}
```

Then update every philosophers eating function implementation. We will create a cycle of philosopher 1 eating, then philosopher 2 eating once philosopher 1 is done, then philosopher 3 eating once philosopher 2 is done, then back to philosopher 1. 

The cycle should look like this
```
Person one eats
Person two eats
Person three eats
Person one eats
Person two eats
Person three eats
Person one eats
Person two eats
Person three eats
Person one eats
Person two eats
Person three eats
Person one eats
Person two eats
Person three eats
```


Update the code for each philosopher in the sequence, each of them are the same but the locks they acquire and update change based on what position of the cycle they are in.

```rust
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
        println!("Philosopher one eats");
        eat();

        // update the next philosopher's mutex value
        *two_eat = true;
        // update your value to not be able to eat
        *one_eat = false;
        // notify the next philosopher that they can eat
        two_cvar.notify_one();
    }
}
```

this will guarantee that all the philosophers take bites in order, and that none will get to eat until the other has eaten.

- Notes: this solution would have the potential for deadlock if we tweaked the problem, if a philosopher wanted to take a different number of bites then it would break our current cycle with this implementation. To address that we would need to add some kind of queue mechanism to it.
