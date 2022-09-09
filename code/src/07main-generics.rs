



fn main(){
    // fn1();
    generics();
    // 泛型在编译时执行了单态化，替换成了具体类型
    
}

fn fn1(){
    fn max_num (arr:&[i32]) -> i32{
        let mut max = arr[0];
        // arr 是 [i32] 的引用，&num 解构 i32
        for &num in arr{
            if num > max {
                max = num
            }
        }
        max
    }
    println!("max：{}",max_num(&vec![1,2,3,4,5]));
}

fn generics(){
    fn print1 <T:std::fmt::Display>(t:T) -> T{
       println!("{}",t);
       t
    }
    print1(1);
    print1("xxx");

    // 结构体泛型
    struct Point <T>{
        x:T,
        y:T
    }
    impl <T> Point<T> {
        fn x(&self) -> &T{
            &self.x
        }
    }
    impl  Point<i32> {
        fn y(&self) -> &i32{
            &self.y
        }
    }
    let a = Point{x:1,y:222};
    let b = Point{x:1.1,y:1.1}; // 没有 y 方法

    struct Point1<T,U>{
        x:T,
        y:U
    }
    impl <T,U> Point1<T,U>{
        fn mixup<V,W>(self,other:Point1<V,W>)->Point1<T,W>{
            Point1 { x: self.x, y: other.y }
        }
    }

    // 枚举泛型
    enum Opt<T> {
        Sm(T),
        None
    }
}
