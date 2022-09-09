/*
 * @Author: lxt
 * @Date: 2022-09-09 16:05:07
 * @LastEditors: [lxt]
 * @LastEditTime: 2022-09-09 16:53:33
 * @Description: // # 
 */

use std::fmt::Display;

pub trait Summary{
    // fn summarize(&self) -> String; // 抽象方法
    // 默认实现
    fn summarize(&self) -> String{ 
        // 这里面可以调用抽象方法
        String::from("none")
    }
}
pub struct NewsArticle {
    pub headline:String,
    pub location:String,
    pub author:String,
}
impl Summary for NewsArticle{
   
}
pub struct  Tweet{
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool,
}
impl Summary for Tweet{
    fn summarize(&self) -> String {
       format!("{}: {}",self.username,self.content)
    }
}


pub struct Pair<T>{
    x:T,
    y:T
}

impl <T> Pair<T>{
    fn new(x:T,y:T) -> Self{
        Self{
            x,y
        }
    }
}
impl <T:Display+PartialOrd> Pair<T>{
    fn cmp_display(&self){
        if self.x >= self.y {
            println!("{}",self.x)
        }
    }
}
