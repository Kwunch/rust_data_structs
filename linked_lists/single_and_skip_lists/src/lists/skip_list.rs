use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    next: Vec<Link>,
    offset: usize,
    command: String,
}

impl Node {
    pub fn new(next: Vec<Link>, offset: usize, command: String) -> Self {
        Node {
            next,
            offset,
            command,
        }
    }
}

#[derive(Clone)]
struct SkipList {
    head: Link,
    tails: Vec<Link>,
    max_level: usize,
    length: usize,
}

impl SkipList {
    pub fn new(max_level: usize) -> Self {
        SkipList {
            head: None,
            tails: vec![],
            max_level,
            length: 0,
        }
    }
    
    pub fn append(&mut self, offset: usize, command: String) {
        let level = 1 + if self.head.is_none() {
            self.max_level
        } else {
            self.get_level()
        };
        
        let new = Rc::new(RefCell::new(Node::new(vec![None; level], offset, command)));
        
        // Update the tails for each level
        for i in 0..level {
            if let Some(old) = self.tails[i].take() {
                let next = &mut old.borrow_mut().next;
                next[i] = Some(new.clone())
            }
        }
        
        if self.head.is_none() {
            // First node in the list
            self.head = Some(new.clone());
        }
        self.length += 1;
    }
    
    pub fn find(&self, offset: usize) -> Option<String> {
        match self.head {
            Some(ref head) => {
                let mut start_level = self.max_level;
                let node = head.clone();
                let mut result = None;
                loop {
                    if node.borrow().next[start_level].is_some() {
                        break;
                    }
                    start_level -= 1;
                }
                let mut n = node;
                for level in (0..=start_level).rev() {
                    loop {
                        let next = n.clone();
                        match next.borrow().next[level] {
                            Some(ref next) if next.borrow().offset <= offset => {
                                n = next.clone();
                            }
                            _ => {
                                break;
                            }
                        };
                    }
                    if n.borrow().offset == offset {
                        let tmp = n.borrow();
                        result = Some(tmp.command.clone());
                        break;
                    }
                }
                result
            }
            None => None,
        }
    }
    
    fn get_level(&self) -> usize {
        let mut n = 0;
        while rand::random::<bool>() && n < self.max_level {
            n += 1;
        }
        n
    }
}