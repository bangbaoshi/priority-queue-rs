# priority_queue

#### Description
Priority Queue is more specialized data structure than Queue. Like ordinary queue, priority queue has same method but with a major difference. In Priority queue items are ordered by key value so that item with the lowest value of key is at front and item with the highest value of key is at rear or vice versa. So we're assigned priority to item based on its key value. Lower the value, higher the priority. Following are the principal methods of a Priority Queue.

#### How To Use

```rust
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
```

#### License
This library is licensed under MIT license. See LICENSE for details.