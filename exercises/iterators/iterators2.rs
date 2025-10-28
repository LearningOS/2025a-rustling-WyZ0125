// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.



// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();//把字符串转成字符迭代器
    match c.next() {//取出第一个字符
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>()+c.as_str(),
        /*input.chars() 把字符串转成字符迭代器；
c.next() 取出第一个字符；
first.to_uppercase() 返回一个 迭代器（因为某些 Unicode 字母大写后会变成多个字符，比如德语的“ß”→“SS”）；
.collect::<String>() 把它收集成字符串；
c.as_str() 返回当前迭代器剩下的 未消费的子串；
使用 + 拼接。 */
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    //words.iter().map(|word| capitalize_first(word)).collect()
    words.iter().copied().map(capitalize_first).collect()
//words.iter()创建一个遍历 words 的迭代器，元素类型为 &&str。

//map(|word| capitalize_first(word))
//含义：把迭代器中的每个元素映射成另一个值
//.map() 是高阶函数，接收一个闭包 |word| ...
//这里闭包的作用是：对每个元素执行 capitalize_first
//即 "hello" → "Hello"，"world" → "World"

//collect()
//把 map 生成的迭代器收集成 Vec<String>。
//Rust 会自动根据函数返回类型推断容器类型。

//等价于更“展开”的写法：
/* 
let mut result = Vec::new();
for word in words {
    result.push(capitalize_first(word));
}
result
*/
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    capitalize_words_vector(words).concat()
    //.concat()直接把所有字符串拼接成一个新 String 更快（少一次分隔符逻辑判断） 相当于：vec.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
