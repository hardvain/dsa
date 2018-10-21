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
    sll.add_at(4, 2);
    println!("{:?}", sll);
    assert_eq!(sll.length(), 5);
}
