#[cfg(test)]
mod tests {
    #[test]
    fn test_binary_digit() {
        assert_eq!(bid::binary_digit(0), "00000000");
        assert_eq!(bid::binary_digit(255), "11111111");
        assert_eq!(bid::binary_digit(170), "10101010");
        assert_eq!(bid::binary_digit(123), "01111011");
    }
}
