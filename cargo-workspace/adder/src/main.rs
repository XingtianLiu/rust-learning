/*
 * @Author: lxt
 * @Date: 2022-09-15 16:11:22
 * @LastEditors: [lxt]
 * @LastEditTime: 2022-09-15 16:32:38
 * @Description: // # 
 */
use add_one;

// cargo build
// cargo run -p adder 运行

// cargo test 会一次性执行所有测试
// 需要单独发布

// 安装二进制包（有 main.rs 或者其他文件被指定为二进制文件）
// cargo install rust_tutorials（在根目录的 bin 下，rustup 安装的 rust 则在 ~/.cargo/bin 下）需要配置环境变量
// 可以使用该特性，扩展 cargo 命令

fn main() {
   println!("{}",add_one::add_one(19));
}
