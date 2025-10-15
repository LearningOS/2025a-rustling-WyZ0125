// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.


#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word)=optional_target {assert_eq!(word, target);}
        /*
        Some(word) 是一个模式（pattern）
它表示：
只有当 optional_target 是 Some(...) 时才会进入；
把 Some 里面包的内容绑定给变量 word。 */
        /*
        match optional_target {
        Some(word) => {
        println!("{}", word);
        }
        _ => {}
        }
     */
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        /*vector.pop() 返回的类型本身是 Option<T>，
所以你实际上会有「两层 Option」的情况。
Rust 允许你在 while let 或 if let 中嵌套匹配（也就是“堆叠 Option”）。 */
        while let Some(Some(integer)) = optional_integers.pop() {
            //外层 Option 是 Vec 的行为（是否取出成功），Vec::pop() 的函数签名是这样的：
            //fn pop(&mut self) -> Option<T>
            //内层 Option 是 元素自身的语义（是否有值） Option<Option<i8>>
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
