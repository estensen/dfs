struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn add_left(&mut self, value: T) {
        self.left = Some(Box::new(Node::new(value)));
    }

    fn add_right(&mut self, value: T) {
        self.right = Some(Box::new(Node::new(value)));
    }
}

fn generate_tree() -> Node<i32> {
    // Root
    let mut root = Node::new(1);

    // Children
    root.add_left(2);
    root.add_right(3);

    // Grandchildren
    if let Some(ref mut left) = root.left {
        left.add_left(4);
        left.add_right(5);
    }
    if let Some(ref mut right) = root.right {
        right.add_left(6);
        right.add_right(7);
    }

    // Great grandchildren
    if let Some(ref mut left) = root.left {
        if let Some(ref mut left_left) = left.left {
            left_left.add_left(8);
            left_left.add_right(9);
        }
    }
    if let Some(ref mut left) = root.left {
        if let Some(ref mut left_right) = left.right {
            left_right.add_left(10);
            left_right.add_right(11);
        }
    }
    // Unbalanced
    if let Some(ref mut right) = root.right {
        if let Some(ref mut right_left) = right.left {
            right_left.add_left(12);
            right_left.add_right(13);
        }
    }
    root
}

fn main() {
    _ = generate_tree();
}
