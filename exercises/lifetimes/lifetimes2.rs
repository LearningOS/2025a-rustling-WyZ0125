// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        //string2 是在内部代码块（{ ... }）中定义的 String，生命周期仅持续到内部代码块结束（离开代码块后会被销毁）。
        //当内部代码块结束后，string2 被销毁，但 result 可能仍然引用 string2（如果 string2 是较长的字符串），
        //此时 result 就成了悬垂引用，违反 Rust 的内存安全规则。
        println!("The longest string is '{}'", result);
        //这个修复体现了 Rust 生命周期的核心原则：引用的使用不能超过它所指向的数据的生命周期。
    }
    //println!("The longest string is '{}'", result);
}
