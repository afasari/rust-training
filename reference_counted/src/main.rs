// use std::rc::Rc;

// fn main() {
//     let mut a = Rc::new(10);
//     let mut b = a.clone();

//     println!("a: {a}");
//     println!("b: {b}");
// }

use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i64,
    parent: Option<Weak<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

fn main() {
    let mut root = Rc::new(RefCell::new(Node {
        value: 42,
        parent: None,
        children: vec![],
    }));
    let child = Rc::new(RefCell::new(Node {
        value: 43,
        children: vec![],
        parent: Some(Rc::downgrade(&root))
    }));
    root.borrow_mut().children.push(child);

    println!("graph: {root:#?}");
}