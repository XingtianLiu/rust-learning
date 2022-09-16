/*
 * @Author: lxt
 * @Date: 2022-09-15 13:07:35
 * @LastEditors: [lxt]
 * @LastEditTime: 2022-09-15 16:08:19
 * @Description: // # 
 */



// cargo build 执行 dev profile
// cargo build --release 执行 release profile
// 自定义 profile：
//   1.在 Cargo.toml 中加 [profile.xxx] 域，覆盖子集；
//      [profile.dev]
//      opt-level=0 # 编译时执行哪种程度的优化 0~3 越大编译时间越长
//   2./// 用于生成文档注释，可以写 Markdown 语法，最后是 html，放置在说明条目之前，使用 cargo doc --open 生成文档（target/doc）；
//     注释文档可能会用到：Examples、Panics、Errors、Safety（unsafe 原因）
//     运行 cargo test 时（或者 cargo test --doc），文档注释的代码块会被当做测试用例执行
//   3.//! 描述外层注释，一般加载 lib.rs 中
//   4.pub use 可以重新导出，创建一个与内部私有结构不同的对外公关结构，
//   5.发布：cargo login 输入 api token，保存在本地 ~/.cargo/credentials 下；cargo publish 发布，发布之后不能删除；
//   6.撤回某个版本：cargo yank --vers 1.0.1 （相当于废弃某个版本）cargo yank --vers 1.0.1 --undo（取消撤回）


//! # Code
//! Code is a demo
//! ha ha ha ha
//! 


/// Add one to the number given
/// 
/// # Examples
/// 
/// ```
///   let arg = 5;
///   let answer = evan_toy::add_one(arg);
///   assert_eq!(6,answer);
/// ```

pub fn add_one(x:i32)->i32{
    x+1
}



pub mod kinds{
    pub enum PrimaryColor{
        Red,
        Blue,
        Yellow
    }
    pub enum SecondaryColor{
        Orange,
        Green,
        Purple
    }
}

pub mod Utils{
    use crate::kinds::{PrimaryColor, SecondaryColor};
    pub fn mix(c1:PrimaryColor,c2:PrimaryColor)->SecondaryColor{
        SecondaryColor::Green
    }
}

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::Utils::mix;

