use crate::string::String;
use crate::vec::VecString;
use std::alloc::dealloc;
use std::io::Write;
use std::{io, ptr};

pub struct Dll {
    head: *mut VecString,
    tail: *mut VecString,
    length: usize,
}

impl Dll {
    pub fn new() -> Self {
        Dll {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            length: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn get_head(&self) -> *mut VecString {
        self.head
    }

    pub fn push(&mut self, string: String, push_front: bool) {
        let mut new_node = Box::new(VecString::new(string));

        unsafe {
            match self.length {
                0 => {
                    new_node.set_head(true);
                    new_node.set_tail(true);
                    self.head = Box::into_raw(new_node);
                    self.tail = self.head;
                    (*self.head).set_next(self.tail);
                    (*self.head).set_prev(self.tail);
                    (*self.tail).set_next(self.head);
                    (*self.tail).set_prev(self.head);
                    self.length += 1;
                }
                1 => {
                    if push_front {
                        new_node.set_head(true);
                        (*self.head).set_head(false);
                        self.head = Box::into_raw(new_node);
                    } else {
                        new_node.set_tail(true);
                        (*self.tail).set_tail(false);
                        self.tail = Box::into_raw(new_node);
                    }

                    (*self.head).set_next(self.tail);
                    (*self.head).set_prev(self.tail);
                    (*self.tail).set_next(self.head);
                    (*self.tail).set_prev(self.head);

                    self.length += 1;
                }
                _ => {
                    let node_ptr = Box::into_raw(new_node);
                    (*node_ptr).set_prev(self.tail);
                    (*node_ptr).set_next(self.head);
                    (*self.tail).set_next(node_ptr);
                    (*self.head).set_prev(node_ptr);

                    if push_front {
                        (*node_ptr).set_head(true);
                        (*self.head).set_head(false);
                        self.head = node_ptr;
                    } else {
                        (*node_ptr).set_tail(true);
                        (*self.tail).set_tail(false);
                        self.tail = node_ptr;
                    }

                    self.length += 1;
                }
            }
        }
    }

    pub fn pop(&mut self, pop_front: bool) -> Option<String> {
        match self.length {
            0 => {
                println!("Empty list");
                None
            }
            1 => unsafe {
                let temp = self.head;
                self.head = ptr::null_mut();
                self.tail = ptr::null_mut();
                self.length = 0;

                let string = (*temp).get_string().clone();

                ptr::drop_in_place(temp);
                dealloc(temp as *mut u8, std::alloc::Layout::new::<VecString>());

                Some(string)
            },
            _ => unsafe {
                let temp = if pop_front {
                    (*self.head).set_head(false);
                    let temp = self.head;
                    self.head = (*self.head).get_next();
                    (*self.head).set_head(true);
                    temp
                } else {
                    (*self.tail).set_tail(false);
                    let temp = self.tail;
                    self.tail = (*self.tail).get_prev();
                    (*self.tail).set_tail(true);
                    temp
                };

                (*self.tail).set_next(self.head);
                (*self.head).set_prev(self.tail);
                self.length -= 1;

                let string = (*temp).get_string().clone();

                ptr::drop_in_place(temp);
                dealloc(temp as *mut u8, std::alloc::Layout::new::<VecString>());

                Some(string)
            },
        }
    }

    pub fn view(&self, from_head: bool) {
        unsafe {
            let (mut current, end) = if from_head {
                print!("From Head -> ");
                (self.head, self.tail)
            } else {
                print!("From Tail -> ");
                (self.tail, self.head)
            };
            io::stdout().flush().unwrap();

            loop {
                let string = (*current).get_string();
                let char_array = &string.get_char_array();
                let len = string.get_len();
                for i in 0..len {
                    print!("{}", char_array[i] as char);
                }
                io::stdout().flush().unwrap();

                if current == end {
                    break;
                } else {
                    current = match from_head {
                        true => (*current).get_next(),
                        false => (*current).get_prev(),
                    };

                    print!(" -> ");
                    io::stdout().flush().unwrap();
                }
            }
        }
        println!();
    }
}
