#![feature(generic_associated_types)]
#![feature(specialization)]
mod advanced_lifetimes;
mod advanced_traits;
mod advanced_types;
mod hkt;
mod hkt_1;
mod hkt_2;
mod hkt_3;
mod hkt_4;
mod hkt_5;
mod unsafe_powers;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
