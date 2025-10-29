// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        //my_option.unwrap();
        // 原 unwrap() 会 panic，直接移除或替换为合理逻辑（如打印提示）
        println!("my_option is None, cannot unwrap!");

    }

    let my_arr = &[
        -1, -2, -3,//修复方案：在 -3 后添加逗号，确保每个元素独立。
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    //let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    // 3. 修复：单独调用 resize（无返回值），不赋值给变量
    //但是,Clippy 的 vec_resize_to_zero 规则指出：用 resize(0, value) 清空向量虽然功能可行，但意图不明确
    // ——resize 的主要用途是 “调整向量长度到任意值（可能增大也可能减小）”，而 clear() 是专门用于 “清空向量” 的方法，语义更清晰。
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();  // 先创建向量，再调用 clear 清空
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    //value_a = value_b;
    //value_b = value_a;
    // 4. 修复：用 Rust 标准的 swap 方法交换变量，或用临时变量；此处用 std::mem::swap 更简洁
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
