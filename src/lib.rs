struct PriorityQueueNode<T> where T: Sized {
    pub priority: u32,
    pub value: Option<T>,
}

impl<T> PriorityQueueNode<T> {
    pub fn new(priority: u32, value: T) -> PriorityQueueNode<T> {
        PriorityQueueNode {
            priority,
            value: Some(value),
        }
    }
}

impl<T> Drop for PriorityQueueNode<T> {
    fn drop(&mut self) {
        // println!("value :{} is destory", self.priority);
    }
}


struct PriorityQueue<T> where T: Sized {
    pub nodes: Vec<Option<Box<PriorityQueueNode<T>>>>,
}

impl<T> PriorityQueue<T> {
    pub fn new() -> PriorityQueue<T> {
        PriorityQueue {
            nodes: Vec::with_capacity(128),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.nodes.len() < 1 {
            return None;
        }
        let top = self.nodes[0].take();
        let mut max_pos = 0;
        let mut cur_pos = 0;

        let bottom_node = self.nodes.pop().unwrap();
        if self.nodes.len() < 1 {
            return top.unwrap().value.take();
        }

        self.nodes[max_pos] = bottom_node;
        let priority = self.nodes[max_pos].as_ref().unwrap().priority;
        let queue_len = self.nodes.len();

        loop {
            let left_child_pos = max_pos * 2 + 1;
            let right_child_pos = max_pos * 2 + 2;

            if left_child_pos < queue_len && right_child_pos < queue_len {
                let left_child_priority = self.nodes[left_child_pos].as_ref().unwrap().priority;
                let right_child_priority = self.nodes[right_child_pos].as_ref().unwrap().priority;
                if left_child_priority > right_child_priority {
                    max_pos = left_child_pos;
                } else {
                    max_pos = right_child_pos;
                }
            } else if left_child_pos < queue_len {
                let left_child_priority = self.nodes[left_child_pos].as_ref().unwrap().priority;
                if left_child_priority > priority {
                    max_pos = left_child_pos;
                }
            } else if right_child_pos < queue_len {
                let right_child_priority = self.nodes[right_child_pos].as_ref().unwrap().priority;
                if right_child_priority > priority {
                    max_pos = right_child_pos;
                }
            }
            if max_pos < 1 || cur_pos == max_pos {
                break;
            }
            if priority > self.nodes[max_pos].as_ref().unwrap().priority {
                break;
            }

            let cur_node = self.nodes[cur_pos].take();
            let max_node = self.nodes[max_pos].take();
            self.nodes[max_pos] = cur_node;
            self.nodes[cur_pos] = max_node;
            cur_pos = max_pos;
        }
        return top.unwrap().value.take();
    }

    pub fn peek(&mut self) -> Option<&mut T> {
        if self.nodes.len() < 1 {
            return None;
        }
        return self.nodes[0].as_mut().unwrap().value.as_mut();
    }

    pub fn push(&mut self, priority: u32, value: T) {
        if self.nodes.len() < 1 {
            self.nodes.push(Some(Box::new(PriorityQueueNode::new(priority, value))));
            return;
        }
        let mut compare_pos = (self.nodes.len() - 1) / 2;
        let new_node = PriorityQueueNode::new(priority, value);
        self.nodes.push(Some(Box::new(new_node)));
        let mut new_node_pos = self.nodes.len() - 1;
        loop {
            if priority > self.nodes[compare_pos].as_ref().unwrap().priority {
                let new_node = self.nodes[new_node_pos].take();
                self.nodes[new_node_pos] = self.nodes[compare_pos].take();
                self.nodes[compare_pos] = new_node;
            } else {
                break;
            }
            if compare_pos < 1 {
                break;
            }
            new_node_pos = compare_pos;
            compare_pos = (compare_pos - 1) / 2;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn main() {
        let mut queue = PriorityQueue::new();
        for priority in 10..10000 {
            queue.push(priority, String::from(format!("HelloWorld{}", priority)));
        }

        if let Some(t) = queue.peek() {
            println!("peek {}", t);
        }

        for priority in 0..10 {
            let value = queue.pop();
            if let Some(t) = value {
                println!("pop {}", t);
            }
        }

        if let Some(t) = queue.peek() {
            println!("peek {}", t);
        }
    }
}