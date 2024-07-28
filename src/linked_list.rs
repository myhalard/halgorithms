pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None }
    }

    pub fn insert(&mut self, item: T) {
        let next = self.head.take();
        self.head = Some(Box::new(Node {item, next}));
    }

    pub fn remove(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            Some(node.item)
        }
        else {
            None
        }
    }
}