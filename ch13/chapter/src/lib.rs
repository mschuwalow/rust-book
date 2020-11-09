use std::{collections::HashMap, hash::Hash, thread, time::Duration};

mod iterators;

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result =
        Cacher::new(|num| simulated_expensive_calculation(num));

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

struct Cacher<A, R, T>
where
    A: Hash + Eq + Copy,
    R: Copy,
    T: Fn(A) -> R,
{
    calculation: T,
    values:      HashMap<A, R>,
}

impl<A, R, T> Cacher<A, R, T>
where
    A: Hash + Eq + Copy,
    R: Copy,
    T: Fn(A) -> R,
{
    fn new(calculation: T) -> Cacher<A, R, T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: A) -> R {
        match self.values.get(&arg) {
            Some(r) => *r,
            None => {
                let r = (self.calculation)(arg);
                self.values.insert(arg, r);
                r
            }
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);
        assert_eq!(v2, 2);
    }

    #[test]
    fn test_move_closure() {
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;
        // println!("can't use x here: {:?}", x);
        let y = vec![1, 2, 3];
        assert!(equal_to_x(y));
    }
}
