use std::{marker::PhantomData, ptr::NonNull};

mod pre_implemented;
#[allow(unused)]
pub struct Node<T> {
    data: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

#[allow(unused)]
impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data: (data),
            next: (None),
            prev: (None),
        }
    }

    fn into_data(self: Box<Self>) -> T {
        self.data
    }
}

#[allow(unused)]
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub const fn new() -> Self {
        LinkedList {
            head: (None),
            tail: (None),
            len: (0),
            marker: (PhantomData),
        }
    }

    /// You may be wondering why it's necessary to have is_empty()
    /// when it can easily be determined from len().
    /// It's good custom to have both because len() can be expensive for some types,
    /// whereas is_empty() is almost always cheap.
    /// (Also ask yourself whether len() is expensive for LinkedList)
    ///
    /// Because just checking whether the head has null value is enough here
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    unsafe fn unlink_node(&mut self, mut node: NonNull<Node<T>>) {
        let node = unsafe { node.as_mut() }; // this one is ours now, we can create an &mut.

        // Not creating new mutable (unique!) references overlapping `element`.
        match node.prev {
            Some(prev) => unsafe { (*prev.as_ptr()).next = node.next },
            // this node is the head node
            None => self.head = node.next,
        };

        match node.next {
            Some(next) => unsafe { (*next.as_ptr()).prev = node.prev },
            // this node is the tail node
            None => self.tail = node.prev,
        };

        self.len -= 1;
    }

    #[inline]
    unsafe fn splice_nodes(
        &mut self,
        existing_prev: Option<NonNull<Node<T>>>,
        existing_next: Option<NonNull<Node<T>>>,
        mut splice_start: NonNull<Node<T>>,
        mut splice_end: NonNull<Node<T>>,
        splice_length: usize,
    ) {
        // This method takes care not to create multiple mutable references to whole nodes at the same time,
        // to maintain validity of aliasing pointers into `element`.
        if let Some(mut existing_prev) = existing_prev {
            unsafe {
                existing_prev.as_mut().next = Some(splice_start);
            }
        } else {
            self.head = Some(splice_start);
        }
        if let Some(mut existing_next) = existing_next {
            unsafe {
                existing_next.as_mut().prev = Some(splice_end);
            }
        } else {
            self.tail = Some(splice_end);
        }
        unsafe {
            splice_start.as_mut().prev = existing_prev;
            splice_end.as_mut().next = existing_next;
        }

        self.len += splice_length;
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            index: 0,
            current: self.head,
            list: self,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            index: self.len.checked_sub(1).unwrap_or(0),
            current: self.tail,
            list: self,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            marker: PhantomData,
        }
    }
}

#[allow(unused)]
pub struct Cursor<'a, T: 'a> {
    index: usize,
    current: Option<NonNull<Node<T>>>,
    list: &'a mut LinkedList<T>,
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<'a, T> Cursor<'a, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&'a mut T> {
        unsafe { self.current.map(|curr: NonNull<Node<T>>| &mut (*curr.as_ptr()).data) }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            let next: Option<NonNull<Node<T>>> = match self.current {
                None => self.list.head,
                Some(curr) => curr.as_ref().next,
            };

            next.map(|next: NonNull<Node<T>>| &mut (*next.as_ptr()).data)
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe {
            let prev: Option<NonNull<Node<T>>> = match self.current {
                None => self.list.tail,
                Some(curr) => curr.as_ref().prev,
            };

            prev.map(|prev: NonNull<Node<T>>| &mut (*prev.as_ptr()).data)
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        let unlinked_node: NonNull<Node<T>> = self.current?;
        unsafe {
            self.current = unlinked_node.as_ref().next;
            self.list.unlink_node(unlinked_node);
            let unlinked_node: Box<Node<T>> = Box::from_raw(unlinked_node.as_ptr());
            Some(unlinked_node.data)
        }
    }

    pub fn insert_after(&mut self, data: T) {
        unsafe {
            let spliced_node: NonNull<Node<T>> = Box::leak(Box::new(Node::new(data))).into();
            let node_next: Option<NonNull<Node<T>>> = match self.current {
                None => self.list.head,
                Some(node) => node.as_ref().next,
            };
            self.list
                .splice_nodes(self.current, node_next, spliced_node, spliced_node, 1);
            if self.current.is_none() {
                // The "ghost" non-element's index has changed.
                self.index = self.list.len;
            }
        }
    }

    pub fn insert_before(&mut self, data: T) {
        unsafe {
            let spliced_node: NonNull<Node<T>> = Box::leak(Box::new(Node::new(data))).into();
            let node_prev: Option<NonNull<Node<T>>> = match self.current {
                None => self.list.tail,
                Some(node) => node.as_ref().prev,
            };
            self.list
                .splice_nodes(node_prev, self.current, spliced_node, spliced_node, 1);
            self.index += 1;
        }
    }
}

#[allow(unused)]
pub struct Iter<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node: NonNull<Node<T>>| unsafe {
                // Need an unbound lifetime to get 'a
                let node: &Node<T> = &*node.as_ptr();
                self.len -= 1;
                self.head = node.next;
                &node.data
            })
        }
    }
}
