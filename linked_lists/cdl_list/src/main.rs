use std::char;
use std::io;
use std::io::{Read, Write};

use linked_list::*;
use string::String;

use crate::vec::VecString;

mod linked_list;
mod string;
mod vec;

fn main() {
    let mut list = Dll::new();
    loop {
        // Take input from the user and put it in a char array
        print!("Enter CMD -> ");
        io::stdout().flush().unwrap();
        let input = get_input();

        let index = get_cmd_index(&input);

        // Get the command from the input
        let command = &input[..index];

        match command.to_ascii_lowercase().as_slice() {
            b"pop_front" => {
                let removed = list.pop(true);
                if removed.is_none() {
                    continue;
                } else {
                    print_removed(removed.unwrap());
                }
                list.view(true);
                list.view(false);
                continue;
            }
            b"pop_back" => {
                let removed = list.pop(false);
                if removed.is_none() {
                    continue;
                } else {
                    print_removed(removed.unwrap());
                }
                list.view(true);
                list.view(false);
                continue;
            }
            b"iter" => {
                iter(&list);
                continue;
            }
            b"exit" => {
                break;
            }
            b"push_front" | b"push_back" => {
                // Get the string from the input
                let input = input[index + 1..].to_vec();

                // Create a String struct from the input
                let string = String::new(input.into_boxed_slice());

                // Push the string to the linked list
                if command == b"push_front" {
                    list.push(string, true);
                } else if command == b"push_back" {
                    list.push(string, false);
                } else {
                    println!("Invalid command");
                }
            }
            _ => {}
        }

        // Print the linked list
        list.view(true);
        list.view(false);
    }
}

pub fn get_input() -> Vec<u8> {
    let mut input: Vec<u8> = vec![];

    loop {
        let mut buffer = [0; 1];
        io::stdin().read(&mut buffer).unwrap();
        let c = buffer[0] as char;
        if c == '\n' {
            break;
        }
        input.push(c as u8);
    }

    input
}

pub fn get_cmd_index(input: &Vec<u8>) -> usize {
    /*
        ** COMMANDS **
        - push_front -> push a string to the front of the linked list
        - push_back -> push a string to the back of the linked list
        - pop_front -> pop a string from the front of the linked list
        - pop_back -> pop a string from the back of the linked list
    */

    // Find index of first space in input
    let mut index = 0;
    for (i, &c) in input.iter().enumerate() {
        if c == b' ' {
            index = i;
            break;
        }
    }

    // If there is no space in the input, then the command is either a pop, iter or exit command
    if index == 0 {
        index = input.len();
    }

    index
}

pub fn iter(list: &Dll) {
    if list.is_empty() {
        println!("List is empty");
        return;
    }

    let mut current = list.get_head();

    loop {
        unsafe {
            print_current(current);

            list.view(true);

            print!("Enter a command [(n)ext/(p)rev/(e)xit]: ");
            io::stdout().flush().unwrap();
            let input = get_input();

            if input.eq_ignore_ascii_case(b"n") || input.eq_ignore_ascii_case(b"next") {
                current = (*current).get_next();
            } else if input.eq_ignore_ascii_case(b"p") || input.eq_ignore_ascii_case(b"prev") {
                current = (*current).get_prev();
            } else if input.eq_ignore_ascii_case(b"e") || input.eq_ignore_ascii_case(b"exit") {
                break;
            } else {
                println!("Invalid command");
            }
        }
    }
}

pub fn print_current(current: *mut VecString) {
    print!("Current Node -> ");
    io::stdout().flush().unwrap();

    unsafe {
        let string = (*current).get_string();
        let char_array = string.get_char_array();
        let len = string.get_len();
        for i in 0..len {
            print!("{}", char_array[i] as char);
        }
        print!(" ");
        io::stdout().flush().unwrap();

        if (*current).is_head() {
            print!("[Head]")
        }
        if (*current).is_tail() {
            print!("[Tail]")
        }
    }
    println!();
}

pub fn print_removed(removed: String) {
    print!("Removed -> ");
    io::stdout().flush().unwrap();

    let char_array = removed.get_char_array();
    let len = removed.get_len();
    for i in 0..len {
        print!("{}", char_array[i] as char);
    }
    println!();
}
