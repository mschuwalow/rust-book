mod parsing {
    struct Context<'s>(&'s str);
    struct Parser<'c, 's: 'c> {
        context: &'c Context<'s>,
    }

    impl<'c, 's> Parser<'c, 's> {
        fn parse(&self) -> Result<(), &'s str> {
            Err(&self.context.0[1..])
        }
    }

    fn parse_context(context: Context) -> Result<(), &str> {
        Parser { context: &context }.parse()
    }
}

struct Ref<'a, T: 'a>(&'a T);
struct StaticRef<T: 'static>(&'static T);

trait Red {}
struct Ball<'a> {
    diameter: &'a i32,
}
impl<'a> Red for Ball<'a> {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trait_objects_1() {
        let num = 5;
        let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
    }
    #[test]
    fn trait_objects_2() {
        fn foo<'a>(diameter: &'a i32) -> Box<dyn Red + 'a> {
            Box::new(Ball { diameter })
        }
    }
}
