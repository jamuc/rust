mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {
            
        }

        fn seat_at_table() {
            
        }
    } /* hosting */

    mod serving {
        fn take_order() {
            
        }

        fn serve_order() {
            
        }

        fn take_payment() {
            
        }
    } /* serving */
} /* front_of_house */

mod back_of_house {
    // Struct is public
    pub struct Breakfast {
        // only toast is public
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast  {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_reastaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    //change our mind on what kind of bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // This will not work because seasonal fruit is private
    // meal.seasonal_fruit = String::from("blueberries");
}
