
// concurrent、parallel


// 线程的实现：
//  1.操作系统的 API 创建线程，一个操作系统线程对应一个语言的线程，运行时小；
//  2.语言自己实现线程，和系统线程不是 1:1，是 M:N，需要较大的运行时；

// rust 标准库提供 1:1 模型，社区有 M:N 模型

use std::{thread, time::Duration, sync::{mpsc::{channel, Sender}, Mutex, Arc}};

fn main(){
    // test_thread();
    // test_move();
    // test_channel();
    test_sync();
}

fn test_thread(){
    let handler = thread::spawn(||{
        for i in 0..10 {
            println!("{}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // 主线程结束，不管子线程是否结束都终止程序
    for i in  20..24 {
        println!("{}",i);
        thread::sleep(Duration::from_millis(1));
    }
    // handler 都结束才继续执行
    handler.join().unwrap();
}

fn test_move(){
    // move 闭包可以把值的所有权从一个线程转移到另一个线程
    let v =vec![1,2,3];
    // 闭包的生命周期可能比 v 要长
    // let handler = thread::spawn(||{
    //     println!("{:?}",v);
    // });
    let handler = thread::spawn(move||{
        // 闭包的生命周期可能比 v 要长
        println!("{:?}",v);
    });
    // println!("{:?}",v); // 主线程不能再使用 v
    handler.join().unwrap();
}

fn test_channel(){
    // 消息传递通信
    // channel 包含消息发送端和接收端，多个生产者，一个消费者
    let (tx,rx) = channel();
    thread::spawn(move||{
       let str = String::from("cccc");
       thread::sleep(Duration::from_secs(3));
       tx.send(str).unwrap();
    //    println!("2222 {}",str); // 所有权移动
    });
    // 一种阻塞，有消息再往下执行
    println!("11111");
    let received = rx.recv().unwrap();
    println!("123 {}",received);


    /**
     * 
     * 发送端克隆
     * 
     */
    let (tx,rx) = channel();
    let tx1 = Sender::clone(&tx);
    thread::spawn(move||{
        let str = String::from("tx1 tx1 tx1");
        thread::sleep(Duration::from_secs(1));
        tx1.send(str).unwrap();
     });
     thread::spawn(move||{
        let str = String::from("tx tx tx tx");
        thread::sleep(Duration::from_secs(1));
        tx.send(str).unwrap();
     });
     for received in rx {
        println!("123 {}",received);
     }

    /***
     * 
     * 共享内存，多所有权，多个线程同时访问 Mutex 互斥锁
     * 
     */
    //  let m = Mutex::new(10);
    //  {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    //  }
    //  println!("mutex main {:?}",m);

    
    //  Arc 和 Rc 类似，但是可以用于并发，保证原子性
     let mut handlers = vec![];
     let counter = Arc::new(Mutex::new(10));
     for i in 0..10 {
        let counter = Arc::clone(&counter);
        
        let handler = thread::spawn(move||{
            let mut num = counter.lock().unwrap();
            *num += i;
         });
         handlers.push(handler);
     }
     for handler in handlers {
        handler.join().unwrap();
     }
     println!("mutex main {:?}",counter);
     println!("result {}",*counter.lock().unwrap());

}

fn test_sync(){
    // Send trait 允许线程间转移所有权
    // Async trait 允许多线程同时访问
    // 手动实现很难保证安全
}