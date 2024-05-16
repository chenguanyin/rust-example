mod front_of_house;
use crate::front_of_house::hosting;
use std::fmt::Display;

mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // 私有的，外界不允许访问
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}


pub fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();
    hosting::seat_at_table();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut breakfast = back_of_house::Breakfast::summer("Rye");
    breakfast.toast = String::from("Wheat");
    println!("I'd like {:#?} toast please", breakfast)
}
