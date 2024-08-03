use std::{collections::VecDeque, fmt::Display};


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

    fn BFS(&self){
        Self::BFS_Search(&self.root);
    }

    fn BFS_Search(node: &Option<Box<Node<T>>>)
    {
        let mut queue: VecDeque<&Node<T>> = VecDeque::new();
        if let Some(ref current_node) = node{
            queue.push_back(&current_node);
        }

        while let  Some(current_node)= queue.pop_front() {
            print!("{} ," ,current_node.data);

            if let Some(ref left_node) = current_node.left{
                queue.push_back(left_node);
            }
            if let Some(ref right_node) = current_node.right{
                queue.push_back(right_node);
            }
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
    println!();
    print!("BST of this tree :");
    root.BFS();
}