use linked_list::double_list::List as List3;
use linked_list::good_list::List;
use linked_list::list_imutable::List as List2;
use linked_list::three::Three;

fn _test_stack() {
    let mut stack = List::new();

    assert_eq!(stack.pop(), None);

    stack.push(1);
    stack.push(2);
    stack.push(3);

    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));

    stack.peek_mut().map(|v| {
        *v = 2;
    });

    println!("{}", stack.peek().unwrap());
}

fn _test_list_2() {
    let list = List2::new();
    assert_eq!(list.head(), None);

    let list = list.prepend(1).prepend(2).prepend(3);
    assert_eq!(list.head(), Some(&3));

    let list = list.tail();
    assert_eq!(list.head(), Some(&2));

    let list = list.tail();
    assert_eq!(list.head(), Some(&1));

    let list = list.tail();
    assert_eq!(list.head(), None);

    // Make sure empty tail works
    let list = list.tail();
    assert_eq!(list.head(), None);
}

fn _test_double_list() {
    let mut list = List3::new();

    // Check empty list behaves right
    assert_eq!(list.pop_front(), None);

    // Populate list
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    // Check normal removal
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push_front(4);
    list.push_front(5);

    // Check normal removal
    assert_eq!(list.pop_front(), Some(5));
    assert_eq!(list.pop_front(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);
}

fn test_three() {
    let mut three = Three::new();

    three.add(1);
    three.add(2);
    three.add(2);
    three.add(0);

    three.print();
}

fn main() {
    test_three()
}
