// errors6.rs
//
// Using catch-all error types like `Box<dyn error::Error>` isn't recommended
// for library code, where callers might want to make decisions based on the
// error content, instead of printing it out or propagating it further. Here, we
// define a custom error type to make it possible for callers to decide what to
// do next when our function returns an error.
//
// Execute `rustlings hint errors6` or use the `hint` watch subcommand for a
// hint.


//总结：编写顺序逻辑链
//导入依赖 → 定义核心数据类型（PositiveNonzeroInteger）→ 定义该类型的创建错误（CreationError）→ 
//实现创建方法 → 定义统一错误类型（包装两类错误）→ 实现错误转换 → 编写核心解析函数。

use std::num::ParseIntError;

// This is a custom error type that we will be using in `parse_pos_nonzero()`.
// 这是一个自定义错误类型，将在 `parse_pos_nonzero()` 中使用
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    // TODO: add another error conversion function here.
    // fn from_parseint...
    // 1. 补充：从 ParseIntError 转换为自定义错误的函数
    fn from_parseint(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)
    }
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
//Box<dyn error::Error> 只能提供 Error trait 定义的基础能力
//ParsePosNonzeroError 调用者可以：
//直接将错误传递给上层（保留具体类型）；
//或匹配错误类型后，转换为自己的错误类型（携带更丰富的上下文）。

    // TODO: change this to return an appropriate error instead of panicking
    // when `parse()` returns an error.
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parseint)?;
    /*原代码 s.parse().unwrap() 会在解析失败时 panic，改为 s.parse().map_err(...)：
若 parse() 成功（Ok(i64)），则提取整数继续执行；
若 parse() 失败（Err(ParseIntError)），则通过 map_err 调用 from_parseint，将错误转为 ParsePosNonzeroError::ParseInt 并返回。
末尾的 ? 操作符：将 Result<i64, ParsePosNonzeroError> 中的整数提取出来（若错误则直接返回），确保后续逻辑只处理有效整数。
     */
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]//派生 trait：PartialEq 用于后续比较（如测试或业务逻辑），Debug 用于调试时打印信息。
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}
