use dsa::singly_linked_list::*;

#[test]
fn length_works() {
    let mut sll: SinglyLinkedList<i32> = SinglyLinkedList::new();
    assert_eq!(sll.length(), 0);

    sll.add(1);
    assert_eq!(sll.length(), 1);

    sll.add(2);
    assert_eq!(sll.length(), 2);
}

#[test]
fn element_at_works() {
    let mut sll: SinglyLinkedList<i32> = SinglyLinkedList::new();
    assert_eq!(sll.element_at(0), None);
    sll.add(10);
    sll.add(21);
    sll.add(33);
    sll.add(50);
    assert_eq!(sll.element_at(0), Some(&10));
    assert_eq!(sll.element_at(1), Some(&21));
    assert_eq!(sll.element_at(2), Some(&33));
    assert_eq!(sll.element_at(3), Some(&50));
}

#[test]
fn get_element_at_mut_works() {
    let mut sll: SinglyLinkedList<i32> = SinglyLinkedList::new();
    assert_eq!(sll.element_at_mut(0), None);
    sll.add(10);
    sll.add(21);
    sll.add(33);
    sll.add(50);
    assert_eq!(sll.element_at_mut(0), Some(&mut 10));
    assert_eq!(sll.element_at_mut(1), Some(&mut 21));
    assert_eq!(sll.element_at_mut(2), Some(&mut 33));
    assert_eq!(sll.element_at_mut(3), Some(&mut 50));
}

#[test]
fn add_and_add_at_works() {
    let mut sll: SinglyLinkedList<i32> = vec![1, 2, 3, 5].into();
    sll.add_at(4, 0);
    assert_eq!(sll.length(), 5);
    assert_eq!(sll.to_string(), "41235");
}

#[test]
fn add_all_works() {
    let mut sll: SinglyLinkedList<i32> = SinglyLinkedList::new();
    let items = vec![1, 2, 3, 5];
    sll.add_all(items);
    sll.add_at(4, 0);
    assert_eq!(sll.length(), 5);
    assert_eq!(sll.to_string(), "41235");
}

#[test]
fn add_first_works() {
    let mut sll: SinglyLinkedList<i32> = SinglyLinkedList::new();
    sll.add_first(9);
    assert_eq!(sll.length(), 1);
    assert_eq!(sll.to_string(), "9");
    let items = vec![1, 2, 3, 5];
    sll.add_all(items);
    assert_eq!(sll.length(), 5);
    assert_eq!(sll.to_string(), "91235");
}

#[test]
fn add_last_works() {
    let mut sll: SinglyLinkedList<i32> = SinglyLinkedList::new();
    sll.add_last(9);
    assert_eq!(sll.length(), 1);
    assert_eq!(sll.to_string(), "9");
    let items = vec![1, 2, 3, 5];
    sll.add_all(items);
    assert_eq!(sll.length(), 5);
    assert_eq!(sll.to_string(), "91235");
}

#[test]
fn clear_works() {
    let mut sll: SinglyLinkedList<i32> = SinglyLinkedList::new();
    let items = vec![1, 2, 3, 5];
    sll.add_all(items);
    assert_eq!(sll.length(), 4);
    sll.clear();
    assert_eq!(sll.length(), 0);
    assert_eq!(sll.to_string(), "");
}

#[test]
fn contains_works() {
    let mut sll: SinglyLinkedList<i32> = SinglyLinkedList::new();
    let items = vec![1, 2, 3, 5];
    sll.add_all(items);
    assert_eq!(sll.contains(2), true);
    assert_eq!(sll.contains(20), false);
}

#[test]
fn get_first_works() {
    let mut sll: SinglyLinkedList<i32> = SinglyLinkedList::new();
    assert_eq!(sll.get_first(), None);
    let items = vec![1, 2, 3, 5];
    sll.add_all(items);
    assert_eq!(sll.get_first(), Some(&1));
}

#[test]
fn get_last_works() {
    let mut sll: SinglyLinkedList<i32> = SinglyLinkedList::new();
    assert_eq!(sll.get_last(), None);
    let items = vec![1, 2, 3, 5];
    sll.add_all(items);
    assert_eq!(sll.get_last(), Some(&5));
}

#[test]
fn index_of_works() {
    let mut sll: SinglyLinkedList<i32> = SinglyLinkedList::new();
    assert_eq!(sll.index_of(10), None);
    let items = vec![1, 2, 3, 5];
    sll.add_all(items);
    assert_eq!(sll.index_of(2), Some(1));
}

#[test]
fn last_index_of_works() {
    let mut sll: SinglyLinkedList<i32> = SinglyLinkedList::new();
    assert_eq!(sll.last_index_of(10), None);
    let items = vec![1, 2, 3, 5, 2];
    sll.add_all(items);
    assert_eq!(sll.last_index_of(2), Some(4));
}
