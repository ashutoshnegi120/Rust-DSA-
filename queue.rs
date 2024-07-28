use std::{fmt::Display, io::{self, Write}};

struct Node<T: Display> {
    data: T,
    address: Option<Box<Node<T>>>,
}

struct Queue<T: Display> {
    head: Option<Box<Node<T>>>,
}

impl<T: Display> Queue<T> {
    fn new() -> Queue<T> {
        Queue { head: None }
    }

    fn queue(&mut self, data: T) {
        let new_node = Some(Box::new(Node { data, address: None }));

        if let Some(ref mut head) = self.head {
            let mut current = head;
            while let Some(ref mut next) = current.address {
                current = next;
            }
            current.address = new_node;
        } else {
            self.head = new_node;
        }
    }

    fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.address;
            node.data
        })
    }

    fn first(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    fn display(&self) {
        let mut current = &self.head;

        if current.is_none() {
            println!("Queue is empty!");
        } else {
            while let Some(ref node) = current {
                print!("{} -> ", node.data);
                current = &node.address;
            }
            println!("None");
        }
    }
}

fn main() {
    let mut queue = Queue::new();
    let mut input_buffer = String::new();

    loop {
        print!("Enter the element in the queue (-1 to stop): ");
        io::stdout().flush().expect("Error while flushing stdout");

        input_buffer.clear();
        io::stdin().read_line(&mut input_buffer).expect("Error while reading input");

        match input_buffer.trim().parse::<i32>() {
            Ok(change) => {
                if change == -1 {
                    break;
                }
                queue.queue(change);
                queue.display();
            }
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }

    if let Some(first_element) = queue.first() {
        println!("First element in queue is: {}", first_element);
    } else {
        println!("Queue is empty!");
    }

    loop {
        print!("Enter any key to dequeue (type -1 to stop): ");
        io::stdout().flush().expect("Error while flushing stdout");

        input_buffer.clear();
        io::stdin().read_line(&mut input_buffer).expect("Error while reading input");

        match input_buffer.trim() {
            "-1" => break,
            _ => {
                if let Some(dequeued_element) = queue.dequeue() {
                    println!("Dequeued: {}", dequeued_element);
                } else {
                    println!("Queue is empty!");
                }
                queue.display();
            }
        }
    }
}
