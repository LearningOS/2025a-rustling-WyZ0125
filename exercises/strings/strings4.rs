// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");//"blue" 是字符串字面量，类型是 &'static str（不可变切片）;字面量默认是 &str
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());//into() 会推断成 String
    string(format!("Interpolation {}", "Station"));//format! 宏生成 String
    string_slice(&String::from("abc")[0..1]);//切片 [0..1] 返回 &str
    string_slice("  hello there ".trim());//trim() 返回 &str
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
/*
类型	说明	常见生成方式 / 函数
&str	不可变字符串切片，指向某个字符串的内存	字面量 "..."、切片 &s[..]、trim()、get()
String	可变、拥有所有权的字符串	String::from("...")、.to_string()、.to_owned()、format!()、.replace()、.into()（推断为 String） 
任何需要所有权或可以修改字符串的方法（to_string、to_owned、format!、replace 等）通常会返回 String。
*/
}
