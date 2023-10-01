pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod front_of_house { // defines module
    pub mod hosting { //submodule
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
pub fn eat_at_restauarant() {
    crate::front_of_house::hosting::add_to_waitlist(); //absolute path
    front_of_house::hosting::add_to_waitlist(); //relative path

}