// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.


mod macros {
    macro_rules! my_macro {// 私有宏
        () => {
            println!("Check out my macro!");
        };
    }
    pub(crate) use my_macro;//把模块内的私有宏，变成 “整个项目内可见”
    //为什么 pub use 不行？
//pub 是 “完全公开”（其他项目也能用），但原宏是私有，权限不够，会报错
}

fn main() {
    //调用方式：模块名::宏名!()
    macros::my_macro!();
}
