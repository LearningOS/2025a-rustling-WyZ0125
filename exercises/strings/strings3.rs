// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.



/*
🔹 &str 是“只读引用”，轻便但不能改；
🔹 String 是“拥有者”，能改、能扩展、能拼接。
🔹 需要修改字符串时，用 String；只需读取时，用 &str。
 */
/*🧭 快速记忆：
🔹 String 能改、能拼、能扩展（堆上所有权）
🔹 &str 是引用，能读不能改
🔹 共同点：都能用 .len()、.contains()、.replace()、.trim() 等“读取型”方法 */

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!//去除空格
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    input.to_string() + " world!"
    //format!("{} world!", input)
    //
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    input.replace("cars","balloons").to_string()
}

// 📘 Rust &str 常用方法速查笔记
// --------------------------------------------
// 🧠 快记口诀：
// 转为 String → .to_string() / .to_owned()
// 拼接字符串 → + / format! / .push_str()
// 判断包含等 → .contains() / .starts_with() / .ends_with()
// 查找替换 → .find() / .replace()
// 去空格 → .trim()
// 大小写 → .to_lowercase() / .to_uppercase()
// 切片 → &s[start..end]
// 遍历 → .chars() / .bytes()
// --------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
