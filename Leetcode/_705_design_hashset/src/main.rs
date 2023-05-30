use std::collections::HashMap;
fn main() {
    let n: usize = 5;

    println!("{}", fib_iter_nth(n));
}

/// Returns nth Fibonacci number
/// Indexing starts from 0, so n=0 gives the 0th Fibonacci number
/// It looks like this:
///
/// Index: 0 1 2 3 4 5 6
///
/// Value: 0 1 1 2 3 5 9
fn fib_iter_nth(n: usize) -> u64 {
    if (n == 0) | (n == 1) {
        return n as u64;
    }

    let mut table: Vec<u64> = vec![0; n];
    table[0] = 0;
    table[1] = 1;

    for i in 2..n {
        table[i] = table[i - 1] + table[i - 2];
    }

    table[n - 1]
}

/// Memoized fibonacci.
pub fn memoized_fibonacci(n: u32) -> u128 {
    let mut cache: HashMap<u32, u128> = HashMap::new();

    _memoized_fibonacci(n, &mut cache)
}

fn _memoized_fibonacci(n: u32, cache: &mut HashMap<u32, u128>) -> u128 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let f = match cache.get(&n) {
        Some(f) => f,
        None => {
            let f1 = _memoized_fibonacci(n - 1, cache);
            let f2 = _memoized_fibonacci(n - 2, cache);
            cache.insert(n, f1 + f2);
            cache.get(&n).unwrap()
        }
    };

    *f
}
