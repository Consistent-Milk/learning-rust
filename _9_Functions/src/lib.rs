#![allow(non_snake_case)]
#![allow(unused)]

use std::mem;

pub fn closure() -> i32 {
    let outer_var: i32 = 42;

    // Avoid using inference when possible, but the below is also valid
    // and sometimes it is not possible to know the type of a complex data
    // when writing code and it is better to let the compiler infer the type
    // let closure_method = |i| {i + outer_var}; 
    let closure_method = |i: i32| -> i32 { i + outer_var};

    return closure_method(3);
}

pub fn capturing() {
    let color: String = String::from("Green");
    let print = || println!("Color: {}", color);

    print();


    let _reborrow: &String = &color; 
}

pub fn incrementing() -> i32 {
    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };

    // Call the closure using a mutable borrow.
    inc();

    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error, because it is also borrowed as
    // a mutable reference. 
    // let _reborrow = &count; 
    // ^ TODO: try uncommenting this line.
    inc(); 

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error.
    let _count_reborrowed: &mut i32 = &mut count;

    return *_count_reborrowed;
}

pub fn non_copy() {
    // This by default stores 3 as an i32, but we can specify different types
    // let movable = Box::new(3u64); 
    // let movable = Box::new(3 as u64);
    let movable: Box<i32> = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();

    // calling consume again will lead to an compilation error
    //
    // Uncomment next line
    // consume();
}

fn haystack() {
    // `Vec` has non-copy semantics.
    let haystack: Vec<i32> = vec![1, 2, 3];

    let contains = move |needle: &i32| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There're {} elements in vec", haystack.len());
    // ^ Uncommenting above line will result in compile-time error
    // because borrow checker doesn't allow re-using variable after it
    // has been moved.
    
    // Removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available and uncommenting above line will not cause an error.

}