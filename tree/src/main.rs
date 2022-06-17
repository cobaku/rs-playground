use core::cmp::Ordering::{Greater, Less};

type TreeNode<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T: Ord> {
    value: Option<Box<T>>,
    left: TreeNode<T>,
    right: TreeNode<T>,
}

impl<T: Ord> Node<T> {
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
}

fn main() {
    let mut tree = Node::new(Box::new(35));

    tree.add(Box::new(33));
    tree.add(Box::new(36));
    tree.add(Box::new(15));
    tree.add(Box::new(45));
    tree.add(Box::new(1));

    println!("{:?}", tree)
}
