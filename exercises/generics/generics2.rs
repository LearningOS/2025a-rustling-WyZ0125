// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.


struct Wrapper<T> {//<T> 表示这是一个 泛型结构体。
    value: T,
}

impl<T> Wrapper<T> {//diyige 声明!!!你告诉编译器：在这个 impl 代码块内部，你会用到一个叫 T 的泛型类型。
    //Wrapper<T> 表示 “针对所有 Wrapper<T> 类型（带有类型参数 T）实现方法”。
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
