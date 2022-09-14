
// closure

use std::thread;
use std::time::Duration;

// 闭包：
// 1.匿名函数；
// 2.闭包可以保存变量、作为函数参数、返回值；
// 3.可以从其定义的作用域获取值

// 所有闭包都实现了 Fn、FnMut、FnOnce 之一
// 获取所有权：FnOnce
// 可变借用：FnMut
// 不可变借用：Fn

// move 可以移动所有权到闭包内

fn main(){
    // closure1();
    closure2();
}

fn closure1(){
    let closure = |duration:u64,number:i32|->i32{
        thread::sleep(Duration::from_secs(duration));
        println!("{}",number);
        number
    };
    closure(1,2);
}

fn closure2(){
    struct Cacher<T> where T:Fn(u32)->u32{
        calc:T,
        value:Option<u32>
    }
    impl <T> Cacher <T> where T:Fn(u32)->u32{
        fn new(calc:T)->Cacher<T>{
            Cacher { calc, value: None }
        }
        fn value(&mut self,arg:u32)->u32{
            match self.value {
                Some(v) => v,
                None =>{
                    let v = (self.calc)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }
    let mut cacher = Cacher::new(|num|{
        num
    });
    println!("{}",cacher.value(32));
    println!("{}",cacher.value(1));
    println!("{}",cacher.value(2));
}

