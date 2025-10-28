// errors2.rs
//
// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy, and
// the `total_cost` function will calculate the total cost of the tokens. Since
// the player typed in the quantity, though, we get it as a string-- and they
// might have typed anything, not just numbers!
//
// Right now, this function isn't handling the error case at all (and isn't
// handling the success case properly either). What we want to do is: if we call
// the `parse` function on a string that is not a number, that function will
// return a `ParseIntError`, and in that case, we want to immediately return
// that error from our function and not try to multiply and add.
//
// There are at least two ways to implement this that are both correct-- but one
// is a lot shorter!
//
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.


use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    //parse 是 Rust 中用于 “类型转换” 的方法，
    //专门把字符串（&str 或 String）解析成其他类型（比如整数、浮点数等），但解析可能失败，
    //所以返回 Result 类型来处理成功 / 失败的情况。
    let qty = match item_quantity.parse::<i32>(){
        Ok(num) => num,
        Err(e)=> return Err(e)
    };
//方案 2：使用 ? 操作符（简洁写法）
//let qty = item_quantity.parse::<i32>?;
//成功时：返回 Ok(解析后的值)，比如 "34".parse::<i32>() 会返回 Ok(34)；
//失败时：返回 Err(错误信息)，比如 "beep boop".parse::<i32>() 会返回 Err(ParseIntError)（专门表示整数解析错误的类型）。

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
