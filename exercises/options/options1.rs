// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.


// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a
    // value of 0 The Option output should gracefully handle cases where
    // time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!
    if time_of_day > 23 {
        None
    } else if time_of_day == 22 || time_of_day == 23 {
        Some(0)
    } else {
        Some(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the
        // Option?
        let icecreams = maybe_icecream(12).expect("Error");//❌ 遇到 None 会直接终止程序，没有机会恢复或处理
        /* match 模式匹配写法
✅ 你可以选择不 panic，而 gracefully fallback（平滑降级）
✅ 更符合“Rust 式错误处理哲学”：显式地处理 None 而不是崩溃
⚙️ 稍微比 .expect() 冗长
        let icecreams = match maybe_icecream(12) {
            Some(value) => value,
            None => {
                println!("Warning: invalid time, using default 0");
                0
            }
};
*/
        assert_eq!(icecreams, 5);
    }
}
