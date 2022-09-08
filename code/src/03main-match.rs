


enum IP_TYPE{
    V4,
    V6
}

struct IpAddr{
    kind:IP_TYPE,
    addr:String
}

enum IP_ADDR{
    v4(u8,u8,u8,u8),
    v6(String)
}

impl IP_ADDR{
    // 可以定义函数
    fn call(&self){

    }
}

enum City{
    HZ,
    SH
}
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(City),
    AAA,
    BBB,
    CCC
}
fn main(){
    // 枚举
    let v4 = IP_TYPE::V4;
    let v6 = IP_TYPE::V6;

    let ip = IpAddr{
        kind:v4,
        addr:String::from("192.168.0.0")
    };
    // 枚举加对应的数据
    let ip2 = IP_ADDR::v4(191, 168, 0, 1);
    ip2.call();

    // option 枚举：可能存在，null 在很多语言中表示空，如果直接调用方法可能会报错
    // rust 没有 Null 类型，使用 option<T> 替代 enum Option<T>{ Some(T), None }
    // option、Some、None 被预导入
    let num = Some(0);
    let str = Some("xxx");
    

    // match，允许一个值和一系列模式匹配，执行对应的代码
    // match 需要匹配所有可能结果，不处理的可以使用 _代替
    let num = match_fn(Coin::Dime);
    let num = match_fn(Coin::Quarter(City::HZ));

    // if let 只处理一种匹配，忽略其他
    if_let(Coin::AAA);
    
}

fn if_let(coin:Coin){
    if let Coin::AAA = coin {
        println!("AAA");
    };
}

fn match_fn(coin:Coin) ->u8{
    match coin {
        Coin::Penny =>1,
        Coin::Nickel => 2,
        Coin::Dime =>3,
        Coin::Quarter(city) =>{
            4
        },
        _ => 0
    }
}




