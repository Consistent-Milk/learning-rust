#![allow(clippy::missing_safety_doc)]
#![allow(clippy::should_implement_trait)]

mod pre_implemented;
use std::alloc::{self, Layout};
use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
use std::mem;
use std::ptr::{self, NonNull};

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    _marker: PhantomData<T>,
}

pub struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    data: T,
}

impl<T> Node<T> {
    /// Allocate a new `Node`
    pub unsafe fn alloc(data: T) -> NonNull<Node<T>> {
        let layout: Layout = Layout::new::<Node<T>>();
        let ptr: *mut Node<T> = alloc::alloc(layout) as *mut Node<T>;

        if ptr.is_null() {
            alloc::handle_alloc_error(layout);
        }

        ptr::write(
            ptr,
            Node {
                next: None,
                prev: None,
                data,
            },
        );

        NonNull::new_unchecked(ptr)
    }

    // Deallocate a node without dropping the contents
    #[inline]
    pub unsafe fn dealloc(node: NonNull<Node<T>>) {
        alloc::dealloc(node.as_ptr() as *mut u8, Layout::new::<Node<T>>());
    }

    // Deallocate a node and drop its contents, rendering the passed pointer invalid.
    #[inline]
    pub unsafe fn dealloc_drop(mut node: NonNull<Node<T>>) {
        ptr::drop_in_place(&mut node.as_mut().data as *mut T);
        Self::dealloc(node);
    }

    #[inline]
    pub fn next_node(&self) -> Option<&Node<T>> {
        unsafe { Self::from_ptr(self.next).map(|v: &Node<T>| v as &Node<T>) }
    }

    #[inline]
    pub fn prev_node_mut(&mut self) -> Option<&mut Node<T>> {
        unsafe { Self::from_ptr_mut(self.prev) }
    }

    /// Dereference a Node pointer into a Node reference.
    /// `ptr` must be either None or a valid pointer to a Node.
    #[inline]
    pub unsafe fn from_ptr<'a>(ptr: Option<NonNull<Node<T>>>) -> Option<&'a Node<T>> {
        ptr.map(|node: NonNull<Node<T>>| {
            // transmute is 'safe' here because the ptr is valid the lifetime 'a.
            &*node.as_ptr()
        })
    }

    /// Dereference a Node pointer into a mutable Node reference.
    /// `ptr` must be either None or a valid pointer to a Node.
    #[inline]
    pub unsafe fn from_ptr_mut<'a>(ptr: Option<NonNull<Node<T>>>) -> Option<&'a mut Node<T>> {
        ptr.map(|mut node: NonNull<Node<T>>| {
            // transmute is 'safe' here because the ptr is valid the lifetime 'a.
            mem::transmute::<_, &'a mut Node<T>>(node.as_mut())
        })
    }
}

impl<T> LinkedList<T> {
    #[inline]
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        unsafe {
            // safe because self.head is either None or valid.
            Cursor::new(self, self.head)
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        unsafe {
            // safe because self.tail is either None or valid.
            Cursor::new(self, self.tail)
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter {
                current: Node::from_ptr(self.head),
            }
        }
    }
}

impl<T> Default for LinkedList<T> {
    #[inline]
    fn default() -> LinkedList<T> {
        LinkedList::new()
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_list().entries(self.iter()).finish()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        unsafe {
            let mut next: Option<NonNull<Node<T>>> = self.head;
            while let Some(ptr) = next {
                next = (*ptr.as_ptr()).next;
                Node::dealloc_drop(ptr);
            }
        }
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    current: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a T>,
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<'a, T> Cursor<'a, T> {
    /// Create a new Cursor.
    ///
    /// `node` must either `None` or a valid pointer.
    #[inline]
    unsafe fn new(list: &'a mut LinkedList<T>, node: Option<NonNull<Node<T>>>) -> Cursor<'a, T> {
        Cursor {
            list,
            current: node,
            _marker: PhantomData,
        }
    }

    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.current_data_mut()
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    pub fn next(&mut self) -> Option<&mut T> {
        self.current = self.current_mut().and_then(|node: &mut Node<T>| node.next);
        self.current_data_mut()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        self.current = self.current_mut().and_then(|node: &mut Node<T>| node.prev);
        self.current_data_mut()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        if let Some(mut current_ptr) = self.current {
            unsafe {
                let current: &mut Node<T> = current_ptr.as_mut();
                let next: Option<NonNull<Node<T>>> = match current.next {
                    Some(mut next_ptr) => {
                        let next: &mut Node<T> = next_ptr.as_mut();
                        next.prev = current.prev;
                        if let Some(prev) = current.prev_node_mut() {
                            prev.next = Some(next_ptr);
                        }
                        current.next
                    }
                    None => {
                        if let Some(prev) = current.prev_node_mut() {
                            prev.next = None;
                        }
                        current.prev
                    }
                };

                // If current node was head and/or tail of the list, replace it by `next`
                if self.list.head == self.current {
                    self.list.head = next;
                }

                if self.list.tail == self.current {
                    self.list.tail = next;
                }

                self.list.len -= 1;
                self.current = next;

                // Dealloc old current node and move its data out.
                let data: T = ptr::read(&mut current.data as *mut T);
                Node::dealloc(current_ptr);
                Some(data)
            }
        } else {
            None
        }
    }

    /// Insert an element at the back of the list.
    pub fn insert_after(&mut self, element: T) {
        unsafe {
            let mut node: NonNull<Node<T>> = Node::alloc(element);
            node.as_mut().prev = self.current;

            if let Some(mut current_ptr) = self.current {
                let current: &mut Node<T> = current_ptr.as_mut();
                node.as_mut().next = current.next;

                if let Some(mut next_ptr) = current.next {
                    next_ptr.as_mut().prev = Some(node);
                }

                current.next = Some(node)
            }

            if self.list.tail == self.current {
                self.list.tail = Some(node);
            }

            if self.list.head.is_none() {
                self.list.head = Some(node);
            }

            self.list.len += 1;
        }
    }

    /// Insert an element at the front of the list.
    pub fn insert_before(&mut self, element: T) {
        unsafe {
            let mut node: NonNull<Node<T>> = Node::alloc(element);
            node.as_mut().next = self.current;

            if let Some(mut current_ptr) = self.current {
                let current: &mut Node<T> = current_ptr.as_mut();
                node.as_mut().prev = current.prev;

                if let Some(mut prev_ptr) = current.prev {
                    prev_ptr.as_mut().next = Some(node);
                }

                current.prev = Some(node)
            }

            if self.list.head == self.current {
                self.list.head = Some(node);
            }

            if self.list.tail.is_none() {
                self.list.tail = Some(node);
            }

            self.list.len += 1;
        }
    }

    /// Return a mutable reference to the current node.
    #[inline]
    fn current_mut(&mut self) -> Option<&mut Node<T>> {
        unsafe {
            // safe because self.current is either None or a valid pointer to a Node.
            Node::from_ptr_mut(self.current)
        }
    }

    /// Return a mutable reference to the data in the current node.
    #[inline]
    fn current_data_mut(&mut self) -> Option<&mut T> {
        self.current_mut().map(|node: &mut Node<T>| &mut node.data)
    }
}

pub struct Iter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        self.current.map(|n: &Node<T>| {
            self.current = n.next_node();
            &n.data
        })
    }
}
