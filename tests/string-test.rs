#[cfg(test)]

mod string_test {
    #[test]
    fn camel_case_test() {
        assert_eq!(lodashrust::camel_case("Geeks for Geeks"), "geeksForGeeks");
        assert_eq!(lodashrust::camel_case("Geeks-for-Geeks"), "geeksForGeeks");
        assert_eq!(lodashrust::camel_case("Geeks_for_Geeks"), "geeksForGeeks");
    }

    #[test]
    fn capitalize_test() {
        assert_eq!(
            lodashrust::capitalize("Geeks for Geeks"),
            "Geeks for geeks"
        );
        assert_eq!(lodashrust::capitalize("GeeksforGeeks"), "Geeksforgeeks");
    }

    #[test]
    fn ends_with_test() {
        assert_eq!(lodashrust::ends_with("abc", "c", None), true);
        assert_eq!(lodashrust::ends_with("abc", "b", Some(2)), true);
        assert_eq!(lodashrust::ends_with("abc", "a", None), false);
    }

    #[test]
    fn escape_with_test() {
        assert_eq!(
            lodashrust::escape("fred, barney, & pebbles"),
            "fred, barney, &amp; pebbles"
        );
        assert_eq!(
            lodashrust::escape("fred, barney, <script>aaa</script> pebbles"),
            "fred, barney, &lt;script&gt;aaa&lt;/script&gt; pebbles"
        );
    }

    #[test]
    fn lower_case_test() {
        assert_eq!(lodashrust::lower_case("--Foo-Bar--"), "foo bar");
        assert_eq!(lodashrust::lower_case("fooBar"), "foobar");
        assert_eq!(lodashrust::lower_case("__FOO_BAR__"), "foo bar");
    }

    #[test]
    fn repeat_test() {
        assert_eq!(lodashrust::repeat("*", 3), "***");
        assert_eq!(lodashrust::repeat("abc", 2), "abcabc");
        assert_eq!(lodashrust::repeat("abc", 0), "");
    }

    #[test]
    fn snake_case_test() {
        assert_eq!(lodashrust::snake_case("Foo Bar"), "foo_bar");
        assert_eq!(lodashrust::snake_case("fooBar"), "foobar");
        assert_eq!(lodashrust::snake_case("--FOO-BAR--"), "foo_bar");
    }

    #[test]
    fn split_test() {
        assert_eq!(lodashrust::split("Foo~Bar", "~", None), vec!["Foo", "Bar"]);
        assert_eq!(
            lodashrust::split("Foo~Bar~The", "~", Some(2)),
            vec!["Foo", "Bar"]
        );
    }

    #[test]
    fn starts_with_test() {
        assert_eq!(lodashrust::starts_with("abc", "a", None), true);
        assert_eq!(lodashrust::starts_with("abc", "b", None), false);
        assert_eq!(lodashrust::starts_with("abc", "b", Some(1)), true);
        assert_eq!(lodashrust::starts_with("abc", "ab", Some(0)), true);
        assert_eq!(lodashrust::starts_with("abc", "abc", Some(0)), true);
        assert_eq!(lodashrust::starts_with("abc", "c", Some(2)), true);
        assert_eq!(lodashrust::starts_with("abc", "a", Some(1)), false);
        assert_eq!(lodashrust::starts_with("abc", "", Some(1)), true); // An empty string is always a prefix
    }

    #[test]
    fn to_lower_test() {
        assert_eq!(lodashrust::to_lower("--Foo-Bar--"), "--foo-bar--");
        assert_eq!(lodashrust::to_lower("fooBar"), "foobar");
        assert_eq!(lodashrust::to_lower("__FOO_BAR__"), "__foo_bar__");
        assert_eq!(lodashrust::to_lower("HELLO WORLD!"), "hello world!");
        assert_eq!(
            lodashrust::to_lower("Rust Is Awesome!"),
            "rust is awesome!"
        );
    }

    #[test]
    fn to_upper_test() {
        assert_eq!(lodashrust::to_upper("--foo-bar--"), "--FOO-BAR--");
        assert_eq!(lodashrust::to_upper("fooBar"), "FOOBAR");
        assert_eq!(lodashrust::to_upper("__foo_bar__"), "__FOO_BAR__");
        assert_eq!(lodashrust::to_upper("hello world!"), "HELLO WORLD!");
        assert_eq!(
            lodashrust::to_upper("Rust is awesome!"),
            "RUST IS AWESOME!"
        );
    }
    #[test]
    fn test_trim() {
        assert_eq!(lodashrust::trim("  abc  "), "abc");
        assert_eq!(lodashrust::trim("\t\nabc\r\n"), "abc");
        assert_eq!(lodashrust::trim("abc"), "abc");
        assert_eq!(lodashrust::trim(""), "");
        assert_eq!(lodashrust::trim("   "), "");
    }

