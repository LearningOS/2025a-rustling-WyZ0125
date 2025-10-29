// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.


//错误在于：结构体 Book 包含引用类型字段（&str），但未标注生命周期。
//Rust 要求所有包含引用的结构体必须显式声明生命周期，以确保结构体实例的生命周期不超过其引用字段所指向数据的生命周期。

struct Book <'a>{
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
