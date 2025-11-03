// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.

//顶层宏默认遵循词法顺序，必须 “先定义，后使用”（局部宏更是严格遵循此规则）；



macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}


fn main() {
    my_macro!();
}