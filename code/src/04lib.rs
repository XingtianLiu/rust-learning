/*
 * @Author: lxt
 * @Date: 2022-09-09 08:53:15
 * @LastEditors: [lxt]
 * @LastEditTime: 2022-09-09 09:40:15
 * @Description: // # 
 */
mod front_of_house{
    // 默认私有，pub 标记为公共的
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}

pub fn eat(){
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}


// 父模块不能访问子模块的私有条目
// 子模块可以使用祖先模块条目
fn server_order(){

}

mod back_of_house {
    fn fix_incorrect_order(){
        cook_order();
        // super 当前模块平级的 server_order 方法
        super::server_order();
        // 相当于
        crate::server_order();
    }
    fn cook_order(){}
}


// 同一个 rs 在同一个 crate 中，crate 相当于当前 rs 的根模块
// super 是当前模块的指代



// 公共结构体，内部字段需要加 pub
mod back_of_house1{
    pub struct  Breakfast{
        pub toast:String,
        seasonal_fruit:String
    }
    impl Breakfast{
        pub fn summer(toast:&str) -> Breakfast{
            Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peaches") }
        }
    }
}
pub fn eat_at_restaurant(){
    let mut meal = back_of_house1::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("{}",meal.toast)
}


// 公共枚举，内部元素不需要加 pub
mod back_of_house2{
    pub enum Appetizer {
        Soup,
        Salad
    }
}


// use
mod front_of_house1{
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}
// 函数一般引用到上级模块，不引用到方法，结构体一般引用到本身
use crate::front_of_house1::hosting;
pub fn eat_at_restaurant1(){
    hosting::add_to_waitlist();
}
// use + as
use std::io::Result as IOResult;
// pub use，暴露引用的包
pub mod front_of_house2{
    pub mod hosting{
        pub fn add_to_waitlist(){
            println!("Hello")
        }
    }
}
pub use crate::front_of_house2 as House;


