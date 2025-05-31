struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

struct MyLinkedList {
    head: Option<Box<Node>>,
    tail: Option<*mut Node>,
}

impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList {
            head: None,
            tail: None,
        }
    }

    fn get(&self, index: i32) -> i32 {
        let mut current = &self.head;
        let mut i = 0;

        while let Some(node) = current {
            if i == index {
                return node.val;
            }
            current = &node.next;
            i += 1;
        }

        -1
    }

    fn add_at_head(&mut self, val: i32) {
        let mut new_node = Box::new(Node {
            val,
            next: self.head.take(),
        });

        let raw: *mut _ = &mut *new_node;
        if self.tail.is_none() {
            self.tail = Some(raw);
        }

        self.head = Some(new_node);
    }

    fn add_at_tail(&mut self, val: i32) {
        let mut new_node = Box::new(Node { val, next: None });
        let raw: *mut _ = &mut *new_node;

        if let Some(tail_ptr) = self.tail {
            unsafe {
                (*tail_ptr).next = Some(new_node);
            }
            self.tail = Some(raw);
        } else {
            self.add_at_head(val);
        }
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index == 0 {
            self.add_at_head(val);
            return;
        }

        let mut current = &mut self.head;
        let mut i = 0;

        while let Some(node) = current {
            if i + 1 == index {
                let new_node = Box::new(Node {
                    val,
                    next: node.next.take(),
                });
                node.next = Some(new_node);

                if node.next.as_ref().unwrap().next.is_none() {
                    let raw: *mut _ = &mut **node.next.as_mut().unwrap();
                    self.tail = Some(raw);
                }
                return;
            }
            current = &mut node.next;
            i += 1;
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if index == 0 {
            if let Some(mut head_node) = self.head.take() {
                self.head = head_node.next.take();

                if self.head.is_none() {
                    self.tail = None;
                }
            }
            return;
        }

        let mut current = &mut self.head;
        let mut i = 0;

        while let Some(node) = current {
            if i + 1 == index {
                if let Some(mut to_delete) = node.next.take() {
                    node.next = to_delete.next.take();

                    if node.next.is_none() {
                        let raw: *mut _ = &mut **node;
                        self.tail = Some(raw);
                    }
                }
                return;
            }
            current = &mut node.next;
            i += 1;
        }
    }
}

