#[path = "../src/pytest.rs"]
mod pytest;

#[cfg(test)]
mod tests {
    use crate::pytest;

    #[test]
    fn it_is_error_handler_for_test() {
        assert_ne!(pytest::is_error_header_for_test("_______________________________ test_passed_test _______________________________"), None);
        assert_eq!(pytest::is_error_header_for_test("_______________________________ test_passed_test _______________________________").unwrap(), "test_passed_test");
    }
}
