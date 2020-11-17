mod never {

    fn diverge() -> ! {
        loop {}
    }
}

mod sized {
    // may or may not be sized
    fn generic<T: ?Sized>(t: &T) {
        // --snip--
    }
}

mod function_pointers {
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}
