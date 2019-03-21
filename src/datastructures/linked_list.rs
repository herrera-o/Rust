use std::rc::Rc;
use std::boxed::Box;
use std::cell::RefCell;

#[derive(debug)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Rc<Refcell<Box<Node<T>>>>>,
}

impl<T> Node<T> {
    fn new(input: T) -> Refcell<Box<Node<T>>> {
        RefCell::new(Box::new(Node {
            data: input,
            next: None,
        }))
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Rc<Refcell<Box<Node<T>>>>>,
    tail: Option<Rc<Refcell<Box<Node<T>>>>>,
    count: isize,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            count: 0;
        }
    }
}
