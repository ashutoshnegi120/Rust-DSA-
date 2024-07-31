 
use std::fmt::Display;


struct Node<T: Display> {
    left: Option<Box<Node<T>>>,
    data: T,
    right: Option<Box<Node<T>>>,
}

struct BST<T: Display> {
    root: Option<Box<Node<T>>>,
}

impl<T: Display+PartialOrd> BST<T> {
    fn new() -> Self {
        BST { root: None }
    }

    fn insertion(&mut self, data: T) {
        self.root = Self::insert_node(self.root.take(), data);
    }

    fn insert_node(node: Option<Box<Node<T>>>, data: T) -> Option<Box<Node<T>>> {
        match node {
            Some(mut current_node) => {
                if data < current_node.data {
                    current_node.left = Self::insert_node(current_node.left.take(), data);
                } else {
                    current_node.right = Self::insert_node(current_node.right.take(), data);
                }
                Some(current_node)
            }
            None => Some(Box::new(Node {
                left: None,
                data,
                right: None,
            })),
        }
    }

    fn print(&self) {
        Self::print_node(&self.root);
    }

    fn print_node(node: &Option<Box<Node<T>>>) {
        if let Some(ref current_node) = node {
            Self::print_node(&current_node.left);
            println!("{}", current_node.data);
            Self::print_node(&current_node.right);
        }
    }

}

fn main(){
    let mut root = BST::new();

    root.insertion(10);
    root.insertion(15);
    root.insertion(1);
    root.insertion(5);
    root.insertion(20);

    root.print();
}