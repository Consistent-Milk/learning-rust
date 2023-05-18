//!
//! Source: https://github.com/TheAlgorithms/Rust
//! 
use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;
use std::ptr::NonNull;

struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}

pub struct LinkedList<T> {
    length: u32,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
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
    
    // TODO : Can we implement LinkedList without using unsafe blocks?
    // Rust forces the use of raw pointers to be put inside unsafe 
    // blocks, hence it probably won't be possible to implement a dynamic
    // data structure such as a LinkedList using Rust without the use
    // of unsafe blocks.

    // Algorithm:
    // 1 -> Create a new Node with the given data and box it (Allocate memory on the heap
    // and place the Node object in it)
    // 2 -> Set node.next to the current head
    // 3 -> Set node.prev as None
    // 4 -> Create a raw pointer to the created Node object called node_ptr
    // 5 -> Use pattern matching on self.head to check for None and Some
    // 6 -> If None then the LinkedList is empty and the current Node is the only element
    // in LinkedList thus set self.tail to node_ptr
    // 7 -> If self.head has Some value then the LinkedList is not empty
    // so we need to dereference head_ptr and set its .prev to node_ptr
    // 8 -> Set self.head to node_ptr
    // 9 -> Increase self.length by one
    pub fn insert_at_head(&mut self, obj: T) {
        let mut node: Box<Node<T>> = Box::new(Node::new(obj));
        node.next = self.head;
        node.prev = None;

        let node_ptr: Option<NonNull<Node<T>>> =
            Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });

        match self.head {
            None => self.tail = node_ptr,
            Some(head_ptr) => unsafe { (*head_ptr.as_ptr()).prev = node_ptr },
        }

        self.head = node_ptr;
        self.length += 1;
    }

    // Quite similar to the algorithm used for insert_at_head
    pub fn insert_at_tail(&mut self, obj: T) {
        let mut node: Box<Node<T>> = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.tail;

        let node_ptr: Option<NonNull<Node<T>>> =
            Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });

        match self.tail {
            None => self.head = node_ptr,
            Some(tail_ptr) => unsafe { (*tail_ptr.as_ptr()).next = node_ptr },
        }

        self.tail = node_ptr;
        self.length += 1;
    }

    // Algorithm:
    // 1 -> Check if self.length is less than index, if true then panic.
    // 2 -> If index is 0, use self.insert_at_head() and return
    // 3 -> If self.length is equal to index use self.insert_at_tail() and return
    // 4 -> If index is somewhere between head and tail go to inner algorithm
    pub fn insert_at_ith(&mut self, index: u32, obj: T) {
        if self.length < index {
            panic!("Index out of bounds");
        }

        if index == 0 || self.head.is_none() {
            self.insert_at_head(obj);
            return;
        }

        if self.length == index {
            self.insert_at_tail(obj);
            return;
        }

        // Inner Algorithm:
        // 1 -> If self.head has Some value, store it in mutable ith_node
        // 2 -> Loop from 0 to index 
        // 3 -> 
        if let Some(mut ith_node) = self.head {
            for _ in 0..index {
                unsafe {
                    match (*ith_node.as_ptr()).next {
                        None => panic!("Index out of bounds"),
                        Some(next_ptr) => ith_node = next_ptr,
                    }
                }
            }

            let mut node: Box<Node<T>> = Box::new(Node::new(obj));
            unsafe {
                node.prev = (*ith_node.as_ptr()).prev;
                node.next = Some(ith_node);
                if let Some(p) = (*ith_node.as_ptr()).prev {
                    let node_ptr: Option<NonNull<Node<T>>> =
                        Some(NonNull::new_unchecked(Box::into_raw(node)));
                    println!("{:?}", (*p.as_ptr()).next);
                    (*p.as_ptr()).next = node_ptr;
                    (*ith_node.as_ptr()).prev = node_ptr;
                    self.length += 1;
                }
            }
        }
    }

    pub fn delete_head(&mut self) -> Option<T> {
        self.head.map(|head_ptr: NonNull<Node<T>>| unsafe {
            let old_head: Box<Node<T>> = Box::from_raw(head_ptr.as_ptr());
            match old_head.next {
                Some(mut next_ptr) => next_ptr.as_mut().prev = None,
                None => self.tail = None,
            }
            self.head = old_head.next;
            self.length -= 1;
            old_head.val
        })
    }

    pub fn delete_tail(&mut self) -> Option<T> {
        self.tail.map(|tail_ptr: NonNull<Node<T>>| unsafe {
            let old_tail: Box<Node<T>> = Box::from_raw(tail_ptr.as_ptr());
            match old_tail.prev {
                Some(mut prev) => prev.as_mut().next = None,
                None => self.head = None,
            }
            self.tail = old_tail.prev;
            self.length -= 1;
            old_tail.val
        })
    }

    pub fn delete_ith(&mut self, index: u32) -> Option<T> {
        if self.length < index {
            panic!("Index out of bounds");
        }

        if index == 0 || self.head.is_none() {
            return self.delete_head();
        }

        if self.length == index {
            return self.delete_tail();
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

            unsafe {
                let old_ith: Box<Node<T>> = Box::from_raw(ith_node.as_ptr());
                if let Some(mut prev) = old_ith.prev {
                    prev.as_mut().next = old_ith.next;
                }
                if let Some(mut next) = old_ith.next {
                    next.as_mut().prev = old_ith.prev;
                }

                self.length -= 1;
                Some(old_ith.val)
            }
        } else {
            None
        }
    }

    pub fn get(&mut self, index: i32) -> Option<&'static T> {
        Self::get_ith_node(self.head, index)
    }

    fn get_ith_node(node: Option<NonNull<Node<T>>>, index: i32) -> Option<&'static T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => Self::get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.delete_head().is_some() {}
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.head {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::LinkedList;

    #[test]
    fn insert_at_tail_works() {
        let mut list = LinkedList::<i32>::new();
        let second_value = 2;
        list.insert_at_tail(1);
        list.insert_at_tail(second_value);
        println!("Linked List is {}", list);
        match list.get(1) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {} at index 1", second_value),
        }
    }
    #[test]
    fn insert_at_head_works() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        let second_value: i32 = 2;
        list.insert_at_head(1);
        list.insert_at_head(second_value);
        println!("Linked List is {}", list);
        match list.get(0) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {} at index 0", second_value),
        }
    }

    #[test]
    fn insert_at_ith_can_add_to_tail() {
        let mut list = LinkedList::<i32>::new();
        let second_value = 2;
        list.insert_at_ith(0, 0);
        list.insert_at_ith(1, second_value);
        println!("Linked List is {}", list);
        match list.get(1) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {} at index 1", second_value),
        }
    }

    #[test]
    fn insert_at_ith_can_add_to_head() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        let second_value: i32 = 2;
        list.insert_at_ith(0, 1);
        list.insert_at_ith(0, second_value);
        println!("Linked List is {}", list);
        match list.get(0) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {} at index 0", second_value),
        }
    }

    #[test]
    fn insert_at_ith_can_add_to_middle() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        let second_value: i32 = 2;
        let third_value: i32 = 3;
        list.insert_at_ith(0, 1);
        list.insert_at_ith(1, second_value);
        list.insert_at_ith(1, third_value);
        println!("Linked List is {}", list);
        match list.get(1) {
            Some(val) => assert_eq!(*val, third_value),
            None => panic!("Expected to find {} at index 1", third_value),
        }

        match list.get(2) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {} at index 1", second_value),
        }
    }

    #[test]
    fn insert_at_ith_and_delete_at_ith_in_the_middle() {
        // Insert and delete in the middle of the list to ensure pointers are updated correctly
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        let first_value: i32 = 0;
        let second_value: i32 = 1;
        let third_value: i32 = 2;
        let fourth_value: i32 = 3;

        list.insert_at_ith(0, first_value);
        list.insert_at_ith(1, fourth_value);
        list.insert_at_ith(1, third_value);
        list.insert_at_ith(1, second_value);

        list.delete_ith(2);
        list.insert_at_ith(2, third_value);

        for (i, expected) in [
            (0, first_value),
            (1, second_value),
            (2, third_value),
            (3, fourth_value),
        ] {
            match list.get(i) {
                Some(val) => assert_eq!(*val, expected),
                None => panic!("Expected to find {} at index {}", expected, i),
            }
        }
    }

    #[test]
    fn insert_at_ith_and_delete_ith_work_over_many_iterations() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        for i in 0..100 {
            list.insert_at_ith(i, i.try_into().unwrap());
        }

        // Pop even numbers to 50
        for i in 0..50 {
            println!("list.length {}", list.length);
            if i % 2 == 0 {
                list.delete_ith(i);
            }
        }

        assert_eq!(list.length, 75);

        // Insert even numbers back
        for i in 0..50 {
            if i % 2 == 0 {
                list.insert_at_ith(i, i.try_into().unwrap());
            }
        }

        assert_eq!(list.length, 100);

        // Ensure numbers were adderd back and we're able to traverse nodes
        if let Some(val) = list.get(78) {
            assert_eq!(*val, 78);
        } else {
            panic!("Expected to find 78 at index 78");
        }
    }

    #[test]
    fn delete_tail_works() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        let first_value: i32 = 1;
        let second_value: i32 = 2;
        list.insert_at_tail(first_value);
        list.insert_at_tail(second_value);
        match list.delete_tail() {
            Some(val) => assert_eq!(val, 2),
            None => panic!("Expected to remove {} at tail", second_value),
        }

        println!("Linked List is {}", list);
        match list.get(0) {
            Some(val) => assert_eq!(*val, first_value),
            None => panic!("Expected to find {} at index 0", first_value),
        }
    }

    #[test]
    fn delete_head_works() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        list.insert_at_tail(first_value);
        list.insert_at_tail(second_value);
        match list.delete_head() {
            Some(val) => assert_eq!(val, 1),
            None => panic!("Expected to remove {} at head", first_value),
        }

        println!("Linked List is {}", list);
        match list.get(0) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {} at index 0", second_value),
        }
    }

    #[test]
    fn delete_ith_can_delete_at_tail() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        list.insert_at_tail(first_value);
        list.insert_at_tail(second_value);
        match list.delete_ith(1) {
            Some(val) => assert_eq!(val, 2),
            None => panic!("Expected to remove {} at tail", second_value),
        }

        assert_eq!(list.length, 1);
    }

    #[test]
    fn delete_ith_can_delete_at_head() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        list.insert_at_tail(first_value);
        list.insert_at_tail(second_value);
        match list.delete_ith(0) {
            Some(val) => assert_eq!(val, 1),
            None => panic!("Expected to remove {} at tail", first_value),
        }

        assert_eq!(list.length, 1);
    }

    #[test]
    fn delete_ith_can_delete_in_middle() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        let third_value = 3;
        list.insert_at_tail(first_value);
        list.insert_at_tail(second_value);
        list.insert_at_tail(third_value);
        match list.delete_ith(1) {
            Some(val) => assert_eq!(val, 2),
            None => panic!("Expected to remove {} at tail", second_value),
        }

        match list.get(1) {
            Some(val) => assert_eq!(*val, third_value),
            None => panic!("Expected to find {} at index 1", third_value),
        }
    }

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.insert_at_tail(1);
        list.insert_at_tail(2);
        list.insert_at_tail(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.insert_at_tail("A".to_string());
        list_str.insert_at_tail("B".to_string());
        list_str.insert_at_tail("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn get_by_index_in_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.insert_at_tail(1);
        list.insert_at_tail(2);
        println!("Linked List is {}", list);
        let retrived_item = list.get(1);
        assert!(retrived_item.is_some());
        assert_eq!(2, *retrived_item.unwrap());
    }

    #[test]
    fn get_by_index_in_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.insert_at_tail("A".to_string());
        list_str.insert_at_tail("B".to_string());
        println!("Linked List is {}", list_str);
        let retrived_item = list_str.get(1);
        assert!(retrived_item.is_some());
        assert_eq!("B", *retrived_item.unwrap());
    }
}
