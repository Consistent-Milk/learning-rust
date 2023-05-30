fn main() {
    let n: u64 = 10;

    let steps: Option<u64> = collatz_conjecture::collatz(n);

    println!("Steps taken to reach 1 from {} is = {}", n, steps.unwrap());
}
