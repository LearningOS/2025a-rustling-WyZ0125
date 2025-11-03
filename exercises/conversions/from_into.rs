// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
//当前的测试用例，只比较了 Person 的字段（name 和 age），没直接比较 Person 实例本身，
//而字段（String 和 usize）早就默认实现了 PartialEq，所以不用给 Person 加#[derive(Debug, PartialEq)]。
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {//含义：default 方法的返回值类型，表示这个方法最终会返回一个 Person 类型的实例。
        Person {//Person 结构体的实例构造语法，表示 “创建一个 Person 类型的具体对象”。
            name: String::from("John"),
            age: 30,
        }
    }
}

// Your task is to complete this implementation in order for the line `let p =
// Person::from("Mark,20")` to compile Please note that you'll need to parse the
// age component into a `usize` with something like `"4".parse::<usize>()`. The
// outcome of this needs to be handled appropriately.
//需要为 Person 实现 From<&str> trait，将字符串（格式为 "姓名，年龄"）转换为 Person 实例。
//按照要求处理各种异常情况，失败时返回默认的 Person。
// Steps:
// 1. If the length of the provided string is 0, then return the default of
//    Person.
// 2. Split the given string on the commas present in it.
// 3. Extract the first element from the split operation and use it as the name.
// 4. If the name is empty, then return the default of Person.
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age.
// If while parsing the age, something goes wrong, then return the default of
// Person Otherwise, then return an instantiated Person object with the results


/*
From 与 Into 核心笔记
一、本质：双向的 “值到值” 转换 trait
两者都属于 Rust 标准库的 std::convert 模块，用于不同类型间的廉价值转换（非引用转换，会转移所有权或复制值）。
核心关系：若为类型 A 实现了 From<B>，则 Rust 会自动为类型 B 实现 Into<A>，无需手动写代码。
二、From trait：“从 B 转换成 A”
1. 定义与用法
作用：为类型 A 实现 From<B>，表示 “A 可以从 B 转换而来”。
核心方法：fn from(value: B) -> Self（接收 B 类型的值，返回 A 类型的 Self）。
2. 本题中的例子
为 Person 实现 From<&str>，表示 “Person 可以从 &str（字符串切片）转换而来 */

impl From<&str> for Person {
    fn from(s: &str) -> Person {
// 步骤1：如果字符串为空，返回默认值
        if s.is_empty() {
            return Person::default();
        }

        // 步骤2：按逗号分割字符串
        let parts: Vec<&str> = s.split(',').collect();

        // 步骤3：必须恰好分割出两个部分，否则返回默认值
        if parts.len() != 2 {
            return Person::default();
        }

        let name_part = parts[0];
        let age_part = parts[1];

        // 步骤4：姓名为空，返回默认值
        if name_part.is_empty() {
            return Person::default();
        }

        // 步骤5：解析年龄为 usize，失败则返回默认值
        let age = match age_part.parse::<usize>() {
            Ok(num) => num,
            Err(_) => return Person::default(),
        };

        // 所有检查通过，返回新的 Person 实例
        Person {
            name: String::from(name_part),
            age,
        }
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an
        // error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
