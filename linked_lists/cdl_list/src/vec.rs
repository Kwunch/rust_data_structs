use crate::string::String;

pub struct VecString {
    next: *mut VecString,
    prev: *mut VecString,
    string: String,
    is_head: bool,
    is_tail: bool,
}

impl VecString {
    pub fn new(string: String) -> VecString {
        VecString {
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
            string,
            is_head: false,
            is_tail: false,
        }
    }

    pub fn get_next(&self) -> *mut VecString {
        self.next
    }

    pub fn get_prev(&self) -> *mut VecString {
        self.prev
    }

    pub fn set_next(&mut self, next: *mut VecString) {
        self.next = next;
    }

    pub fn set_prev(&mut self, prev: *mut VecString) {
        self.prev = prev;
    }

    pub fn get_string(&self) -> &String {
        &self.string
    }

    pub fn is_head(&self) -> bool {
        self.is_head
    }

    pub fn is_tail(&self) -> bool {
        self.is_tail
    }

    pub fn set_head(&mut self, is_head: bool) {
        self.is_head = is_head;
    }

    pub fn set_tail(&mut self, is_tail: bool) {
        self.is_tail = is_tail;
    }
}
