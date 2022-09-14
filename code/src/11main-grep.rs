


// grep 模拟

use std::env;
use std::error::Error;
use std::fs;
use std::process;


fn main(){
    // cargo run 1234 readme.txt
    // 第一个元素是当前程序二进制文件，第二个参数开始是参数
    let args : Vec<String> = env::args().collect();
    // println!("{:?}",ags);
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}",err); // 标准错误
        process::exit(1);
    });
    if let Err(e) = run(config){
        eprintln!("{}",e); // 标准错误
        process::exit(1);
    }
}
fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    println!("case_sensitive：{}",config.case_sensitive);
    let res = if !config.case_sensitive {
        search(&config.query,&contents)
    }else{
        search_case_insensitive(&config.query,&contents)
    };
    for line in res {
        println!("{}",line)
    }
    Ok(())
}
fn search(query:&str,content:&str) -> Vec<String>{
    let mut res = vec![];
    for line in content.lines() {
        if line.contains(query) {
            res.push(line.to_string());
        }
    }
    res
}
fn search_case_insensitive(query:&str,content:&str ) -> Vec<String>{
    let query = query.to_lowercase();
    let mut res = vec![];
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line.to_string());
        }
    }
    res
}

struct Config {
    query:String,
    filename:String,
    case_sensitive:bool
}

impl Config{
    fn new(args:&[String])->Result<Config, &'static str>{
        if args.len() < 3 {
           return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        println!("{},{}",query,filename);
        let  case_sensitive = env::var("CASE_SENSITIVE").is_err();
        Ok(Config{ query,filename,case_sensitive:case_sensitive})
    }
}