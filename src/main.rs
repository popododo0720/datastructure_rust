use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct Node<T> {
    pub val: T,
    pub next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t:T) -> Node<T> {
        Node {
            val: t,
            prev:None,
            next:None,
        }
    }
}

pub struct LinkedList<T> {
    pub length: u64,
    pub head: Option<NonNull<Node<T>>>,
    pub tail: Option<NonNull<Node<T>>>,
    // act like we own boxed nodes since we construct and leak them
    marker: PhantomData<Box<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
            marker: PhantomData,
        }
    }

    pub fn insert_at_head(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = self.head;
        node.prev = None;
        let node_ptr = NonNull::new(Box::into_raw(node));
        match self.head {
            None => self.tail = node_ptr,
            Some(head_ptr) => unsafe {
                (*head_ptr.as_ptr()).prev = node_ptr
            },
        }
        self.head = node_ptr;
        self.length += 1;
    }

    pub fn insert_at_tail(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.tail;
        let node_ptr = NonNull::new(Box::into_raw(node));
        match self.tail {
            None => self.head = node_ptr,
            Some(tail_ptr) => unsafe {
                (*tail_ptr.as_ptr()).next = node_ptr
            },
        }
        self.tail = node_ptr;
        self.length += 1;
    }

    pub fn insert_at_ith(&mut self, index: u64, obj: T) {
        if self.length < index {
            panic!("Index out of bounds");
        }

        if index == 0 || self.head.is_none() {
            self.insert_at_head(obj);
            return;
        }

        if index == self.length {
            self.insert_at_tail(obj);
            return;
        }
    
        if let Some(mut ith_node) = self.head {
            for _ in 0..index {
                unsafe {
                    match (*ith_node.as_ptr()).next {
                        None => panic!("Index out of bounds"),
                        Some(next_ptr) => ith_node = next_ptr,
                    }
                }
            }

            let mut node = Box::new(Node::new(obj));
            unsafe {
                node.prev = (*ith_node.as_ptr()).prev;
                node.next = Some(ith_node);
                if let Some(p) = (*ith_node.as_ptr()).prev {
                    let node_ptr = NonNull::new(Box::into_raw(node));
                    (*p.as_ptr()).next = node_ptr;
                    (*ith_node.as_ptr()).prev = node_ptr;
                    self.length += 1;
                }
            }
        }    
    }

    pub fn delete_head(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }

        self.head.map(|head_ptr| unsafe {
            let old_head = Box::from_raw(head_ptr.as_ptr());
            match old_head.next {
                Some(mut next_ptr) => next_ptr.as_mut().prev = None,
                None => self.tail = None,
            }
            self.head = old_head.next;
            self.length = self.length.checked_add_signed(-1).unwrap_or(0);
            old_head.val
        })
    }

    pub fn delete_tail(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }

        self.tail.map(|tail_ptr| unsafe {
            let old_tail = Box::from_raw(tail_ptr.as_ptr());
            match old_tail.prev {
                Some(mut prev_ptr) => prev_ptr.as_mut().next = None,
                None => self.tail = None,
            }
            self.tail = old_tail.prev;
            self.length -= 1;
            old_tail.val
        })
    }

}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.head {
            Some(node) => write!(f, "[{}]", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

fn main() {
    let mut testlist = LinkedList::<i64>::new();
    testlist.insert_at_head(10);
    testlist.insert_at_head(20);
    testlist.insert_at_head(30);
    println!("insert at head 3 : {}", testlist);

    testlist.insert_at_tail(40);
    testlist.insert_at_tail(50);
    testlist.insert_at_tail(60);
    println!("insert at tail 3 : {}", testlist);

    testlist.insert_at_ith(4, 1);
    println!("insert at 4th : {}", testlist);

    testlist.delete_head();
    println!("delete head : {}", testlist);

    
    testlist.delete_tail();
    println!("delete tail : {}", testlist);
}