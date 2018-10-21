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
            let node = current_node.as_ref().unwrap();
            length = length + 1;
            current_node = &node.next;
        }
        length
    }

    pub fn element_at(&self, index: i32) -> Option<&T> {
        let opt_node = self.node_at(index);
        opt_node.map(|n| &n.value)
    }

    pub fn element_at_mut(&mut self, index: i32) -> Option<&mut T> {
        let mut_opt_node = self.node_at_mut(index);
        mut_opt_node.map(|n| &mut n.value)
    }

    pub fn add_at(&mut self, t: T, index: i32) -> &mut Self {
        let optional_node = self.node_at_mut(index);
        optional_node.map(|node| {
            let current_next = &mut node.next;
            let new_node = Node {
                value: t,
                next: current_next.take(),
            };
            node.next = Some(Box::new(new_node));
        });
        self
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

    pub fn iter(&self) -> SinglyLinkedListIterator<T> {
        let next = self.head.as_ref().map(|n| &**n);
        SinglyLinkedListIterator { next: next }
    }

    pub fn iter_mut(&mut self) -> SinglyLinkedListIteratorMut<T> {
        let next = self.head.as_mut().map(|node| &mut **node);
        SinglyLinkedListIteratorMut { next: next }
    }

    fn node_at_mut(&mut self, index: i32) -> Option<&mut Node<T>> {
        let mut current_node = &mut self.head;
        let mut counter = index;
        while counter != 0 && current_node.is_some() {
            let current_node_value = current_node.as_mut().unwrap();
            current_node = &mut current_node_value.next;
            counter = counter - 1;
        }
        match current_node {
            Some(node) => Some(&mut *node),
            None => None,
        }
    }

    fn node_at(&self, index: i32) -> Option<&Node<T>> {
        let mut current_node = &self.head;
        let mut counter = index;
        while counter != 0 && current_node.is_some() {
            let current_node_value = current_node.as_ref().unwrap();
            current_node = &current_node_value.next;
            counter = counter - 1;
        }
        match current_node {
            Some(node) => Some(node),
            None => None,
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

pub struct SinglyLinkedListIterator<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct SinglyLinkedListIteratorMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for SinglyLinkedListIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.value
        })
    }
}

impl<'a, T> Iterator for SinglyLinkedListIteratorMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.value
        })
    }
}
