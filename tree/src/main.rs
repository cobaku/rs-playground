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
        let current = self.value.as_ref().unwrap();
        if current.as_ref().cmp(value.as_ref()).is_eq() {
            return;
        }
        if current.as_ref().cmp(value.as_ref()).is_gt() {
            if self.left.is_some() {
                self.left.as_mut().unwrap().add(value);
                return;
            }
            self.left.replace(Box::new(Node::new(value)));
            return;
        }
        if current.as_ref().cmp(value.as_ref()).is_lt() {
            if self.right.is_some() {
                self.right.as_mut().unwrap().add(value);
                return;
            }
            self.right.replace(Box::new(Node::new(value)));
        }
    }
}

fn main() {
    let mut tree = Node::new(Box::new(35));

    tree.add(Box::new(33));
    tree.add(Box::new(36));
    tree.add(Box::new(15));
    tree.add(Box::new(45));

    println!("{:?}", tree)
}
