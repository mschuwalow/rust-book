#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, Mutex},
        thread,
    };
    #[test]
    fn single_thread() {
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        println!("m = {:?}", m);
    }

    #[test]
    fn example() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Result: {}", *counter.lock().unwrap());
    }

    #[test]
    fn simple_example() {
        let counter = Arc::new(Mutex::new(0));
        let counter1 = Arc::clone(&counter);
        let counter2 = Arc::clone(&counter);
        let mut handles = vec![];

        let handle = thread::spawn(move || {
            let mut num = counter1.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
        let handle2 = thread::spawn(move || {
            let mut num2 = counter2.lock().unwrap();
            *num2 += 1;
        });
        handles.push(handle2);
    }
}
