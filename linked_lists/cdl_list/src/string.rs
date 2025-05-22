#[derive(Debug)]
pub struct String {
    char_array: Box<[u8]>,
    len: usize,
}

impl String {
    pub fn new(char_array: Box<[u8]>) -> Self {
        let len = char_array.len();
        String { char_array, len }
    }

    pub fn from_chars(chars: Vec<char>) -> Self {
        let mut char_array = vec![];
        for c in chars {
            char_array.push(c as u8);
        }
        let len = char_array.len();
        String {
            char_array: char_array.into_boxed_slice(),
            len,
        }
    }

    pub fn get_char_array(&self) -> &[u8] {
        &self.char_array
    }

    pub fn get_len(&self) -> usize {
        self.len
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        let mut char_array = vec![];
        for c in self.char_array.iter() {
            char_array.push(*c);
        }
        String {
            char_array: char_array.into_boxed_slice(),
            len: self.len,
        }
    }
}
