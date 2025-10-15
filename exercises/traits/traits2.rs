// traits2.rs
//
// Your task is to implement the trait `AppendBar` for
// a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.


trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String>{//表示给 Vec<String> 类型实现该 trait
    fn append_bar(mut self)->Self{//这里用 mut self 因为我们需要修改向量
        /*为什么这里要 mut self   适合链式操作
self 表示方法获得了 vector 的所有权。
.push(...) 需要 可变 vector，所以必须标记 mut self。
如果写成 self 而不加 mut，编译器会报错：
cannot borrow `self` as mutable, as it is not declared as mutable
因为默认所有权传入的方法，变量是不可变的。 */
        self.push("Bar".to_string());//push:Vec<T> 自带的方法，用于 在向量尾部添加一个元素
        //Vec<String> 需要的是 String，所以必须把 &str 转成 String，to_string() 就做了这个转换。
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
