

// pub trait 
// rust 没有继承，但是可以使用 trait 默认方法
// 泛型 + trait 约束 实现多态


// trait 作用于泛型，rust 会执行单态化，编译器会为泛型类型的具体类型生成非泛型实现，无法再编译过程中知道，很多优化无法执行
fn main(){
    test_oop1();
}

fn test_oop1(){
   let screen = Screen{
    components:vec![
        Box::new(Button{})
    ]
   };
   screen.run();
}

pub trait Draw{
    fn draw(&self);
}

pub struct Screen{
    // Box<dyn Draw> 都实现了 Draw
    pub components:Vec<Box<dyn Draw>>
}
impl Screen {
    pub fn run(&self){
        for component in self.components.iter() {
            component.draw();
        }
    }
}

struct Button{

}
impl Draw for Button {
    fn draw(&self) {
        println!("button");
    }
}
