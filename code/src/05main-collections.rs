/*
 * @Author: lxt
 * @Date: 2022-09-09 09:55:38
 * @LastEditors: [lxt]
 * @LastEditTime: 2022-09-09 11:44:52
 * @Description: // # 
 */

use std::collections::HashMap;

fn main (){
    // vector();
    // string();
    hash_map();
}

fn hash_map(){
    // 键值对存储数据，数据存储在 heap
    
    // 创建，创建指明类型或者 insert 指明
    let mut map = HashMap::new();
    map.insert(String::from("k"), 1);
    let teams = vec![String::from("blue"),String::from("yellow")];
    let scores = vec![10,50];
    let mut map :HashMap<_,_> = teams.iter().zip(scores.iter()).collect(); // 必须指明，但是类型可以推导
    // 对于 copy trait 值被复制到 hashmap，拥有所有权的数据，值移动，所有权移交
    let s1 = String::from("key");
    let mut map = HashMap::new();
    map.insert(s1, 11);
    // println!("{}",s1); // 报错

    // 访问
    let score = map.get("key");
    match score {
        Some(s)=>println!("{}",s),
        None => println!("x")
    }

    // 遍历
    for (k,v) in map {
        println!("{}：{}",k,v);
    }

    // 更新：如果已存在了可以选择覆盖、保留之前、合并
    let mut map = HashMap::new();
    map.insert(String::from("key"), 11);
    map.insert(String::from("key"), 111); // 覆盖
    map.entry(String::from("key")).or_insert(122); // 不存在则插入
    map.entry(String::from("val")).or_insert(122);
    let score = map.entry(String::from("val")).or_insert(10);
    *score += 100; // 手动处理
    

    println!("{:?}",map)
    
}

fn string(){
    // rust 中字符串只 String 和 &str，字符串是基于 byte 的集合

    // rust 核心语言层面只提供了一个字符串类型，字符串切片 str、&str
    // 字符串切片是对存储在其它地方、UTF-8字符串的引用；
    // 字符串字面值：存储在二进制文件中，也是字符串切片
    
    // string 类型来自标准库，不是核心语言层面
    // 可增长、可修改、可拥有
    // 标准库中还有 OsString、OsStr、CString、CStr，string 结尾可以获得所有权
    
    // 创建
    let mut s = String::new();
    s = "adddd".to_string();
    s = String::from("asdasd");
    
    // 更新
    s.push_str("bar"); // 末尾添加
    s.push('a'); // 末尾添加单个字符
    
    // 拼接
    s = s + "asdad"; // + 前面的是 String，后面的是 &str，第二个是字符串引用，rust 使用了解引用强制转换
    s = format!("{},-{}",s,s);
    
    // 访问，string 本质是一个 Vec<u8>
    s = String::from("Hello");
    println!("{}", s.len()); // 5 字节
    s = String::from("你好");
    println!("{}", s.len()); // 6 字节
    // 为了防止字节解析出错，rust 不允许使用下标访问
    // 在 rust 中字符串可以看做字节、标量值、字形簇（字母）
    // 字节
    for b in s.bytes()  {
        println!("{}",b);
    }
    // unicode 标量
    for c in s.chars()  {
        println!("{}",c);
    }

    // 切割
    let s1 = &s[0..3]; // 切片
    println!("{}",s1); // 需要按照边界切割得到 &str，否则报错
   
    
}

fn vector(){
    // 标准库提供，连续存放相同类型的值
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1,2,3,4];
    v.push(5);
    
    // 读取
    println!("{},{}",&v[0],v.get(3).expect("None"));

    // 所有权，报错原因：vector 是连续的，push 可能导致内存重新分配，原来的内存可能释放，不再有数据
    let mut v = vec![1,2,3,4]; // 可变
    let a = &v[0]; // 不可变借用
    // v.push(7); // 可变借用，报错
    println!("{}",a); // 不可变

    // 遍历，不可变
    for i in &v {
        print!("{}",i);
    }
    // 可变
    for i in &mut v {
        println!("{}",i);
    }

    // Vector + 枚举，用于存放不同类型数据
    enum SpreadsheetCell {
        Int(i32),
        Flot(f64),
        Text(String)
    }
    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Flot(1.2),
        SpreadsheetCell::Text(String::from("xcc"))
    ];

}








