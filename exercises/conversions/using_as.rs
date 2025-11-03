// using_as.rs
//
// Type casting in Rust is done via the usage of the `as` operator. Please note
// that the `as` operator is not only used when type casting. It also helps with
// renaming imports.
/*在 Rust 中，类型转换通过 as 运算符实现。请注意，as 运算符不仅用于类型转换，它还能帮助重命名导入的模块 / 成员。
本题目标是确保除法运算能够编译通过，并且返回正确的类型。 */
// The goal is to make sure that the division does not fail to compile and
// returns the proper type.
//
// Execute `rustlings hint using_as` or use the `hint` watch subcommand for a
// hint.


fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len() as f64//Rust 不允许 f64 / usize 这样的混合类型运算，必须显式转换类型。
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
