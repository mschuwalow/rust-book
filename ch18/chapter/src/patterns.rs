struct Point {
    x: i32,
    y: i32,
}

struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct D(i32);
impl Drop for D {
    fn drop(&mut self) {
        println!("dropped {}", self.0);
    }
}

enum Message2 {
    Hello { id: i32 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matching_literals() {
        let x = 1;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    #[test]
    fn matching_named() {
        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {:?}", y),
            _ => println!("Default case, x = {:?}", x),
        }
        println!("at the end: x = {:?}, y = {:?}", x, y);
    }

    #[test]
    fn multiple_patterns() {
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    #[test]
    fn matching_ranges() {
        let x = 5;
        match x {
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }
    }

    #[test]
    fn destructuring_structs() {
        let p = Point { x: 0, y: 7 };
        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);
    }

    #[test]
    fn destructuring_structs_short() {
        let p = Point { x: 0, y: 7 };
        let Point { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);
    }

    #[test]
    fn matching_structs() {
        let p = Point { x: 0, y: 7 };
        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }

    #[test]
    fn destructuring_enums() {
        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.")
            }
            Message::Move { x, y } => {
                println!(
                    "Move in the x direction {} and in the y direction {}",
                    x, y
                );
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!(
                "Change the color to red {}, green {}, and blue {}",
                r, g, b
            ),
        }
    }

    #[test]
    fn destructuring_references() {
        let points = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 5 },
            Point { x: 10, y: -3 },
        ];
        let sum_of_squares: i32 =
            points.iter().map(|&Point { x, y }| x * x + y * y).sum();
    }

    #[test]
    fn complex_patterns() {
        let ((feet, inches), Point { x, y }) =
            ((3, 10), Point { x: 3, y: -10 });
    }

    #[test]
    fn unused() {
        fn foo(_: i32, y: i32) {
            println!("This code only uses the y parameter: {}", y);
        }

        foo(3, 4);
    }

    #[test]
    fn nested_ignore() {
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);
        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }
        println!("setting is {:?}", setting_value);
    }

    #[test]
    fn ignoring_multiple_in_one_pattern() {
        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {}, {}, {}", first, third, fifth)
            }
        }
    }

    #[test]
    fn unused_variables() {
        let _x = 5;
        let y = 10;
    }

    #[test]
    fn underscore_is_not_moving() {
        let s = Some(String::from("Hello!"));
        if let Some(_) = s {
            println!("found a string");
        }
        println!("{:?}", s);
    }

    #[test]
    fn underscore_dropping() {
        #![allow(path_statements, unused_variables)]

        {
            D(0);
            let mut x = D(1);
            let _ = D(2);
            D(3);
            x = D(6);
            let y = D(4);
            let _ = D(5);
        }

        // {
        //     let var = D(0);
        //     let _ = var;
        //     var;
        //     D(1);
        // }
    }

    #[test]
    fn ignoring_remaining() {
        let origin = Point3 { x: 0, y: 0, z: 0 };
        match origin {
            Point3 { x, .. } => println!("x is {}", x),
        }
    }

    #[test]
    fn ignoring_middle() {
        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, .., last) => {
                println!("Some numbers: {}, {}", first, last);
            }
        }
    }

    #[test]
    fn ambigious_elipsis() {
        // let numbers = (2, 4, 8, 16, 32);
        // match numbers {
        //     (.., second, ..) => println!("Some numbers: {}", second),
        // }
    }

    #[test]
    fn creating_references() {
        let robot_name = Some(String::from("Bors"));
        match robot_name {
            Some(ref name) => println!("Found a name: {}", name),
            None => (),
        }
        println!("robot_name is: {:?}", robot_name);
    }

    #[test]
    fn creating_mutable_references() {
        let mut robot_name = Some(String::from("Bors"));
        match robot_name {
            Some(ref mut name) => *name = String::from("Another name"),
            None => (),
        }
        println!("robot_name is: {:?}", robot_name);
    }

    #[test]
    fn match_guards() {
        let num = Some(4);
        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }
    }

    #[test]
    fn match_guards_multiple_patterns() {
        let x = 4;
        let y = false;
        match x {
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
    }

    #[test]
    fn at_bindings() {
        let msg = Message2::Hello { id: 5 };
        match msg {
            Message2::Hello {
                id: id_variable @ 3...7,
            } => println!("Found an id in range: {}", id_variable),
            Message2::Hello { id: 10...12 } => {
                println!("Found an id in another range")
            }
            Message2::Hello { id } => println!("Found some other id: {}", id),
        }
    }
}
