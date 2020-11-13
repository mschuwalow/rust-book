#[cfg(test)]
mod tests {
    use std::{sync::mpsc, thread, time::Duration};

    #[test]
    fn sending_dat() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
            // burrowed after move
            // println!("val is {}", val);
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

    #[test]
    fn waiting() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
        for received in rx {
            println!("Got: {}", received);
        }
    }

    #[test]
    fn multiple_producer() {
        let (tx, rx) = mpsc::channel();
        let tx1 = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];
            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }
}