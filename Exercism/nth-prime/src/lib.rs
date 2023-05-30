pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::new();
    let mut start: u32 = 2;

    while primes.len() != (n + 1) as usize {
        if is_prime(start) {
            primes.push(start);
        }

        start += 1;
    }

    return *primes.last().unwrap();
}

fn is_prime(num: u32) -> bool {
    let sqrt = (num as f64).sqrt() as u32;

    for i in 2..=sqrt {
        if num % i == 0 {
            return false;
        }
    }

    return true;
}
