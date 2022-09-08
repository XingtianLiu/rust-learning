/*
 * @Author: lxt
 * @Date: 2022-09-07 13:12:47
 * @LastEditors: [lxt]
 * @LastEditTime: 2022-09-08 16:06:20
 * @Description: // # 
 */

use std::fmt::Debug;

struct User{
    name:String,
    email:String,
    age:u8,
    active:bool,
}

#[derive(Debug)]
struct  Rectamgle {
    width:u32,
    height:u32,
}

impl Rectamgle {
    fn get_area(&self)->u32{
        self.width * self.height
    }
    fn can_hold(&self,other:&Rectamgle) -> bool{
        self.width > other.width && self.height > other.height
    }
    fn createSquare(size:u32) ->Rectamgle{
        Rectamgle { width: size, height: size }
    }
}


fn main(){
    // 创建
    let user = get_user("xm",12);
    // 更新语法
    let user2 = User{
        email:String::from("bc@qq.com"),
        ..user
    };
    println!("{},{}",user2.email,user2.name);
    // tuple struct
    struct Color(u8,u8,u8);
    let black = Color(0,0,0);

    // 空结构体 {}：用于实现 trait
    
    // 例子
    let rect = Rectamgle{
        width:30,
        height:60
    };
    println!("{}", area(&rect)); // & 借用，所有权不会改变
    println!("{:#?}",rect);

    println!("area {}",rect.get_area());

    // 第一个部署 self 的函数叫关联函数，可以使用 :: 调用
    let rect1 = Rectamgle::createSquare(20);
    println!("{}", rect.can_hold(&rect1))
    
}

fn area(rect:&Rectamgle)->u32{
    rect.width * rect.height
}

fn get_user(name:&str,age:u8)->User{
    // 不允许声明一部分字段可变
    let mut user = User {
        name:String::from(name),
        email:String::from(""),
        age,
        active:false
    };
    user.email = String::from("abc@163.com");
    return user;
}

