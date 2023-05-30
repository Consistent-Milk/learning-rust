pub fn collatz(mut n: u64) -> Option<u64> {
    // To do:
    // Infinite loop
    for i in 0.. {
        if n == 0 {
            break;
        }

        if n == 1 {
            return Some(i);
        }

        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n.checked_mul(3)?.checked_add(1)?;
        }
        // match n {
        //     0 => break,
        //     1 => return Some(i),
        //     num if num % 2 == 0 => n /= 2,
        //     _ => n = n.checked_mul(3)?.checked_add(1)?,
        // }
    }
    return None;
}