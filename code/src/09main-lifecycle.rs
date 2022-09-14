/*
 * @Author: lxt
 * @Date: 2022-09-09 16:53:43
 * @LastEditors: [lxt]
 * @LastEditTime: 2022-09-14 11:31:39
 * @Description: // # 
 */


fn main(){
    // life1();
    // life2();
    // life3();
    // life4();
    // life5();
    // life6();
    life7();
    
}

// 生命周期就是让引用保持有效的作用域
// 大部分情况下是隐式的、可被推断的，当引用的生命周期可能被不同方式关联时可以手动标记
// 生命周期的主要目标：避免指针悬垂

fn life1(){
    let x = 5;
    let r;
    {
        // let x = 5;
        r = &x; // 报错，因为 x 的生命周期没有 r 长
        // rust 使用借用检查器判断是否合法
        // 修改：需要保证 x 的生命周期不小于 r
    }
    println!("{}",r)
}

// 参数和泛型的生命周期
fn life2(){
    let str1 = String::from("aaa");
    let str2 = "xadd";
    let result = longest(str1.as_str(),str2);
    println!("{}",result);
    // 不知道返回值的生命周期和 x 有关还是和 y 有关
    // fn longest(x:&str,y:&str)->&str{
    //     if x.len() > y.len() {
    //         x
    //     }else{
    //         y
    //     }
    // }
    
    // 生命周期的标注只用于描述生命周期的关系，不影响生命周期的长短
    // 指定了泛型生命周期参数，函数可以接收任何带有生命周期的引用
    // 生命周期的标注使用 '+小写字母，通常很短
    // x、y、返回值的生命周期不能短于 'a
    fn longest<'a>(x: &'a str,y:  &'a str)-> &'a str{
        if x.len() > y.len() {
            x
        }else{
            y
        }
    }
}

fn life3(){
    // 返回引用的时候，返回类型的生命周期必须和其中一个引用相匹配
    // 如果返回类型没有指向任何参数，它就只能引用函数内部的值，这就是悬垂引用

    let str1 = String::from("aaa");
    let str2 = "xadd";
    let result = longest(str1.as_str(),str2);
    println!("{}",result);

    // fn longest<'a>(x: &'a str,y:  &'a str)-> &'a str{
    //     let res = String::from("aaaaa");
    //     res.as_str() // 离开 longest 就回收，但是 result 引用了该变量
    // }

    fn longest<'a>(x: &'a str,y:  &'a str)-> String{
        let res = String::from("aaaaa");
        res // 不返回引用，移交所有权
    }
}

fn life4(){
    // 结构体生命周期标注
    // 结构体类型也可以是引用类型，需要添加生命周期标注
    struct A<'a>{
        part : & 'a str
    }
    let model = String::from("aaaa");
    let a = A{
        part:&model
    };
}

fn life5(){
    // 生命周期的省略
    // 1.每个引用类型的参数都有自己的生命周期（一个参数就有一个生命周期，以此类推）；
    // 2.如果只有一个输入生命周期，那么它的生命周期会被赋给所有的输出生命周期；
    // 3.如果有多个输入生命周期参数，其中有一个是 &self/&mut self，self 的生命周期会被赋给所有输出参数；
    // 如果以上三条都不满足，也没声明生命周期，就报错
}

fn life6(){
    // 方法中的生命周期标注
    // 声明在 impl 后，在结构体后使用
    struct A<'a>{
        part : & 'a str
    }
    impl <'a> A<'a>{
        fn level (&self) ->i32{
            4
        }
    }
}

fn life7(){
    // 'static 静态生命周期，持续在整个程序的执行时间
    // 例如：字符串的字面值
    let str: &'static str = "asdasd";
    
  
}