use futures::executor::block_on;
use asynchronous_rust::async_main;

fn main() {
    block_on(async_main());
    println!("Now I am tired!");
}
