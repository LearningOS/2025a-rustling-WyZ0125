// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let ref y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
    /*不加 ref → y 是 拥有值 的变量（Option<Point>）。
加了 ref → y 是 指向值的引用（&Option<Point>）:ref 是在绑定阶段“取引用”，避免所有权移动；
                                            & 是在匹配阶段“匹配引用”，避免移动值.两者的目标都是：让你能在 match 之后继续使用原变量。
 
 let y: Option<Point> = Some(Point { x: 100, y: 200 });
那么 y 是拥有值的。
match y { Some(p) => ... } 会尝试“移动” p 出去（Point 被移出 Option）
这样在 match 结束后，y 的所有权已经被移动走
所以 y; 这行会报错：*/
}
