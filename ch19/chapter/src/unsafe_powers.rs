use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_pointers() {
        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
    }
    #[test]
    fn raw_memory() {
        let address = 0x012345usize;
        let r = address as *const i32;
    }

    #[test]
    fn dereference_pointers() {
        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }

    #[test]
    fn calling_unsafe_functions() {
        unsafe fn dangerous() {}
        unsafe {
            dangerous();
        }
    }

    #[test]
    fn test_split_at_mut() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];
        let (a, b) = r.split_at_mut(3);
        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    #[test]
    fn test_split_at_mut_1() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];
        let (a, b) = super::split_at_mut(r, 3);
        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    #[test]
    fn extern_functions() {
        extern "C" {
            fn abs(input: i32) -> i32;
        }
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

    #[test]
    fn use_static() {
        println!("name is: {}", HELLO_WORLD);
    }

    #[test]
    fn mutate_static() {
        add_to_count(3);
        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }
}
