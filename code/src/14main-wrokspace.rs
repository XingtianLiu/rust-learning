// use evan_toy::{self, kinds::PrimaryColor, Utils::mix};

// pub use
use evan_toy::{PrimaryColor,mix};

fn main(){
    // println!("{}",evan_toy::add_one(1));

    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red,yellow);
}
