use dsa::singly_linked_list::*;

#[test]
fn initial_length_is_0() {
    let sll: SinglyLinkedList<i32> = SinglyLinkedList::new();
    assert_eq!(sll.length(), 0);
}
