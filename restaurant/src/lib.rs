use std::io::{self, Write};

mod front_of_house;

pub use crate::front_of_house::hosting;

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}




pub mod customer {
    use super::back_of_house;
    use super::front_of_house::hosting;     

    pub fn eat_at_restaurant() {

        let mut meal: back_of_house::Breakfast = back_of_house::Breakfast::summer("Wheat");

        meal.toast = String::from("Wheat");

        println!("I'd like {} toast please", meal.toast);

        // won't compile because accessing private field, all fields are private by default
        // meal.seasonal_fruit = String::from("mango, markets loool");

        let order1: back_of_house::Appetizer = back_of_house::Appetizer::Soup;
        let order2: back_of_house::Appetizer = back_of_house::Appetizer::Salad;

        hosting::add_to_waitlist();


    }
}
fn deliver_order() {}




























