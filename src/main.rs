use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    /*
      - if a parent node is dropped so
        should its child nodes
      - not the other way around
      - this is a case for weak
        references
    */
    parent: RefCell<Weak<Node>>,
    /*
      - a node own its children
      - a node can also share that ownership
        : Vec<Rc<Node>>
      - and we want to modify the
        nodes that are children
        of another node: RefCell

    */
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    prevent_reference_cycles();
}

fn prevent_reference_cycles() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
