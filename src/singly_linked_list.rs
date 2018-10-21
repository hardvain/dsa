pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}
pub struct SinglyLinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        SinglyLinkedList { head: None }
    }
    pub fn length(&self) -> i32 {
        let mut length = 0;
        let mut current_node = &self.head;
        while current_node.is_some() {
            match current_node {
                Some(node) => {
                    length = length + 1;
                    current_node = &(*node).next;
                }
                None => {}
            }
        }
        length
    }
}