    #[test]
    fn test_trim_chars() {
        assert_eq!(lodashrust::trim_chars("-_-abc-_-", "_-"), "abc");
        assert_eq!(lodashrust::trim_chars("***abc***", "*"), "abc");
        assert_eq!(lodashrust::trim_chars("abc", "_-"), "abc");
        assert_eq!(lodashrust::trim_chars("", "_-"), "");
        assert_eq!(lodashrust::trim_chars("---", "-"), "");
        assert_eq!(
            lodashrust::trim_chars("-_-abc-_-def-_-", "_-"),
            "abc-_-def"
        );
        assert_eq!(lodashrust::trim_chars("__abc__def__", "_"), "abc__def");
        assert_eq!(lodashrust::trim_chars("-_-", "_-"), "");
        assert_eq!(lodashrust::trim_chars("abc-_-def", "_-"), "abc-_-def");
    }

    #[test]
    fn test_trim_end() {
        assert_eq!(lodashrust::trim_end("  abc  "), "  abc");
        assert_eq!(lodashrust::trim_end("\t\nabc\r\n"), "\t\nabc");
        assert_eq!(lodashrust::trim_end("abc"), "abc");
        assert_eq!(lodashrust::trim_end(""), "");
        assert_eq!(lodashrust::trim_end("   "), "");
        assert_eq!(lodashrust::trim_end("abc   \n\t"), "abc");
    }

    #[test]
    fn test_trim_end_chars() {
        assert_eq!(lodashrust::trim_end_chars("-_-abc-_-", "_-"), "-_-abc");
        assert_eq!(lodashrust::trim_end_chars("***abc***", "*"), "***abc");
        assert_eq!(lodashrust::trim_end_chars("abc", "_-"), "abc");
        assert_eq!(lodashrust::trim_end_chars("", "_-"), "");
        assert_eq!(lodashrust::trim_end_chars("---", "-"), "");
        assert_eq!(
            lodashrust::trim_end_chars("-_-abc-_-def-_-", "_-"),
            "-_-abc-_-def"
        );
        assert_eq!(
            lodashrust::trim_end_chars("__abc__def__", "_"),
            "__abc__def"
        );
        assert_eq!(lodashrust::trim_end_chars("-_-", "_-"), "");
        assert_eq!(lodashrust::trim_end_chars("abc-_-def", "_-"), "abc-_-def");
    }

    #[test]
    fn test_edge_cases() {
        // Mixed characters
        assert_eq!(
            lodashrust::trim_end_chars("---***abc***---", "-*"),
            "---***abc"
        );

        // Repeated characters
        assert_eq!(lodashrust::trim_end_chars("aaaaabcaaaa", "a"), "aaaaabc");

        // Single character
        assert_eq!(lodashrust::trim_end_chars("a", "a"), "");

        // All characters to be trimmed
        assert_eq!(lodashrust::trim_end_chars("abcdef", "abcdef"), "");

        // No characters to be trimmed
        assert_eq!(lodashrust::trim_end_chars("abc", "xyz"), "abc");

        // Mixed whitespace and custom characters
        assert_eq!(lodashrust::trim_end_chars("  abc  ***", "*"), "  abc  ");
    }

    #[test]
    fn test_trim_start() {
        assert_eq!(lodashrust::trim_start("  abc  "), "abc  ");
        assert_eq!(lodashrust::trim_start("\t\nabc\r\n"), "abc\r\n");
        assert_eq!(lodashrust::trim_start("abc"), "abc");
        assert_eq!(lodashrust::trim_start(""), "");
        assert_eq!(lodashrust::trim_start("   "), "");
        assert_eq!(lodashrust::trim_start("   \n\tabc"), "abc");
    }

    #[test]
    fn test_trim_start_chars() {
        assert_eq!(lodashrust::trim_start_chars("-_-abc-_-", "_-"), "abc-_-");
        assert_eq!(lodashrust::trim_start_chars("***abc***", "*"), "abc***");
        assert_eq!(lodashrust::trim_start_chars("abc", "_-"), "abc");
        assert_eq!(lodashrust::trim_start_chars("", "_-"), "");
        assert_eq!(lodashrust::trim_start_chars("---", "-"), "");
        assert_eq!(
            lodashrust::trim_start_chars("-_-abc-_-def-_-", "_-"),
            "abc-_-def-_-"
        );
        assert_eq!(
            lodashrust::trim_start_chars("__abc__def__", "_"),
            "abc__def__"
        );
        assert_eq!(lodashrust::trim_start_chars("-_-", "_-"), "");
        assert_eq!(
            lodashrust::trim_start_chars("abc-_-def", "_-"),
            "abc-_-def"
        );
    }

