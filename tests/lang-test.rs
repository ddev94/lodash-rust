#[cfg(test)]

mod lang_tests {
    #[test]
    fn is_vector_test() {
        let array = vec![1, 2, 3];
        let not_array = "abc"; // This is a string
        let noop_input = || {}; // A closure
        assert_eq!(lodash_rust::is_array(&array), true);
        assert_eq!(lodash_rust::is_array(&not_array), false);
        assert_eq!(lodash_rust::is_array(&noop_input), false);
    }
}
