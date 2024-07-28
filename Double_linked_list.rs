use std::{cell::RefCell, fmt::Display, io::{self, Write}, rc::Rc};

#[derive(Clone)]
struct Node<T: Display> {
    data: T,
    pre: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Display> Node<T> 
{
    fn new(data: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            data,
            pre: None,
            next: None,
        }))
    }

    fn push_back(head: &Rc<RefCell<Node<T>>>, data: T) {
        let new_node = Node::new(data);
        let mut current = Rc::clone(head);
    
        loop {
            // Use a scoped block to ensure that `current_borrow` is dropped
            // before potentially reassigning `current`.
            let mut current_borrow = current.borrow_mut();
            if let Some(next_node) = current_borrow.next.clone() {
                drop(current_borrow); // Explicitly drop the borrow to release it
                current = next_node;
            } else {
                current_borrow.next = Some(Rc::clone(&new_node));
                new_node.borrow_mut().pre = Some(Rc::clone(&current));
                break;
            }
        }
    }

    fn print(head: &Rc<RefCell<Node<T>>>) 
    {
        println!();
        let mut current = Rc::clone(head);

        // The `while let` loop will continue as long as there is a `next` node.
        while let Some(next) = {
            let current_borrow = current.borrow(); // Borrow current node
            print!("{} -> ", current_borrow.data); // Print the data

            // Clone the `next` Rc if it exists, dropping the borrow immediately after
            current_borrow.next.clone()
        } {
            current = next; // Move to the next node
        }
        println!("None"); // Indicate the end of the list

    }

    
}

fn main() {
    let head = Node::new(10);
    let mut input_buffer = String::new();

    loop {
        print!("Enter the data (-1 to stop): ");
        io::stdout().flush().expect("Error flushing stdout");

        input_buffer.clear();
        io::stdin().read_line(&mut input_buffer).expect("Error reading line");

        match input_buffer.trim().parse::<i32>() {
            Ok(change) => {
                if change == -1 {
                    break;
                }
                Node::push_back(&head, change);
            }
            Err(_) => println!("Invalid input. Please enter a number."),
        }
        Node::print(&head);
    }
}
