
/*
 * @Author: lxt
 * @Date: 2022-09-07 13:12:47
 * @LastEditors: [lxt]
 * @LastEditTime: 2022-09-08 14:42:15
 * @Description: // # 基础部分
 */

use::rand::Rng;
use std::{io, cmp::Ordering};
fn main(){
    // guess();
    // variable();
    // variable1();
    // function(10);
    // println!("{}",function1());
    // condition();
    // loopfn();
    // string();

    // let str = String::from("hello world");
    // string2(str);
    // println!("{}",str);

    // let str = String::from("hello world");
    // let (s,len) = string3(str);
    // println!("{},{}",s,len);

    // let str = String::from("hello world");
    // let len = string4(&str);
    // println!("{},{}",str,len);

    // let mut str = String::from("hello world");
    // let len = string5(&mut str);
    // println!("{},{}",str,len);

    // let mut s = String::from("Hello World");
    // let index = first_world(&s);
    // s.clear(); // 此时字符串为空，index 失去原有的意义
    // println!("{}",index);

    // let mut s = String::from("Hello World");
    // let hello = &s[0..5]; // hello
    // let world = &s[6..11]; // world
    // let index = first_world1(&s);
    // // s.clear(); // 此时字符串为空，index 失去原有的意义
    // println!("{}",index);


    let st1 = String::from("Hello");
    let first = first_world2(&st1[..]);

    let st2 = "asdasd";
    let first2 = first_world2(st2);

}

fn first_world2 (s:&str)->&str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate()  {
        if item ==b' '{
            return &s[..i];
        }
    }
    &s[..]
}

fn first_world1 (s:&String)->&str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate()  {
        if item ==b' '{
            return &s[..i];
        }
    }
    &s[..]
}

fn first_world (s:&String)->usize{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate()  {
        if item ==b' '{
            return i;
        }
    }
    s.len()
}

fn string5(s:&mut String)->usize{
    s.push_str("asdasd");
    return s.len();
}

fn string4(s:&String)->usize{
    // s.push_str("asdasd");
    return s.len();
}

fn string3(s:String)->(String,usize){
    let len = s.len();
    (s,len)
}

fn string2(s:String){

}

fn string(){
    let mut s = String::from("Hello");
    // s.push_str(", world");

    // let s2 = s;
    let s2 = s.clone();
    println!("{}",s);
    
}

fn loopfn(){
    let mut counter = 0;
    loop {
        counter +=1;
        println!("hello!");
        if counter == 10 {
            break;
        }
    }
    counter = 7;
    while counter != 10 {
        counter+=1;
        println!("while")
    }

    let arr = ["str","str1"];
    for item in arr.iter() {
        println!("{}",item);
    }

    for number in (1..4).rev() {
        println!("{}",number);
    }
}

fn condition (){
    let number = 5;
    if number > 5 {
        println!(">5 is true")
    }else if number < 5{
        println!(">5 is false")
    }else {
        println!("=5")
    }
}


fn function1() -> i32{
    5
}

fn function(x1:u8){
    let y = {
        let z = 10;
        z+x1
    };
    println!("{}",y);
}

fn variable1(){
    let sum = 5 + 10;
    let difference = 95.5 - 4.5;
    let product = 4 * 30;
    let quotient = 57.6/32.2;
    let reminder = 54 % 5;
    let a = "xxx";

    let tuple:(i32,f64,u8) = (5000,4.6,20);
    println!("{},{},{}",tuple.0,tuple.1,tuple.2);
    let (x,y,z) = tuple;
    println!("{},{},{}",x,y,z);

    let arr:[i32;5] = [1,2,3,4,5];

    let x = 1;
    println!("{}", arr[x]);
    
}

fn variable(){
    let x = 5;
    // x = 10;
    println!("{}",x);

    let mut x1 = 5;
    let mut x1 = x1 + 10;
    x1 = 8;
    println!("{}",x1);

    const max:i32 = 1;
    println!("{}",max);
}


/**
 * 猜数字游戏
 */
fn guess (){
    // 本地线程空间，通过操作系统时间获取，范围是 0~100
    let num:u32 = rand::thread_rng().gen_range(0..100);
    println!("生成的数字是：{}",num);
    
    loop {
        // rust 所有变量默认是不可变的， 如果想改变需要加 mut
        // String::new 这种调用，类似调用静态方法
        let mut str = String::new();
        // &mut str 表示这个参数可变，引用类型传参
        // read_line 返回 Result，它有两个结果：OK/Err
        // Result 定义了一些方法，如果返回 Err，expect 中断程序，把传入的参数显示出来
        io::stdin().read_line(&mut str).expect("无法读取行");

        // let input_number:u32 = str.trim().parse().expect("输入的不是数字");
        // println!("你猜测的数是：{}",input_number);

        let input_number:u32 =match str.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("not a number");
                continue;
            }
        };
        println!("你猜测的数是：{}",input_number);

        match input_number.cmp(&num){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
    
}

