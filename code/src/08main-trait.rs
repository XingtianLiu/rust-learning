/*
 * @Author: lxt
 * @Date: 2022-09-09 13:26:41
 * @LastEditors: [lxt]
 * @LastEditTime: 2022-09-09 16:53:49
 * @Description: // # 
 */


use std::fmt::Display;
use std::fmt::Debug;
use code::Summary;
use code::Tweet;
use code::NewsArticle;

fn main(){
    trait_fn();
}

fn trait_fn(){
    // trait 用于描述特定类型具有哪些可以共享的功能，类似接口
    let mut tw = Tweet{
        username:String::from("Tom"),
        content:String::from("hahahaha"),
        reply:false,
        retweet:false
    };
    let mut news = NewsArticle{
        headline:String::from("DDD"),
        author:String::from("Tom"),
        location:String::from("aaa"),
    };
    print1(tw);
    print1(news);
}

// 简单写法
fn print1(item: impl Summary){
    println!("{}",item.summarize());
}
// 泛型
fn print2<T:Summary>(item: T){
    println!("{}",item.summarize());
}
// 类型需要同时实现了多个 trait
fn print3<T:Summary+Display>(item: T){
    println!("{}",item.summarize());
}
// where 简化
fn print4<T,U>(i1: T,i2:U) -> String
where 
    T:Summary+Display,
    U:Clone + Debug,
{
   String::from("cccc")
}

// impl trait 表示返回值，只能返回一种子类，不能用 if/else 判定




