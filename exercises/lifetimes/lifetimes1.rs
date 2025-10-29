// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.

/*
核心原则
生命周期的目的是避免悬垂引用：编译器通过生命周期检查确保 “引用的有效期 ≤ 被引用值的有效期”。
标注不改变实际生命周期：仅帮助编译器理解引用间的关系，最终由编译器验证有效性。
省略规则优先：大多数场景（如简单函数、方法）可依赖省略规则，无需手动标注，仅在编译器无法推断时补充标注。
 */


//编译器不知道返回的引用是来自参数 x 还是 y，也无法确定返回引用的生命周期与哪个参数的生命周期一致；
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //需要为 longest 函数添加 生命周期标注，明确 “返回引用的生命周期与两个参数的生命周期一致”—— 
    //即返回的引用有效期，不会超过 x 和 y 中生命周期较短的那个（确保引用始终有效）。
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd"); // string1 是 String 类型，所有者是 main 函数
    let string2 = "xyz";  // string2 是字符串字面量，生命周期为 'static

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
