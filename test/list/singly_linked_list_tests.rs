use dsa::singly_linked_list::*;

#[test]
fn initial_length_is_0() {
    let mut sll: SinglyLinkedList<i32> = SinglyLinkedList::new();
    sll.add(1);
    sll.add(2);
    println!("{:?}", sll);
    assert_eq!(sll.length(), 2);
}
