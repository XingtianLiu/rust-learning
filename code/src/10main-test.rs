fn main(){

}

// 测试就是要一个函数，通常执行三个操作：
// - 准备数据
// - 运行测试代码
// - 断言

// 测试函数 #[test] 标注
// cargo test 执行测试

// cargo test 默认并行运行所有测试用例，测试通过不显示输出
// cargo test -- --test-threads=1 线程数量
// cargo test -- --show-output 默认成功不打印信息，加这个参数可以打印正确信息
// cargo test test 可以指定模块名称、方法名称
// cargo test -- --ignored 只运行 ignore 标记的测试

//  rust 对测试分类
// 单元测试： #[cfg(test)]，可以测试私有函数
// 集成测试： 只能调用对外公开的 api，需要创建 tests 目录和 src 平级， cargo test 运行，会被特殊对待，不用 #[cfg(test)]
// cargo test test_add_two 指定集成测试名称
// cargo test --test integration_test 指定文件名
// 集成测试：如果只是 helper，可以在 tests 下新建一个目录，第二级目录不会单独编译、测试

fn is_adult(age:i32)->bool{
    age >= 18
}

#[cfg(test)]
mod test{
    use super::*; // 导入外部模块所有内容
    
    #[test]
    fn it_works(){
        assert_eq!(2+2,4);
    }
    // #[test]
    // fn an(){
    //     panic!("error")
    // }

    #[test]
    fn test_is_adult(){
        // true 通过 false 失败
        assert!(is_adult(19));
    }

    #[test]
    fn test_eq(){
        assert_eq!(1,1);
        assert_ne!(2,1);
    }

    #[test]
    fn test_err(){
        assert!(1==1,"错误信息：{}",123)
    }

    #[test]
    #[should_panic(expected="123")] // 预期会报错，错误信息包括 123
    fn test_a(){
        assert!(1==2,"错误信息：{}",1231)
    }

    #[test]
    fn test_result() -> Result<(),String>{
        // 无需 panic，使用 result，返回 OK 通过，不需要添加 should_panic
        if 2 + 2 == 4 {
            Ok(())
        }else{
            Err(String::from("adfsad"))
        }
    }

    #[test]
    #[ignore = "reason"] // 忽略
    fn test_ignore(){
        assert_eq!(1+1,3,"哈哈哈");
    }


    
}
