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
fn deliver_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // not modifyable after instantiation
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"), 
            }
        }
    }
    pub enum Appetizer { // enums are default public
        Soup,
        Salad, // both accessible, since listed as public
    }
}

// pub fn eat_at_restauarant() {
//     crate::front_of_house::hosting::add_to_waitlist(); //absolute path
//     front_of_house::hosting::add_to_waitlist(); //relative path

// }

// OR use the use keyword:

use crate::front_of_house::hosting; // can name using "as" keyword, can modify to be public
//alternative 3: use crate::front_of_house::hosting:add_to_waitlist;

pub fn eat_at_restauarant() {
    hosting::add_to_waitlist();
    //alternative 3: add_to_waitlist();
}