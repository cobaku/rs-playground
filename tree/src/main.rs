use core::cmp::Ordering::{Greater, Less};
use std::fmt::Display;

type TreeNode<T> = Option<Box<Node<T>>>;

struct Node<T: Ord + Display> {
    value: Option<Box<T>>,
    left: TreeNode<T>,
    right: TreeNode<T>,
}

impl<T: Ord + Display> Node<T> {
    fn new(value: Box<T>) -> Self {
        Node {
            value: Some(value),
            right: None,
            left: None,
        }
    }

    fn add(self: &mut Self, value: Box<T>) {
        let current = self.value.as_ref()
            .unwrap();
        match current.as_ref().cmp(value.as_ref()) {
            Less { .. } => {
                match self.right {
                    Some { .. } => { self.right.as_mut().unwrap().add(value); }
                    None {} => { self.right.replace(Box::new(Node::new(value))); }
                }
            }
            Greater { .. } => {
                match self.left {
                    Some { .. } => { self.left.as_mut().unwrap().add(value); }
                    None {} => { self.left.replace(Box::new(Node::new(value))); }
                }
            }
            _ => {}
        }
    }

    fn print(self: &Self) {
        fn print_inner<T: Display + Ord>(node: &Node<T>, indention: usize) {
            if node.left.as_ref().is_some() {
                print_inner(node.left.as_ref().unwrap().as_ref(), indention + 1);
            }
            println!("{} {}", "  ".repeat(indention), node.value.as_ref().unwrap().as_ref());
            if node.right.as_ref().is_some() {
                print_inner(node.right.as_ref().unwrap().as_ref(), indention + 1);
            }
        }
        print_inner(self, 0)
    }
}

fn main() {
    let mut tree = Node::new(Box::new(5));
    for i in 0..10 {
        tree.add(Box::new(i));
    }
    tree.print()
}
