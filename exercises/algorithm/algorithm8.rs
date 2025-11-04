/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/
//本题核心任务是 用两个队列（Queue）实现栈（myStack）的功能。


#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    // 2. 压栈：把元素加入存数据的队列（q1）
    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem); // 新元素始终入q1，保证q1是数据队列
    }

    // 3. 弹栈：通过中转队列，弹出栈顶元素（q1的最后一个元素）
    pub fn pop(&mut self) -> Result<T, &str> {
        // 先判断栈是否为空（两个队列都空）
        if self.is_empty() {
            return Err("Stack is empty");
        }

        // 步骤1：把q1中除最后一个元素外的所有元素，转移到q2
        while self.q1.size() > 1 {
            // 从q1出队，入q2（中转）
            let elem = self.q1.dequeue().unwrap(); // q1非空，unwrap安全
            self.q2.enqueue(elem);
        }

        // 步骤2：q1中剩余的最后一个元素，就是栈顶元素，弹出并保存
        let top_elem = self.q1.dequeue().unwrap(); // q1此时有1个元素，unwrap安全

        // 步骤3：交换q1和q2的角色（让q2成为新的数据队列，q1为空，下次中转用）
        std::mem::swap(&mut self.q1, &mut self.q2);

        // 返回栈顶元素
        Ok(top_elem)
    }

    // 4. 判断栈是否为空（两个队列都为空时，栈才空）
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}