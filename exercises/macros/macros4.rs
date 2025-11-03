// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.


#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };// 这里添加分号
    //宏定义中，多个分支（arm）之间需要用 ; 分隔，就像函数参数或语句之间需要分号一样。
    //原代码中两个分支之间没有分隔符，编译器无法区分它们，添加分号后即可正确解析两个分支。
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
