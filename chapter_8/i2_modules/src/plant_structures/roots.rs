pub type Int8 = i8;

#[cfg(test)]
pub mod test {
    use super::*;

    pub fn addition(a: Int8, b: Int8) -> Int8 {
        a + b
    }

    #[test]
    fn run() {
        let result = addition(5, 5);
        assert_eq!(result, 20);
    }
}
