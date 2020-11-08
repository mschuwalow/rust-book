mod errors;
mod smart;

#[cfg(test)]
mod tests {
    use super::smart::*;

    #[test]
    fn guess_value() {
        let guess = Guess::new(10);
        assert_eq!(guess.value(), 10);
    }
}
