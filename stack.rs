use std::{fmt::Display, io::{self, Write}};

struct Node<T: Display> {
    data: T,
    address: Option<Box<Node<T>>>,
}

impl<T: Display> Node<T> {
    #[warn(dead_code)]
    fn new(data: T) -> Node<T> {
        Node { data, address: None }
    }
}

struct Stack<T: Display> {
    head: Option<Box<Node<T>>>,
}

impl<T: Display> Stack<T> {
    fn new() -> Stack<T> {
        Stack { head: None }
    }

    fn push(&mut self, data: T) {
        // Create a new node with the given data
        let new_node = Box::new(Node {
            data,
            address: self.head.take(), // Link the current head to the new node
        });

        // Update the head to the new node
        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        // Take the current head
        if let Some(mut old_head) = self.head.take() {
            self.head = old_head.address.take(); // Update head to the next node
            Some(old_head.data) // Return the data of the old head
        } else {
            None
        }
    }

    fn display(&self) {
        let mut current = &self.head;
        while let Some(ref node) = current {
            print!("{} -> ", node.data);
            current = &node.address;
        }
        println!("None");
    }
}

fn main() {
    let mut stack = Stack::new();
    let mut input_buffer = String::new();

    loop {
        print!("Enter the element in the stack (-1 to stop) : ");
        io::stdout().flush().expect("Error while flushing stdout");

        input_buffer.clear();
        io::stdin().read_line(&mut input_buffer).expect("Error while reading input");

        match input_buffer.trim().parse::<i32>() {
            Ok(change) => {
                if change == -1 {
                    break;
                }
                stack.push(change);
                stack.display();
            }
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }

    loop {
        print!("Enter any key to POP action in stack (-1 to stop) : ");
        io::stdout().flush().expect("Error while flushing stdout");

        input_buffer.clear();
        io::stdin().read_line(&mut input_buffer).expect("Error while reading input");

        match input_buffer.trim().parse::<String>() {
            Ok(change) => {
                if change == "-1" {
                    break;
                }
                stack.pop();
                stack.display();
            }
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}
