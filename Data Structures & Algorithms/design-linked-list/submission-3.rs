struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node { val, next: None }
    }
}

struct MyLinkedList {
    head: Option<Box<Node>>,
    size: i32,
}

impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList { head: None, size: 0 }
    }

    fn get(&self, index: i32) -> i32 {
        if index < 0 || index >= self.size {
            return -1;
        }
        let mut curr = &self.head;
        for _ in 0..index {
            curr = &curr.as_ref().unwrap().next;
        }
        curr.as_ref().unwrap().val
    }

    fn add_at_head(&mut self, val: i32) {
        let mut new_node = Box::new(Node::new(val));
        new_node.next = self.head.take();
        self.head = Some(new_node);
        self.size += 1;
    }

    fn add_at_tail(&mut self, val: i32) {
        if self.head.is_none() {
            self.add_at_head(val);
            return;
        }
        let mut curr = self.head.as_mut().unwrap();
        while curr.next.is_some() {
            curr = curr.next.as_mut().unwrap();
        }
        curr.next = Some(Box::new(Node::new(val)));
        self.size += 1;
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index > self.size {
            return;
        }
        if index <= 0 {
            self.add_at_head(val);
            return;
        }
        let mut curr = self.head.as_mut().unwrap();
        for _ in 0..index - 1 {
            curr = curr.next.as_mut().unwrap();
        }
        let mut new_node = Box::new(Node::new(val));
        new_node.next = curr.next.take();
        curr.next = Some(new_node);
        self.size += 1;
    }

    fn delete_at_index(&mut self, index: i32) {
        if index < 0 || index >= self.size {
            return;
        }
        if index == 0 {
            self.head = self.head.take().unwrap().next;
            self.size -= 1;
            return;
        }
        let mut curr = self.head.as_mut().unwrap();
        for _ in 0..index - 1 {
            curr = curr.next.as_mut().unwrap();
        }
        curr.next = curr.next.take().unwrap().next;
        self.size -= 1;
    }
}