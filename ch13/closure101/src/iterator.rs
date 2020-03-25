fn try_iterator() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter(); // make iterator for v1. now we can use .next() method.
    for val in v1_iter {
        println!("value: {}", val);
    }
}


pub fn run() {
    try_iterator();
}



#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1,2,3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
    #[test]
    fn iterator_sum() {
        let v1 = vec![1,2,3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
    #[test]
    fn iterator_map() {
        let v1: Vec<i32> = vec![1,2,3];
        let v2: Vec<_> = v1.iter().map(|x| x+1).collect();

        assert_eq!(v2, [2,3,4]);
    }
}
