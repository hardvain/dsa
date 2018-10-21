use std::fmt::Debug;

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}
#[derive(Debug)]
pub struct SinglyLinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}

impl<T: Debug> SinglyLinkedList<T> {
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

    pub fn add(&mut self, t: T) -> &mut Self {
        let new_node = Node {
            value: t,
            next: None,
        };
        let mut result = Some(Box::new(new_node));
        if self.head.is_none() {
            self.head = result;
            self
        } else {
            let mut current_node = &mut self.head;
            while current_node.is_some() {
                match current_node {
                    Some(node) => current_node = &mut node.next,
                    None => {}
                }
            }
            current_node = &mut result;
            self
        }
    }
}

impl<T: ToString> ToString for SinglyLinkedList<T> {
    fn to_string(&self) -> String {
        let mut vec: Vec<&T> = vec![];
        let mut current_node = &self.head;
        while current_node.is_some() {
            match current_node {
                Some(node) => {
                    vec.push(&node.value);
                    current_node = &node.next;
                }
                None => {}
            }
        }
        vec.into_iter().map(|i| i.to_string()).collect::<String>()
    }
}
