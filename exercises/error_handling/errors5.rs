// errors5.rs
//
// This program uses an altered version of the code from errors4.
//
// This exercise uses some concepts that we won't get to until later in the
// course, like `Box` and the `From` trait. It's not important to understand
// them in detail right now, but you can read ahead if you like. For now, think
// of the `Box<dyn ???>` type as an "I want anything that does ???" type, which,
// given Rust's usual standards for runtime safety, should strike you as
// somewhat lenient!
//
// In short, this particular use case for boxes is for when you want to own a
// value and you care only that it is a type which implements a particular
// trait. To do so, The Box is declared as of type Box<dyn Trait> where Trait is
// the trait the compiler looks for on any value used in that context. For this
// exercise, that context is the potential errors which can be returned in a
// Result.
//
// What can we use to describe both errors? In other words, is there a trait
// which both errors implement?
//
// Execute `rustlings hint errors5` or use the `hint` watch subcommand for a
// hint.


use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.

//这段代码的核心需求是让 main 函数能同时返回两种不同的错误类型（ParseIntError 和 CreationError），
//而 Box<dyn ???> 中需要填充一个所有错误类型都实现的 trait，这个 trait 就是 std::error::Error。
fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}
/*
当输入为有效正数（如 "42"）时：parse() 成功得到 42，PositiveNonzeroInteger::new(42) 成功返回实例，程序输出 output=PositiveNonzeroInteger(42)，最终返回 Ok(())。
当输入为非数字（如 "abc"）时：parse() 失败返回 ParseIntError，? 将其包装为 Box<dyn error::Error> 并由 main 返回。
当输入为负数（如 "-10"）时：parse() 成功得到 -10，PositiveNonzeroInteger::new(-10) 失败返回 CreationError::Negative，? 将其包装为 Box<dyn error::Error> 并由 main 返回。 */

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

//CreationError 已通过代码手动实现 error::Error（impl error::Error for CreationError {}）。
impl error::Error for CreationError {}
