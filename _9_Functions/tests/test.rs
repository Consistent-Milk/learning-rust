#[cfg(test)]
mod test {

    use _9_Functions::*;

    #[test]
    fn test_1() {
        assert_eq!(closure(), 45);
    }

    #[test]
    fn test_2() {
        capturing();
    }

    #[test]
    fn test_3() {
        assert_eq!(incrementing(), 2);
    }

    #[test]
    fn test_4() {
        non_copy();
    }
}
