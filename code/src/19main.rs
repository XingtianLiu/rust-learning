use std::{ops::Add, fmt::Display};

use evan_toy::HelloMacro;

fn main(){
    // fn_unsafe();
    // advanced_trait();
    // test_newtype();
    // test_advancedfn();
    test_macro();
}

fn fn_unsafe(){
    // 静态分析是非常保守的，使用 unsafe rust 表明程序员知道这些不安全情况
    // 另外计算机硬件本来就是不安全的，rust 需要能够继续底层系统编程
    //  unsafe 可以执行四个动作：
    //      - 解引用原始指针（*mut T、*const）
    //      - 调用 unsafe 方法
    //      - 访问、修改可变的静态变量；
    //      - 实现 unsafe trait
    
    // unsafe 没有关闭代码检查，错误必须留在 unsafe 中，尽量隔离 unsafe 代码，最好封装在抽象中，提供安全的 api
    
    let mut num = 5;
    let n1 = &num as *const i32;
    let n2 = &mut num as *mut i32;

    let addres = 0x012345usize;
    let n = addres as * const i32;
    unsafe{
        // 可以在外面创建，只能在里面进行解引用
        // 原始指针：与 C 语言交互；构建检查器无法理解的安全抽象；
        println!("{}",*n1);
        println!("{}",*n2);
        // println!("{}",*n); // 报错
    }

    // 调用 unsafe 方法
    unsafe fn dangerous(){}
    unsafe{
        dangerous();
    }

    // 创建 unsafe 安全抽象
    // fn split_at_mut(slice:&mut [i32],mid:usize) -> (&mut [i32],&mut [i32]){
    //     let len = slice.len();
    //     assert!(mid <=  len);
    //     (&mut slice[..mid],&mut  slice[mid..])
    // }
    fn split_at_mut(slice:&mut [i32],mid:usize) -> (&mut [i32],&mut [i32]){
        let len = slice.len();
        let ptr = slice.as_mut_ptr();
        assert!(mid <=  len);
        unsafe{
            (
                // 从 ptr 开始，创建 mid  长度切片
                std::slice::from_raw_parts_mut(ptr, mid),
                std::slice::from_raw_parts_mut(ptr.add(mid), len - mid)
            )
        }
    }
    
    // extern 调用外部代码，"C" 遵循 C 语言的 API
    extern "C"{
        // extern 声明的都是不安全的
        fn abs(input:i32)->i32;
    }
    unsafe{
        println!("{}",abs(-3));
    }
    // 其它语言调用 rust
    // 在 fn 前面加 extern 关键字，指定 ABI、添加 #[no_mangle] 注解，避免编译时改变名称
    #[no_mangle]
    pub extern "C" fn call_from_c(){
        println!("xxxx");
    }

    // 访问、修改全局变量，多线程可能出错
    static mut COUNTER:i32 = 0;
    fn add_to_count(inc:i32){
        unsafe{
            COUNTER += inc
        }
    }
    add_to_count(3);
    unsafe{
        println!("{}",COUNTER)
    }

    // unsafe trait，某个 trait 存在至少一个方法拥有编译器无法校验的不安全因素时，这个 trait 不安全
    unsafe trait Foo{
    }
    unsafe impl Foo for i32{
    }
    
}

fn advanced_trait(){
    // 关联类型：无需标注类型，需要指定具体类型，无法多次实现；
    // 泛型：每次实现 trait 时需要标注类型，可以为一个类型都从实现某个 trait（指定不同泛型类型）
    pub trait Iterator{
        type Item; // 关联类型
        fn next(&mut self) -> Option<Self::Item>;
    }
    struct  Counter{}
    impl Iterator for Counter{
        type Item  = i32;
        fn next(&mut self) -> Option<Self::Item>{
            None
        }
    }
    // 重载 + 运算符
    #[derive(Debug,PartialEq)]
    struct  Point{
        x:i32,
        y:i32
    }
    impl Add for Point{
        type Output = Point;
        fn add(self, rhs: Point) -> Point {
            Point { x: self.x + rhs.x, y: self.y + rhs.y}
        }
    }
    assert_eq!(Point{x:1,y:1} + Point{x:2,y:2},Point{x:3,y:3});

    // 调用同名方法
    struct Human{}
    trait Pilot{
        fn fly(&self);
        fn walk();
    }
    impl Pilot for Human {
        fn fly(&self) {
           println!("pilot") 
        }
        fn walk() {
            println!("p walk");
        }
    }
    impl  Human {
       fn fly(&self){
        println!("human")
       }
       fn walk() {
        println!("h walk");
       }
    }
    let human = Human{};
    human.fly();
    Pilot::fly(&human);

    Human::walk();
    <Human as Pilot>::walk();

    // trait 依赖其他 trait，要求需要实现 Display trait
    trait OutlinePrint: Display{
        fn print(&self){
            println!("{}",self.to_string());
        }
    }
    // newtype 可以为非本地包实现外部 trait
    // 为 Vec 实现 Display trait
    struct Wrapper(Vec<String>);
    impl Display for Wrapper{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"[{}]",self.0.join(", "))
        }
    }
}

fn test_newtype(){
    // 类型的高级应用
    // newtype 可以
    //      用来静态的保证各种值之间不会混淆，并表明值的单位；
    //      为类型的细节提供抽象能力
    //      通过轻量级的封装隐藏内部细节
    
    // 类型别名
    type Km = i32;
    type Result1<T> = Result<T,std::io::Error>;
    
    // size trait 泛型会被默认加上 Sized
    // fn generic<T>(t:T)
    // 会被转换为 fn generic<T:Sized>(t:T)
    // 默认情况下，泛型只能用于编译时已知大小的类型，可以通过特殊语法解除这一限制
    // fn generic<T:?Sized>(t:&T)
}

fn test_advancedfn(){
    // 函数指针
    fn add_one(i:i32)->i32{
        i+1
    }
    fn do_twice(f:fn(i32)->i32,arg:i32)->i32{
        f(arg) + f(arg)
    }
    do_twice(add_one, 5);
    // 函数指针实现了 Fn、FnMut、FnOnce 三种闭包，因此总是可以把函数指针用作参数传递给另一个接收闭包的函数
}

// 过程宏：
fn test_macro(){
    // 宏：rust 中一组相关特性的集合称谓；
    // 可以使用 macro_rulse! 构建声明宏（可能要弃用了），和三种过程宏：
    //  - 自定义 #[derive] 宏，用于 struct 或者 enum，可以指定随 derive 属性添加代码；
    //  - 类似属性的宏，在任何条目上添加自定义属性；
    //  - 类似函数的宏，看起来像函数的调用，对其指定为参数的 token 进行操作；
    // 宏是用来编写可以生成其它代码的代码，宏可以处理可变的参数。
    // 宏需要提前定义，或者引入蛋清作用域，函数可以在任意位置定义、使用


    // 基于属性生成代码的过程宏
}
