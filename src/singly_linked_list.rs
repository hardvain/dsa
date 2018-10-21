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
        for _ in self.iter() {
            length = length + 1;
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
        let optional_node = self.node_at_mut(index - 1);
        let mut new_node = Node {
            value: t,
            next: None,
        };
        if index == 0 {
            new_node.next = self.head.take();
            self.head = Some(Box::new(new_node));
            return self;
        }
        optional_node.map(|node| {
            let current_next = &mut node.next;
            new_node.next = current_next.take();
            node.next = Some(Box::new(new_node));
        });
        self
    }

    pub fn add(&mut self, t: T) -> &mut Self {
        self.add_at(t, self.length())
    }

    pub fn add_all(&mut self, items: Vec<T>) -> &mut Self {
        for item in items {
            self.add(item);
        }
        self
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
impl<T> SinglyLinkedList<T> {
    pub fn iter(&self) -> SinglyLinkedListIterator<T> {
        let next = self.head.as_ref().map(|n| &**n);
        SinglyLinkedListIterator { next: next }
    }
}
pub struct SinglyLinkedListIteratorMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
impl<T> SinglyLinkedList<T> {
    pub fn iter_mut(&mut self) -> SinglyLinkedListIteratorMut<T> {
        let next = self.head.as_mut().map(|node| &mut **node);
        SinglyLinkedListIteratorMut { next: next }
    }
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

// Why should Debug constraint be placed here?
impl<T: Debug> From<Vec<T>> for SinglyLinkedList<T> {
    fn from(items: Vec<T>) -> Self {
        let mut sll: SinglyLinkedList<T> = SinglyLinkedList { head: None };
        for item in items {
            sll.add(item);
        }
        sll
    }
}
