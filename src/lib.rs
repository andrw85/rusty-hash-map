pub mod hashing;

#[cfg(test)]
mod tests {
    use super::hashing::*;
    #[test]
    fn test_insert() {
        let mut h: RustyHashMap<u32, String> = RustyHashMap::new();
        let result1 = h.insert(1234, String::from("hello"));
        assert!(result1.is_ok());
        let result2 = h.insert(1235, String::from("hello2"));
        assert!(result2.is_ok());
        let result3 = h.insert(1236, String::from("hello3"));
        assert!(result3.is_ok());
    }
    #[test]
    fn test_contains() {
        let mut h: RustyHashMap<u32, String> = RustyHashMap::new();

        let result1 = h.insert(1234, String::from("hello"));
        assert!(result1.is_ok());
        let result2 = h.insert(1235, String::from("hello2"));
        assert!(result2.is_ok());
        let result3 = h.insert(1236, String::from("hello3"));
        assert!(result3.is_ok());

        assert!(h.contains(1234).is_some());
        assert!(h.contains(1235).is_some());
        assert!(h.contains(1236).is_some());
    }
}
