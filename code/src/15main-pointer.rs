use std::{ops::Deref, rc::Rc, cell::RefCell};




fn main(){
    // pointer_box();
    // pointer_deref();
    // pointer_drop();
    // pointer_rc();
    pointer_refcell();
    
    
}

// 智能指针：行为和指针类似，但是有额外的元数据和功能
// 引用计数：记录所有者数量，使得一份数据被多个所有者同时持有，没有使用者时，自动清理
// 引用只是借用数据，智能指针很多时候拥有它所指向的数据
// String  和 Vec<T>：拥有一片内存区域，允许用户操作，有元数据（容量等）
// 智能指针一般实现：Deref（允许智能指针像引用一样使用） 和 Drop（允许自定义指针走出作用域时的代码）这两个 trait

// 常见智能指针：
//   Box<T> 在堆上分配内存
//   Rc<T> 启用多重所有权的引用计数类型
//   Ref<T> 和 RefMut<T> 通过 RefCell<T> 访问，运行时而不是编译时强制借用

//   内部可变模式：不可变数据类型暴露可修改内部值的 API
//   循环引用：内存泄露等

fn pointer_box(){
    //  Box<T> 允许在堆内存存储数据，而不是栈内存，实现了 Drop、Deref 两个指针
    // 使用场景：
    //    编译时内存大小无法确定，但是使用时，上下文需要确切大小；
    //    有大量数据，先移交所有权，但是需要确保操作时数据不会被复制；
    //    使用某个值时只关心是否实现了某个特定的 trait，而不关心具体类型；
    
    // 超出作用域，释放
    let b = Box::new(5);
    println!("{}",b);

    // 递归类型编译时无法知道空间大小
    // 指针的大小恒定

    // enum List{
    //     Cons(i32,List),
    //     Nil
    // };

    enum List{
        Cons(i32,Box<List>),
        Nil
    }
    let list = List::Cons(1,
        Box::new(List::Cons(2, 
                Box::new(List::Cons(3,
                    Box::new(List::Nil))
                )
            )
        )
    );
      
}

fn pointer_deref(){
    println!("asdasd");
    // 普通解引用
    let x = 5;
    let y = &x;
    assert_eq!(x, *y, " {} and {}", x, *y);

    // 如果实现了 Deref trait ，那么就可以使用 * 解引用
    let x  = 5;
    let y = Box::new(x);
    println!("{}",x==*y);

    // Box 实现
    struct MyBox<T>(T);
    impl  <T> MyBox<T> {
        fn new (x:T)->MyBox<T>{
            MyBox(x)
        }
    }
    // Deref trait 需要实现 deref 方法，借用 self，返回一个指向内部数据的引用
    impl <T>  Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let x  = 5;
    let y = MyBox::new(x);
    // *y 相当于 *(y.deref)
    println!("{} 111",x==*y);

    // 自动解引用
    fn hello(name:&str){
        println!("{}",name);
    }
    // &MyBox<String> -> &String 
    // String 实现了 Deref 继续解引用 -> &str
    hello(&MyBox::new(String::from("3333")));

    // DerefMut trait 可以重载 *
    // 在以下情况会执行 deref
    //   1.T:Deref<Target=U>，允许 &T 转换为 &U；
    //   2.T:DerefMut<Target=U>，允许 &mut T 转换为 &mut U；
    //   3.T:Deref<Target=U>，允许 &mut T 转换为 &U
}

fn pointer_drop(){
    // 离开作用域时可以自定义动作，比如文件、网络资源释放；
    // 需要实现 drop 方法
    struct Custom{
        data:String
    }
    impl Drop for Custom {
        fn drop(&mut self) {
            println!("{}",self.data)
        }
    }
    let c = Custom{
        data:String::from("CCC")
    };
    let b = Custom{
        data:String::from("BBB")
    };
    // drop 方法手动调用
    drop(c);
    println!("{}",222)
}

fn pointer_rc(){
    // Rc<T> 引用计数指针：多重所有权，计数为 0 自动销毁（只能用于单线程）
    // 希望将堆上的数据分享给多个程序使用，但是又无法确定是哪个部分最后使用完这些数据
    // clone：增加引用计数，strong_count：获得引用计数，还有弱引用计数 weak_count
    // 通过不可变引用，完成数据共享

    // b/c list 共享 a list 的数据
    // enum List{
    //     Cons(i32,Box<List>),
    //     Nil
    // }
    // let a = List::Cons(5,
    //     Box::new(
    //         List::Cons(6, Box::new(List::Nil))
    //     )
    // );
    // let b = List::Cons(3, Box::new(a)); // a 的所有权已经发生移动
    // let c = List::Cons(2, Box::new(a)); // 报错

    enum List{
        Cons(i32,Rc<List>),
        Nil
    }
    let a = Rc::new(List::Cons(5,
        Rc::new(
            List::Cons(6, Rc::new(List::Nil))
        )
    ));
    println!("{}",Rc::strong_count(&a));
    // Rc::clone 只增加引用计数不会深度拷贝，a.clone 深度拷贝
    let b = List::Cons(3, Rc::clone(&a)); // a 的所有权已经发生移动
    println!("{}",Rc::strong_count(&a));
    {
        let c = List::Cons(2, Rc::clone(&a)); // 报错
        println!("{}",Rc::strong_count(&a));
    }
    println!("{}",Rc::strong_count(&a));
}

fn pointer_refcell(){
    // 内部可变性：持有不可变引用完成数据更改，使用 unsafe 绕过可变性和借用规则
    // 很多时候需要对外暴露不可变，但是在内部可以修改 RefCell 可以实现这种情况

    
    // let x = 5;
    // let y = &mut x; // 不允许，无法借用不可变的值
    #[derive(Debug)]
    enum List{
        Cons(Rc<RefCell<i32>>,Rc<List>),
        Nil
    }
    let val = Rc::new(RefCell::new(5));
    let a = Rc::new(List::Cons(Rc::clone(&val), Rc::new(List::Nil)));
    let b = List::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a)); 
    let c = List::Cons(Rc::new(RefCell::new(7)), Rc::clone(&a));
    *val.borrow_mut() += 10;
    println!("{:?}",a);
    println!("{:?}",b);
    println!("{:?}",c);
}


// Rc、RefCell 可能造成循环引用，数据无法释放，造成内存泄露


