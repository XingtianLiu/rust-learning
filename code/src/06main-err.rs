use core::panic;
use std::{fs::File, io::{ErrorKind, Error as IOError, Read}};



fn main(){
    // err1();
    // err2();
    guess_number();
}

pub struct Guess{
    val:i32,
}
impl Guess {
    pub fn new(n:i32) -> Guess{
        if n < 1 || n > 100 {
            panic!("must be between 1 and 100, got {}",n)
        }
        Guess{ val:n }
    }
    pub fn value(&self) -> i32{
        self.val
    }
}

fn guess_number(){
    let guess = "32";
    let guess = match guess.trim().parse() {
        Ok(n)=>n,
        Err(_) => return
    };
    let guess = Guess::new(guess);
    println!("{}",guess.val)
}

fn err2(){
    // 错误传播，返回 result
    fn read_username_from_file()->Result<String,IOError>{
        let f = File::open("io.txt");
        let mut f = match f{
            Ok(file)=>file,
            Err(e) => return Err(e)
        };
        let mut s = String::new();
        return match f.read_to_string(&mut s){
            Ok(_) =>Ok(s),
            Err(e)=>Err(e)
        };
    }
    let res = read_username_from_file();

    // 快捷方式
    fn read_username_from_file1()->Result<String,IOError>{
        let mut f = File::open("io.txt")?; // 如果是 OK 继续执行，如果是 Err 返回
        let mut s = String::new();
        f.read_to_string(&mut s)?;// 同上
        Ok(s)
    }
    fn read_username_from_file2()->Result<String,IOError>{
        let mut s = String::new();
        File::open("io.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
    let res = read_username_from_file1();

    // ? 会被 std::convert::From 隐式处理
    // ? 接收的错误类型会被转化为当前函数定义的错误类型
    // errA -> errB  A 要实现 对应的方法

}

fn err1(){
    // 错误类型：可恢复错误（比如文件未找到，可以再次尝试）、不可恢复（比如索引超出范围）
    // 可恢复：Result；不可恢复：panic!

    // panic 发生时：
    // 可以展开调用栈，往回走，每遇到数据就清理掉
    // 也可以立即终止调用栈，直接停止程序，OS 清理内存，这样二进制文件就更小了
    //  修改 cargo.toml，增加 
    //      [profile.release]
    //      panic = 'abort'

    // 可以设置环境变量 RUST_BACKTRACE=full
    // panic!("crash and bin");
    
    // let v = vec![1,2,3];
    // println!("{}", v[20]);


    // result 枚举，不存在文件先创建
    let f = File::open("aa.txt");
    let f = match f {
        Ok(file)=>file,
        // Err(err)=> {
        //     panic!("{:?}",err);
        // }
        Err(err)=> match err.kind(){
            ErrorKind::NotFound => match File::create("aa.txt"){
                Ok(fc)=>fc,
                Err(e) =>panic!("error creating file：{:?}",e)
            },
            other_error => panic!("error opening the file {:?}",other_error)
        }
    };
    // 改良版：unwrap_or_else
    let f = File::open("aa.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("aa.txt").unwrap_or_else(|e|{
                panic!("error creating file：{:?}",e);
            })
        } else {
            panic!("error opening the file {:?}",error)
        }
    });
    // unwrap：match 表达式的快捷方法，返回 Result 中 OK 里面的值或者调用 panic!
    let f = File::open("aa.txt").unwrap();
    // expect：和 unwrap 类似，可以指定 panic! 信息，用于自定义错误类型
    File::open("abc.txt").expect("不存在");  

    
}