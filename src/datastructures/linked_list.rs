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

    pub fn get_index(&mut self, item: isize) -> Option<Rc<RefCell<Box<Node<T>>>>> {
        let i = 0;
        let mut ptr = Some(Rc::clone(self.head.as_ref().unwrap()));

        for _x in i..item - 1 {
            if item == 1 { return ptr }

            ptr = match ptr {
                Some(ref s) => {
                    let mut previous = (*s).borrow();
                    Some(Rc::clone(previous.next.as_ref().unwrap()))
                },
                None => return None,
            };
        }
        ptr
    }
}

#[cfg(test)]
mod tests {
    use datastructures::linked_list::LinkedList;

    #[test]
    fn addlast_get_index() {
        let mut list = LinkedList::new();

        list.addlast(5);
        list.addlast(3);
        list.addlast(9);
        list.addlast(10);
        list.addlast(77);

        println!("Count is: {}", list.count);

        let item1 = list.get_index(1);
        let item1 = item1.unwrap();
        let item1 = item1.borrow();

        let item2 = list.get_index(2);
        let item2 = item2.unwrap();
        let item2 = item2.borrow();

        let item3 = list.get_index(3);
        let item3 = item3.unwrap();
        let item3 = item3.borrow();

        let item4 = list.get_index(4);
        let item4 = item4.unwrap();
        let item4 = item4.borrow();

        let item5 = list.get_index(5);
        let item5 = item5.unwrap();
        let item5 = item5.borrow();

        assert_eq!((*item1).data, 5);
        assert_eq!((*item2).data, 3);
        assert_eq!((*item3).data, 9);
        assert_eq!((*item4).data, 10);
        assert_eq!((*item5).data, 77);
    }
}
