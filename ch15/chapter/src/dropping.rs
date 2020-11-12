struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_drop() {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    }

    #[test]
    fn test_drop() {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created.");
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }
}
