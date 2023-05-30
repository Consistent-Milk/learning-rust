pub fn find<T: Ord, V: AsRef<[T]>>(a: V, item: T) -> Option<usize> {
    let a: &[T] = a.as_ref();
    
    let mut low: usize = 0;
    let mut high: usize = a.len();

    while low < high {
        let mid: usize = (low + high) / 2;
        
        // Termination condition
        if a[mid] == item {
            return Some(mid);
        }

        if a[mid] > item {
            high = mid;
        } else if a[mid] < item {
            low = mid + 1;
        } 
    }

    return None;
}
