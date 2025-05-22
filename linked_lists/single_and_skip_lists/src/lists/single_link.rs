use std::cell::RefCell;
use std::rc::Rc;
use crate::lists::list::List;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    value: String,
    next: Link,
}

impl Node {
    fn new(value: String) -> Self {
        Node {
            value,
            next: None,
        }
    }
}

struct SingleLinkedList {
    head: Link,
    tail: Link,
    length: usize,
}

impl List for SingleLinkedList {
    type Item = String;
    fn new() -> Self {
        SingleLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }
    
    fn append(&mut self, value: Self::Item) {
        let node = Rc::new(RefCell::new(Node::new(value)));
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(node.clone()),
            None => self.head = Some(node.clone())
        };
        self.length += 1;
    }
    
    fn pop(&mut self) -> Option<Self::Item> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Failed to unwrap head")
                .into_inner()
                .value
        })
    }
}

