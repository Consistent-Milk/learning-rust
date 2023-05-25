#[cfg(test)]
mod tests {
    use _4_control_flow::fib;

    #[test]
    fn test_fib_0() {
        assert_eq!(fib(0), 0);
    }

    #[test]
    fn test_fib_1() {
        assert_eq!(fib(1), 1);
    }

    #[test]
    fn test_fib_2() {
        assert_eq!(fib(2), 1);
    }

    
    #[test]
    fn test_fib_4() {
        assert_eq!(fib(4), 3);
    }
}