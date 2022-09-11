pub(crate) fn plus_one(a: i32) -> i32 {
    a + 1
}

#[cfg(test)]
mod tests {
    use super::plus_one;

    #[test]
    fn it_works() {
        assert_eq!(5, plus_one(4))
    }

    #[test]
    fn it_fails() {
        assert_ne!(5, plus_one(5))
    }
}
