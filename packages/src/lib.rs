mod front_of_house;

use front_of_house::hosting;

mod house_unit1 {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        // Absolute path  --> it won't be effect if current mod changed get differnt parent mod.
        // crate::front_of_house::hosting::add_to_waitlist(); //all module need to add into the absolute path
        crate::front_of_house::hosting::add_to_waitlist();
        //crate::front_of_house::hosting::seat_at_table(); //seat_at_table is private

        // Relative path
        super::front_of_house::hosting::add_to_waitlist(); //as long as front_of_house at the same level. it works

        // front_of_house::serving::take_order(); // serving is private

        hosting::add_to_waitlist(); //keyword use
    }
}
