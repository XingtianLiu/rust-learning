/*
 * @Author: lxt
 * @Date: 2022-09-14 14:27:23
 * @LastEditors: [lxt]
 * @LastEditTime: 2022-09-15 11:23:33
 * @Description: // # 
 */
use std::env;

fn main() {
    // iterator_sum();
    // iterator_map();
    // iterator_closure();
    // iterator_1();
    iterator_2();
}

fn iterator_sum(){
    let v1 = vec![1,2,3,4,5,6];
    // sum 会消耗迭代器
    let total:i32 = v1.iter().sum();
    println!("{}",total);
}

fn iterator_map() {
    let v1 = vec![1,2,3,4,5,6];
    let v2:Vec<i32> = v1.iter().map(|x|x*10).collect(); // 必须调用消耗迭代器的方法，否则什么都不会做，所有需要调用 collect
    println!("{:?}",v2);
}

fn iterator_closure(){
    // 闭包捕获环境
    struct Shoe{
        size:i32,
        style:String
    }
    fn shoes_in_my_size(shoes:Vec<Shoe>,shoe_size:i32)->Vec<Shoe>{
        // into_iter 有所有权
        shoes.into_iter().filter(|x|x.size>shoe_size).collect()
    }
    let shoes = vec![
        Shoe{size:32,style:String::from("A")},
        Shoe{size:12,style:String::from("B")},
        Shoe{size:18,style:String::from("C")},
        Shoe{size:30,style:String::from("D")},
    ];
    let shoes = shoes_in_my_size(shoes, 20);
    for shoe in shoes  {
        println!("{}:{}",shoe.size,shoe.style);
    }
}

fn iterator_1(){
    struct  Counter{
        count:i32
    }

    impl Counter {
        fn new()->Counter{
            Counter { count: 0 }
        }
    }
    impl Iterator for Counter{
        type Item = i32;
        fn next(&mut self)->Option<Self::Item>{
            if self.count < 5 {
                self.count+=1;
                Some(self.count)
            }else{
                None
            }
        }
    }

    let mut counter = Counter::new();
    // println!("{}",counter.next().expect("aaa"));
    // println!("{}",counter.next().expect("aaa"));
    // println!("{}",counter.next().expect("aaa"));
    // println!("{}",counter.next().expect("aaa"));
    // println!("{}",counter.next().expect("aaa"));
    // println!("{}",counter.next().expect("aaa"));

    let  total:i32 = Counter::new()
    .zip( Counter::new().skip(1))
    .map(|(a,b)|{
        a*b
    }).filter(|x| x%3==0)
    .sum();
    // [1,2,3,4] [2,3,4,5] 相乘，能被3整除的数求和
    println!("{}",total)
}


fn iterator_2(){
    let config = Config::new(env::args());
    struct Config {
        query:String,
        filename:String
    }
    impl Config{
        fn new(mut args: env::Args)->Result<Config, &'static str>{
            if args.len() < 3 {
                return Err("not enough arguments");
            }
            args.next();
            // 所有权改进
            // let query = args[1].clone();
            // let filename = args[2].clone();
            let query = args.next().expect("");
            let filename = args.next().expect("");
            println!("{},{}",query,filename);
            Ok(Config{ query,filename})
        }
    }
}
