/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

#[derive(Debug)]
struct Stack<T> {
	size: usize,
	data: Vec<T>,
}
impl<T> Stack<T> {
	fn new() -> Self {
		Self {
			size: 0,
			data: Vec::new(),
		}
	}
	fn is_empty(&self) -> bool {
		0 == self.size
	}
	fn len(&self) -> usize {
		self.size
	}
	fn clear(&mut self) {
		self.size = 0;
		self.data.clear();
	}
	fn push(&mut self, val: T) {
		self.data.push(val);
		self.size += 1;
	}
	fn pop(&mut self) -> Option<T> {
		if self.is_empty() {
            None // 栈空时返回 None
        } else {
            self.size -= 1; // 先减大小（因为 data.pop() 会移除最后一个元素）
            self.data.pop() // 弹出 Vec 最后一个元素（栈顶）
        }
	}
	fn peek(&self) -> Option<&T> {
		/*为什么有人会写 0 == self.size？
这种写法本质是一种防御性编程习惯，目的是避免不小心把 == 写成 =（赋值运算符）导致 bug：
如果不小心写成 0 = self.size，Rust 会直接报错（因为 0 是常量，不能被赋值）；
但如果写成 self.size = 0，Rust 不会报错（这会变成 “把 0 赋值给 self.size” 的错误逻辑，而非判断）。 */
		if 0 == self.size {
			return None;
		}
		self.data.get(self.size - 1)
	}
	fn peek_mut(&mut self) -> Option<&mut T> {
		if 0 == self.size {
			return None;
		}
		self.data.get_mut(self.size - 1)
	}
	fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
	fn iter(&self) -> Iter<T> {
		let mut iterator = Iter { 
			stack: Vec::new() 
		};
		for item in self.data.iter() {
			iterator.stack.push(item);
		}
		iterator
	}
	fn iter_mut(&mut self) -> IterMut<T> {
		let mut iterator = IterMut { 
			stack: Vec::new() 
		};
		for item in self.data.iter_mut() {
			iterator.stack.push(item);
		}
		iterator
	}
}
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.0.is_empty() {
			self.0.size -= 1;self.0.data.pop()
		} 
		else {
			None
		}
	}
}
struct Iter<'a, T: 'a> {
	stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}
struct IterMut<'a, T: 'a> {
	stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}

// 2. 实现 bracket_match 函数：利用栈判断括号匹配
fn bracket_match(bracket: &str) -> bool {
    let mut stack = Stack::new(); // 创建一个存储“左括号”的栈

    for c in bracket.chars() {
        match c {
            // 情况1：遇到左括号，压入栈中（后续等待匹配右括号）
            '(' | '[' | '{' => stack.push(c),

            // 情况2：遇到右括号，检查是否与栈顶左括号匹配
            ')' | ']' | '}' => {
                // 先判断栈是否为空：若空，说明没有对应的左括号，直接返回 false
                let top_left = stack.pop();
                match (top_left, c) {
                    // 左括号与右括号类型匹配（如 '(' 对应 ')'）
                    (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') => continue,
                    // 类型不匹配（如 '(' 对应 ']'）或栈空（无左括号），返回 false
                    _ => return false,
                }
            }

            // 情况3：非括号字符（如数字、字母、符号），直接跳过
            _ => continue,
        }
    }

    // 遍历结束后，栈必须为空：若不为空，说明有未匹配的左括号（如 "[[[]]]]" 栈中剩 1 个 '['）
    stack.is_empty()
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1(){
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_2(){
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_3(){
		let s = "{{([])}}";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_4(){
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_5(){
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_6(){
		let s = "";
		assert_eq!(bracket_match(s),true);
	}
}