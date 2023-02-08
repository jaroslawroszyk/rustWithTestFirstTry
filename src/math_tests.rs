#[cfg(test)]
mod tests {
    use crate::math::sum;

    #[test]
    fn add_should_return4() {
        let result = sum(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_should_return23() {
        let result = sum(21, 2);
        assert_eq!(result, 23);
    }
    #[test]
    fn add_should_return0() {
        let result = sum(-1, 1);
        assert_eq!(result, 0);
    }
}
