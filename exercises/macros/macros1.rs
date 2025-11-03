// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.

//Rust 中的宏是编译期代码生成工具，不是函数（函数运行期执行，宏编译期展开为具体代码）；

//1. 宏定义关键字：macro_rules! + 宏名（my_macro）
macro_rules! my_macro {
    //// 2. 宏的“匹配规则”：() 表示匹配“无参数”的调用形式（如 my_macro!()）
    () => {
        println!("Check out my macro!");
    };
}

/* eg:
// 支持带字符串参数的宏定义
macro_rules! my_macro {
    // 匹配“带一个字符串字面量参数”的调用（如 my_macro!("Hello")）
    ($msg:expr) => {
        println!("Macro says: {}", $msg);
    };
    // 保留原无参数规则（支持两种调用形式）
    () => {
        println!("Check out my macro!");
    };
} */

fn main() {
    my_macro!();
}
