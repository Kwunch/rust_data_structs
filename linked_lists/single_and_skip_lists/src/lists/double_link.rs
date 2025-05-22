use std::cell::RefCell;
use std::rc::Rc;
use crate::lists::list::{List, Node as TNode};

/*
    Decided to skip this implementation as I have already implemented a double linked list in Rust
    See my cdl_list
*/

#[derive(Debug, Clone)]
struct Node {
    value: String,
    next: Link,
    prev: Link,
}

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Clone)]
pub struct DoubleLinkList {
    head: Link,
    tail: Link,
    length: usize,
}

impl TNode for Node {
    type Item = String;

    fn new(item: Self::Item) -> Self {
        Self {
            value: item,
            next: None,
            prev: None,
        }
    }
}

impl List for DoubleLinkList {
    type Item = String;

    fn new() -> Self {
        todo!()
    }

    fn append(&mut self, item: Self::Item) {
        todo!()
    }

    fn pop(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

