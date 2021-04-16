use core::mem;
use core::ptr;

pub struct PriorityQueue<T> where T: Sized {
    nodes: Vec<*mut PriorityQueueNode<T>>,
}


impl<T> PriorityQueue<T> where T: Sized {
    pub fn new() -> PriorityQueue<T> {
        PriorityQueue {
            nodes: Vec::with_capacity(128),
        }
    }

    pub fn push(&mut self, priority: i32, value: T) {
        let mut node = Box::new(PriorityQueueNode::new(value, priority));
        self.nodes.push(node.as_mut() as *mut PriorityQueueNode<T>);
        mem::forget(node);
        if self.nodes.len() < 2 {
            return;
        }
        let mut cur_pos = self.nodes.len() - 1;
        let mut pos = cur_pos / 2;
        loop {
            unsafe {
                if (*self.nodes[pos]).priority < priority {
                    let temp_ptr = self.nodes[pos];
                    self.nodes[pos] = self.nodes[cur_pos];
                    self.nodes[cur_pos] = temp_ptr;
                } else {
                    break;
                }
            }
            if pos < 1 {
                break;
            }
            cur_pos = pos;
            pos = (pos - 1) / 2;
        }
    }

    pub fn peek(&mut self) -> Option<&T> {
        if self.nodes.len() < 1 {
            return None;
        }
        let mut res = None;
        unsafe {
            let mut node_ptr = (*self.nodes[0]).value;
            if let Some(t) = node_ptr {
                res = Some(& *t);
            }
        }
        return res;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.nodes.len() < 1 {
            return None;
        }
        unsafe {
            let mut res = None;
            let mut node_ptr = (*self.nodes[0]).value;
            if let Some(t) = node_ptr {
                Box::from_raw(self.nodes[0]);
                res = Some(ptr::read(t));
            }

            if self.nodes.len() == 1 {
                self.nodes.pop();
                return res;
            }
            let mut cur_pos = 0;
            let len = self.nodes.len() - 1;
            self.nodes[cur_pos] = self.nodes[len];
            loop {
                unsafe {
                    let mut left = cur_pos * 2 + 1;
                    let mut right = cur_pos * 2 + 2;

                    let mut max_pos = cur_pos;
                    if left < len && right < len {
                        if (*self.nodes[left]).priority > (*self.nodes[right]).priority {
                            max_pos = left;
                        } else {
                            max_pos = right;
                        }
                    } else if left < len && (*self.nodes[left]).priority > (*self.nodes[max_pos]).priority {
                        max_pos = left;
                    } else if right < len && (*self.nodes[right]).priority > (*self.nodes[max_pos]).priority {
                        max_pos = right;
                    }
                    if max_pos == cur_pos {
                        break;
                    }
                    if (*self.nodes[cur_pos]).priority > (*self.nodes[max_pos]).priority {
                        break;
                    }
                    let mut ptr = self.nodes[max_pos];
                    self.nodes[max_pos] = self.nodes[cur_pos];
                    self.nodes[cur_pos] = ptr;
                    cur_pos = max_pos;
                }
            }
            self.nodes.pop();
            return res;
        }
        return None;
    }
}

struct PriorityQueueNode<T> where T: Sized {
    value: Option<*mut T>,
    priority: i32,
}

impl<T> PriorityQueueNode<T> {
    fn new(value: T, priority: i32) -> PriorityQueueNode<T> {
        let mut realValue = Box::new(value);

        let node = PriorityQueueNode {
            value: Some(realValue.as_mut() as *mut T),
            priority,
        };
        mem::forget(realValue);
        return node;
    }
}

impl<T> Drop for PriorityQueueNode<T> {
    fn drop(&mut self) {
        // println!("drop {}", self.priority);
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