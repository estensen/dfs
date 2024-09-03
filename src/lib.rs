use std::fmt::Debug;

pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Debug> Node<T> {
    pub fn new(value: T) -> Self {
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

    pub fn dfs(&self, visit: &mut impl FnMut(&T)) {
        visit(&self.value);

        // Recursively visit left child
        if let Some(ref left) = self.left {
            left.dfs(visit);
        }
        if let Some(ref right) = self.right {
            right.dfs(visit);
        }
    }

    pub fn pretty_print(&self) {
        let mut lines = vec![];
        self.build_string("", &mut lines, true);
        for line in lines {
            println!("{}", line);
        }
    }

    fn build_string(&self, prefix: &str, lines: &mut Vec<String>, is_tail: bool) {
        let mut line = String::new();
        if is_tail {
            line.push_str("└── ");
        } else {
            line.push_str("├── ");
        }
        line.push_str(&format!("{:?}", self.value));
        lines.push(format!("{}{}", prefix, line));

        let new_prefix = if is_tail { "    " } else { "│   " };
        if let Some(ref right) = self.right {
            right.build_string(&(prefix.to_owned() + new_prefix), lines, false);
        }
        if let Some(ref left) = self.left {
            left.build_string(&(prefix.to_owned() + new_prefix), lines, true);
        }
    }
}

pub fn generate_tree() -> Node<i32> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs() {
        let root = generate_tree();

        let mut visited = Vec::new();
        root.dfs(&mut |value| visited.push(*value));

        let expected_order = vec![1, 2, 4, 8, 9, 5, 10, 11, 3, 6, 12, 13, 7];
        assert_eq!(visited, expected_order, "DFS order mismatch");
    }
}
