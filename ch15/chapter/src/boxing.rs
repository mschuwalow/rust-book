use std::ops::Deref;
use List::{Cons, Nil};

fn create_box() {
    let b = Box::new(5);
    println!("b = {}", b);
}

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn make_list() -> List<i32> {
    Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pointers() {
        let x = 5;
        let y = &x;
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_box() {
        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_my_box() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_deref_coercion() {
        fn hello(name: &str) {
            println!("Hello, {}!", name);
        }
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
    }
}

/// Copy of Box<T>
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
