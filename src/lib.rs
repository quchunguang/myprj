mod front_of_house;

use front_of_house::hosting;

pub fn fun() {
    hosting::add_to_wait_list();
    front_of_house::hosting::add_to_wait_list();
    crate::front_of_house::hosting::add_to_wait_list();
}