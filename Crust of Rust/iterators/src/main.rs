fn main() {
    let vs: Vec<i32> = vec![1, 2, 3];
    
    for v in vs.iter() {
        // borrows vs, gives ref to v (&v)
        println!("{}", v);
    }

    for v in &vs {
        // borrows vs, gives ref to v (&v)
        println!("{}", v);
    }

    for v in vs {
        // consumes vs, owned v, each v is dropped after
        // an iteration goes out of scope
        println!("{}", v);
    }
}