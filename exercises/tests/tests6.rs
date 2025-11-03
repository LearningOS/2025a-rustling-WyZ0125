// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.


struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.
    let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) };
    //Box::from_raw(ptr)（函数中使用）：是 Rust 标准库的 unsafe 函数，
    //将 *mut T 原始指针重构为 Box<T>。它的安全契约是：
    //ptr 必须是通过 Box::into_raw 生成的有效指针（确保指针指向的内存是 Box 分配的、未被释放的）。
    ret.b=Some("hello".to_owned());
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;//记录 a 字段地址 ptr_1
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };
        //Box::into_raw(box)（测试中使用）：是 Rust 标准库的安全函数，
        //将 Box<T> 转为 *mut T 原始指针。转换后，Box 的所有权 “转移到指针”—— 
        //后续需通过 Box::from_raw 重构 Box，否则会导致内存泄漏（因为 Box 的自动析构逻辑被暂时放弃）。

        let ptr_2 = &ret.a as *const u128 as usize;
//记录返回 Box 中 a 字段地址 ptr_2，断言 ptr_1 == ptr_2（地址未变），同时断言 b 字段值正确，测试通过。
        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
    }
}
