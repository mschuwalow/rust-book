fn largest_i32(list: &[i32]) -> &i32 {
    // let mut largest = list[0];

    // for &item in list.iter() {
    //     if item > largest {
    //         largest = item;
    //     }
    // }
    // largest
    largest(list)
}

fn largest_char(list: &[char]) -> &char {
    // let mut largest = list[0];

    // for &item in list.iter() {
    //     if item > largest {
    //         largest = item;
    //     }
    // }
    // largest
    largest(list)
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for item in list.iter() {
        // this will be compared using the actual data
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn points() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
