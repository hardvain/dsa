use dsa::singly_linked_list::*;

#[test]
fn returns_correct_length() {
    let mut sll: SinglyLinkedList<i32> = SinglyLinkedList::new();
    assert_eq!(sll.length(), 0);

    sll.add(1);
    assert_eq!(sll.length(), 1);

    sll.add(2);
    assert_eq!(sll.length(), 2);
}

#[test]
fn add_at_works() {
    let mut sll: SinglyLinkedList<i32> = SinglyLinkedList::new();

    sll.add(1);
    sll.add(2);
    sll.add(3);
    sll.add(5);
    sll.add_at(4, 3);
    println!("{:?}", sll);
    assert_eq!(sll.length(), 5);
}

#[test]
fn get_element_at_works() {
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
