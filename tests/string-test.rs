#[cfg(test)]

mod string_test {
    #[test]
    fn camel_case_test() {
        assert_eq!(lodash_rust::camel_case("Geeks for Geeks"), "geeksForGeeks");
        assert_eq!(lodash_rust::camel_case("Geeks-for-Geeks"), "geeksForGeeks");
        assert_eq!(lodash_rust::camel_case("Geeks_for_Geeks"), "geeksForGeeks");
    }

    #[test]
    fn capitalize_case_test() {
        assert_eq!(lodash_rust::capitalize("Geeks for Geeks"), "Geeks for geeks");
        assert_eq!(lodash_rust::capitalize("GeeksforGeeks"), "Geeksforgeeks");
    }
}
