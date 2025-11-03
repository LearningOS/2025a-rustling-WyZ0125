// tests9.rs
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// languages, and it can even link within the code itself! It makes it through the extern
// block, just like the code below.
//
// The short string after the `extern` keyword indicates which ABI the externally imported
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// "C" for standard C ABI, "stdcall" for the Windows ABI.
//
// The externally imported functions are declared in the extern blocks, with a semicolon to
// mark the end of signature instead of curly braces. Some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
//
// If you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same ABI string note. The default ABI
// for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// the whole extern term can be omitted.
//
// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
//
// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module Foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//
// You should NOT modify any existing code except for adding two lines of attributes.


extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // No `extern` equals `extern "Rust"`.
    // 给实际函数添加 no_mangle 属性，禁用名字修饰，暴露原始符号
    #[no_mangle]
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}


/*
二、问题分析
核心需求是让 extern "Rust" 块中声明的两个函数（my_demo_function 和 my_demo_function_alias）
能正确链接到 Foo 模块内的 my_demo_function，需解决两个关键问题(学习两个属性用法)：

符号可见性：Foo::my_demo_function 是模块内的普通函数，
默认会被 Rust 进行 “名字修饰”，且仅在模块内可见，外部 extern 块无法定位到它 —— 需用 #[no_mangle] 禁用名字修饰，让函数以原始名称作为符号暴露。
别名映射：my_demo_function_alias 是 my_demo_function 的别名，
需用 #[link_name = "my_demo_function"] 告诉链接器：“my_demo_function_alias 实际对应的符号是 my_demo_function”，从而让两个声明指向同一个函数实现。
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
