use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && {
            let self_children = self.children.borrow();
            let other_children = other.children.borrow();
            self_children.is_empty() && other_children.is_empty() ||
                self_children.len() == other_children.len() &&
                    self_children.iter().zip(other_children.iter()).all(|(f, s)| Self::eq(f, s))
        }
    }
}

#[test]
fn test() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    assert_eq!(None, leaf.parent.borrow().upgrade());
    assert_eq!(1, Rc::strong_count(&leaf));
    assert_eq!(0, Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        assert_eq!(1, Rc::strong_count(&branch));
        assert_eq!(1, Rc::weak_count(&branch));

        assert_eq!(2, Rc::strong_count(&leaf));
        assert_eq!(0, Rc::weak_count(&leaf));
    }

    assert_eq!(None, leaf.parent.borrow().upgrade());
    assert_eq!(1, Rc::strong_count(&leaf));
    assert_eq!(0, Rc::weak_count(&leaf));
}
