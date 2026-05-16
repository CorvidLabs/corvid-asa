pub fn hello() -> &'static str {
    "Hello, Corvid ASA!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, Corvid ASA!");
    }
}