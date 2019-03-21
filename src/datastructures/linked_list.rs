use std::rc::Rc;
use std::boxed::Box;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Rc<RefCell<Box<Node<T>>>>>,
}

impl<T> Node<T> {
    fn new(input: T) -> RefCell<Box<Node<T>>> {
        RefCell::new(Box::new(Node {
            data: input,
            next: None,
        }))
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Box<Node<T>>>>>,
    tail: Option<Rc<RefCell<Box<Node<T>>>>>,
    count: isize,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            count: 0,
        }
    }

    pub fn addlast(&mut self, input: T) {
        let newnode: Rc<RefCell<Box<Node<T>>>> = Rc::new(Node::new(input));

        if self.count == 0 {
            self.tail = Some(Rc::clone(&newnode));
            self.head = Some(Rc::clone(&newnode));
            self.count += 1
        } else {
            match &mut self.tail {
                Some(ref mut s) => {
                    let mut oldnode = (*s).borrow_mut();
                    (*oldnode).next = Some(Rc::clone(&newnode));
                },
                None => (),
            }
            self.tail = Some(Rc::clone(&newnode));
            self.count += 1;
        }
    }

    pub fn get_item(&mut self, item: u32) -> Option<Rc<RefCell<Box<Node<T>>>>> {
        let i = 0;
        let mut ptr = None;

        for _x in i..item {
            ptr = match & self.head {
                Some(ref s) => {
                    Some(Rc::clone(s))
                },
                None => return None,
            };
        }
        ptr
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn addlast_get_item() {
        LinkedList list = new LinkedList();
        list.addlast(5);
        list.addlast(3);
        list.addlast(9);
        list.addlast(10);
        list.addlast(77);

        let item1 = list.get_item(1);
        let item1 = a.unwrap();
        let item1 = a.borrow();

        assert_eq!((*item1).data, 5);
    }
}
