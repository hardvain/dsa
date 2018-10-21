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
