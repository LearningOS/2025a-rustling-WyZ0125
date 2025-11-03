// as_ref_mut.rs
//
// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.

//本题需要正确使用 AsRef 和 AsMut 特质，实现引用转换和可变操作。
//AsRef<T>：用于廉价的 “引用到引用” 转换（如 String → &str，Box<u32> → &u32），通过 as_ref() 方法实现。
//AsMut<T>：与 AsRef 类似，但用于可变引用转换（如 &mut Box<u32> → &mut u32），通过 as_mut() 方法实现。

// Obtain the number of bytes (not characters) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn byte_counter<T:AsRef<str>>(arg: T) -> usize {
    //需要参数能通过 as_ref() 转换为 &str（因为 &str 有 as_bytes() 方法）。
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn char_counter<T:AsRef<str>>(arg: T) -> usize {
    //需要参数能通过 as_ref() 转换为 &str（因为 &str 有 chars() 方法）。
    arg.as_ref().chars().count()
}

// Squares a number using as_mut().
// TODO: Add the appropriate trait bound.
fn num_sq<T:AsMut<u32>>(arg: &mut T) {
    // TODO: Implement the function body.
    let num = arg.as_mut();
    *num *= *num;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
