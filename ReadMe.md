# Dining Philosophers (with a twist)

There is a situation where 3 Philosophers sit down at a table to share a single order of fries, represented by this struct with a single property that represents number of fries each person in the group will eat.

```rust
struct DiningPeople { 
    num_bites: i32,
}
```

There is an implementation  of these Philosophers eating with a constructor accepting `num_bites`, and then a function that represents each Philosopher eating.

```rust

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
```

The `eat()` function is just a function to represent some delay for whomever is eating to eat their fry.
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

Output (will be slightly different every time): 
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

The issue with the output is that there is no order to which Philosopher will get to eat first, nor any guarantee that a Philosopher will get to once before the others have eaten twice. 
Correct the `DiningPeople` implementation to guarantee the same order of philosophers eating every time, and guarantee no philosopher will get to eat twice before another has eaten once.

- Note: You are not allowed to change the code inside `main()` for how the `DiningPeople` struct is referenced, only change code inside `DiningPeople` impl/struct. 

