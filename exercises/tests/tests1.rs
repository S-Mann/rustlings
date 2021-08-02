#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        assert!(true);
    }

    #[test]
    fn you_can_compare() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    #[should_panic]
    fn you_can_panic() {
        panic!("Trouble!");
    }

    #[test]
    fn you_can_result() -> Result<(), String> {
        if 1 + 1 == 2 {
            Ok(())
        } else {
            Err(String::from("Equality broken."))
        }
    }
}
