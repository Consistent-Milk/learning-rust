#![feature(prelude_import)]
#![allow(unused_variables)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use _002_add_two_numbers::*;
fn main() {
    let l: Option<Box<ListNode>> = ListLink::link(
        1,
        ListLink::link(2, ListLink::link(3, ListLink::link(4, None))),
    );
}
