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

    pub fn add_at(&mut self, t: T, index: i32) -> &mut Self {
        if index > self.length() {
            return self;
        } else {
            let mut counter = index;
            let mut current_node = &mut self.head;
            while counter != 0 {
                if current_node.is_some() {
                    match current_node {
                        Some(node) => {
                            current_node = &mut node.next;
                            counter = counter - 1;
                        }
                        None => {}
                    }
                }
            }

            match current_node {
                Some(node) => {
                    let current_next = node.next.take();
                    let new_node = Node {
                        value: t,
                        next: current_next,
                    };
                    node.next = Some(Box::new(new_node));
                }
                None => {}
            };

            self
        }
    }

    pub fn add(&mut self, t: T) -> &mut Self {
        let new_node = Node {
            value: t,
            next: None,
        };
        let result = Some(Box::new(new_node));
        if self.head.is_none() {
            self.head = result;
            return self;
        }
        let mut current_node = &mut self.head;
        while current_node.is_some() {
            match current_node {
                Some(node) => {
                    if node.next.is_some() {
                        current_node = &mut node.next;
                        continue;
                    } else {
                        node.next = result;
                        break;
                    }
                }
                None => {}
            }
        }
        self
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