    #[test]
    fn test_trim_start_edge_cases() {
        // Mixed characters
        assert_eq!(
            lodashrust::trim_start_chars("---***abc***---", "-*"),
            "abc***---"
        );

        // Repeated characters
        assert_eq!(lodashrust::trim_start_chars("aaaaabcaaaa", "a"), "bcaaaa");

        // Single character
        assert_eq!(lodashrust::trim_start_chars("a", "a"), "");

        // All characters to be trimmed
        assert_eq!(lodashrust::trim_start_chars("abcdef", "abcdef"), "");

        // No characters to be trimmed
        assert_eq!(lodashrust::trim_start_chars("abc", "xyz"), "abc");

        // Mixed whitespace and custom characters
        assert_eq!(lodashrust::trim_start_chars("***  abc  ", "*"), "  abc  ");
    }

    #[test]
    fn test_trim_start_complex_cases() {
        // Multiple character types
        assert_eq!(lodashrust::trim_start_chars("-_*-abc-_*", "_-*"), "abc-_*");

        // Alternating characters
        assert_eq!(lodashrust::trim_start_chars("-_-_-_abc", "_-"), "abc");

        // Single character in trim set
        assert_eq!(lodashrust::trim_start_chars("---abc---", "-"), "abc---");

        // Empty trim set
        assert_eq!(lodashrust::trim_start_chars("---abc---", ""), "---abc---");

        // Trim set larger than string
        assert_eq!(lodashrust::trim_start_chars("abc", "abcdef"), "");
    }

    #[test]
    fn test_truncate() {
        // Test default behavior
        assert_eq!(
            lodashrust::truncate("hi-diddly-ho there, neighborino", None),
            "hi-diddly-ho there, neighbo..."
        );

        // Test with custom length and space separator
        assert_eq!(
            lodashrust::truncate(
                "hi-diddly-ho there, neighborino",
                Some(lodashrust::TruncateOptions {
                    length: 24,
                    separator: Some(" "),
                    omission: "..."
                })
            ),
            "hi-diddly-ho there,..."
        );

        // Test with comma-space separator
        assert_eq!(
            lodashrust::truncate(
                "hi-diddly-ho there, neighborino",
                Some(lodashrust::TruncateOptions {
                    length: 24,
                    separator: Some(", "),
                    omission: "..."
                })
            ),
            "hi-diddly-ho there..."
        );

        // Test with hyphen separator
        assert_eq!(
            lodashrust::truncate(
                "hi-diddly-ho-there-neighborino",
                Some(lodashrust::TruncateOptions {
                    length: 24,
                    separator: Some("-"),
                    omission: "..."
                })
            ),
            "hi-diddly-ho-there..."
        );

        // Test with custom omission
        assert_eq!(
            lodashrust::truncate(
                "hi-diddly-ho there, neighborino",
                Some(lodashrust::TruncateOptions {
                    length: 30,
                    separator: None,
                    omission: " [...]"
                })
            ),
            "hi-diddly-ho there, neig [...]"
        );

        // Test short string
        assert_eq!(lodashrust::truncate("short", None), "short");

        // Test empty string
        assert_eq!(lodashrust::truncate("", None), "");

        // Test Unicode support
        assert_eq!(
            lodashrust::truncate(
                "Hello 👋 World",
                Some(lodashrust::TruncateOptions {
                    length: 8,
                    separator: None,
                    omission: "..."
                })
            ),
            "Hello..."
        );

        // Test with separator at the end
        assert_eq!(
            lodashrust::truncate(
                "one, two, three, ",
                Some(lodashrust::TruncateOptions {
                    length: 15,
                    separator: Some(", "),
                    omission: "..."
                })
            ),
            "one, two..."
        );
    }

    #[test]
    fn test_unescape() {
        let test_cases = vec![
            // Basic case (original example)
            ("fred, barney, &amp; pebbles", "fred, barney, & pebbles"),
            // Multiple entities
            ("&lt;div&gt;Hello &amp; welcome&lt;/div&gt;", "<div>Hello & welcome</div>"),
            // No entities
            ("Hello World", "Hello World"),
            // Quotes and apostrophes
            ("&quot;Hello&quot; &apos;World&apos;", "\"Hello\" 'World'"),
            // Empty string
            ("", ""),
        ];

        for (input, expected) in test_cases {
            assert_eq!(lodashrust::unescape(input), expected, "Failed on input: {}", input);
        }
    }
}
