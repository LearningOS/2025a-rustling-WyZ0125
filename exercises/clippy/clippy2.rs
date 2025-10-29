// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut res = 42;
    let option = Some(12);
    //for x in option {
    /*Option 类型要么是 Some(x)（一个值），要么是 None（空），
    而 for 循环通常用于遍历 “多个元素”（如数组、集合）。用 if let 能明确表达 “处理单个可能存在的值” 的意图，
    更符合 Rust 代码风格。 */
    if let Some(x) = option{
        res += x;
    }
    println!("{}", res);
}
