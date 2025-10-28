// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.


use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    //let cost = total_cost(pretend_user_input)?;
    //main 函数不能直接使用 ? 操作符—— 因为 ? 的作用是 “若 Result 为 Err 则返回该错误”，
    //而默认的 main 函数返回类型是 ()（无返回值），无法承载 Err 类型，导致编译失败。

    // 用 match 显式处理 total_cost 的返回值
    match total_cost(pretend_user_input) {
        Ok(cost) => {
            // 成功：执行原逻辑（判断是否能购买）
            if cost > tokens {
                println!("You can't afford that many!");
            } else {
                tokens -= cost;
                println!("You now have {} tokens.", tokens);
            }
        }
        Err(e) => {
            // 失败：自定义错误处理（如打印错误信息）
            println!("Error: Failed to parse input - {}", e);
        }
    }
    /*
// 1. 修改 main 函数返回类型为 Result<(), ParseIntError>
fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // 2. 现在可以正常使用 ?，错误会被 main 函数返回
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }

    // 3. 程序正常结束时，返回 Ok(())（空成功结果）
    Ok(())
}
     */
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
