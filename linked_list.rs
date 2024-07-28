use std::io::{self, Write};

struct Node<T: std::fmt::Display> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Display> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data,
            next: None,
        }
    }

    fn push_back(&mut self, data: T) {
        let mut temp = self;
        while let Some(ref mut next) = temp.next {
            temp = next;
        }
        temp.next = Some(Box::new(Node::new(data)));
    }

    fn del(&mut self, index: usize) {
        if index == 0 {
            // Handle removing the head node
            if let Some(next) = self.next.take() {
                *self = *next;
            }
        } else {
            let mut current = self;
            let mut current_index = 0;

            while let Some(mut next) = current.next.take() {
                if current_index == index - 1 {
                    current.next = next.next.take();
                    break;
                }
                current.next = Some(next);
                current = current.next.as_mut().unwrap(); // Move to the next node
                current_index += 1;
            }
        }
    }

    fn print_list(&self) {
        let mut current = self;
        while let Some(next) = &current.next {
            print!("{} -> ", current.data);
            current = next;
        }
        print!("{}", current.data); // Print the last element without "->"
    }
}

fn main() {
    let mut head = Node::new(10); // Initialize with an integer
    let mut input_buffer = String::new();

    loop {
        print!("Enter a number (-1 to stop): ");
        io::stdout().flush().expect("Failed to flush stdout");

        input_buffer.clear(); // Clear the buffer for new input
        io::stdin().read_line(&mut input_buffer)
            .expect("Failed to read from stdin");

        let input_trimmed = input_buffer.trim();

        if input_trimmed == "-1" {
            break;
        }

        match input_trimmed.parse::<i32>() {
            Ok(num) => head.push_back(num),
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }

    println!("Linked list before deletion:");
    head.print_list();

    print!("Enter index to delete (-1 to skip): ");
    io::stdout().flush().expect("Failed to flush stdout");

    input_buffer.clear(); // Clear the buffer for new input
    io::stdin().read_line(&mut input_buffer)
        .expect("Failed to read from stdin");

    let input_trimmed = input_buffer.trim();
    if let Ok(index) = input_trimmed.parse::<usize>() {
        head.del(index);
    }

    println!("Linked list after deletion:");
    head.print_list();
}
