
// 模式匹配
// 模式：rust 的一种特殊语法，用于匹配一些类型结构
// 模式组成：
//  - 字面值；
//  - 解构的数组、enum、struct、tuple；
//  - 变量
//  - 通配符
//  - 占位符
//  - _ 可以用于忽略一些变量，.. 可以忽略剩余部分
//  - @ 可以用于绑定


fn main() {
    fn_example();
}

fn fn_example(){
    // match value {
    //     xxx => aaaa,
    //     _ => bbb
    // }
    let x = 5;
    match x {
        1|5 => println!("{}",1),
        2..=6 => println!("{}",2),
        7 => println!("{}",7),
        _ => println!("xxx")
    }

    let num = Some(4);
    match num {
        Some(x) if x > 5 => println!("{}",1),
        Some(x) => println!("{}",x),
        None => ()
    }


    // if let 用于替代只有一个匹配项的 match
    // 包括 else if 、else if let
    // 不会检查穷尽性

    // while let 满足条件就会一直循环
    
    // for 循环的变量
    
    // let 语句
    let (x,y,z) = (1,2,3);
    
    // 函数参数


    
}