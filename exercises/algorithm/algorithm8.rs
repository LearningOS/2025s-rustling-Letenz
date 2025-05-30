/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

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

pub struct myStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }
    
    pub fn push(&mut self, elem: T) {
        // 将新元素添加到非空的队列中
        // 如果两个队列都为空，则添加到 q1
        if !self.q1.is_empty() {
            self.q1.enqueue(elem);
        } else {
            self.q2.enqueue(elem);
        }
    }
    
    pub fn pop(&mut self) -> Result<T, &str> {
        // 确定哪个队列有元素
        let (from_queue, to_queue) = if !self.q1.is_empty() {
            (&mut self.q1, &mut self.q2)
        } else if !self.q2.is_empty() {
            (&mut self.q2, &mut self.q1)
        } else {
            return Err("Stack is empty");
        };
        
        // 将元素从一个队列移动到另一个队列，直到只剩下最后一个元素
        while from_queue.size() > 1 {
            let elem = from_queue.dequeue().unwrap();
            to_queue.enqueue(elem);
        }
        
        // 返回最后一个元素
        from_queue.dequeue()
    }
    
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}