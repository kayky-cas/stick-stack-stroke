use std::fmt::Display;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T, next: Link<T>) -> Self {
        Self { value, next }
    }
}

struct Stack<T> {
    root: Link<T>,
}

impl<T> Stack<T>
where
    T: Display + Copy,
{
    fn new() -> Self {
        Self { root: None }
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn push(&mut self, value: T) {
        self.root = Some(Box::new(Node::new(value, self.root)));
    }
}

fn main() {
    println!("Hello, world!");
}
