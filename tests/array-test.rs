#[cfg(test)]

mod array_tests {
    #[test]
    fn find_integer_index_test() {
        let arr = vec![1, 2, 3, 4];
        assert_eq!(lodash_rust::find_index(&arr, |&x| x == 2), 1);
        assert_eq!(lodash_rust::find_index(&arr, |&x| x == 5), -1);
    }

    #[test]
    fn find_string_index_test() {
        let arr = vec!["Hello", "Lodash", "Rust"];
        assert_eq!(lodash_rust::find_index(&arr, |&x| x == "Rust"), 2);
        assert_eq!(lodash_rust::find_index(&arr, |&x| x == "Out"), -1);
    }

    #[test]
    fn find_char_index_test() {
        let arr = vec!['r', 'u', 's', 't'];
        assert_eq!(lodash_rust::find_index(&arr, |&x| x == 't'), 3);
        assert_eq!(lodash_rust::find_index(&arr, |&x| x == 'z'), -1);
    }

    #[test]
    fn find_test() {
        #[derive(Debug, Clone, PartialEq, Default)]
        struct User {
            user: String,
            age: u32,
            active: bool,
        }

        let default_user = User {
            user: "".to_string(),
            age: 0,
            active: false,
        };

        let users = vec![
            User {
                user: "barney".to_string(),
                age: 36,
                active: true,
            },
            User {
                user: "fred".to_string(),
                age: 40,
                active: false,
            },
            User {
                user: "pebbles".to_string(),
                age: 1,
                active: true,
            },
        ];

        assert_eq!(
            lodash_rust::find(&users, |user| user.age == 40),
            User {
                user: "fred".to_string(),
                age: 40,
                active: false,
            },
        );

        assert_eq!(
            lodash_rust::find(&users, |user| user.age > 140),
            default_user
        )
    }

    #[test]
    fn chunk_array_test() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(
            lodash_rust::chunk(&arr, 2),
            vec![vec![1, 2], vec![3, 4], vec![5]]
        );
        assert_eq!(
            lodash_rust::chunk(&arr, 0),
            vec![vec![1], vec![2], vec![3], vec![4], vec![5]]
        );

        assert_eq!(
            lodash_rust::chunk(&arr, 1),
            vec![vec![1], vec![2], vec![3], vec![4], vec![5]]
        );

        assert_eq!(lodash_rust::chunk(&arr, 6), vec![[1, 2, 3, 4, 5]]);
    }

    #[test]
    fn compact_array_test() {
        let arr = vec![0, 2, 3, 4, 5];
        assert_eq!(lodash_rust::compact(&arr), [2, 3, 4, 5]);

        let str_arr = vec!["", "Hello", "Lodash", "Rust"];
        assert_eq!(lodash_rust::compact(&str_arr), ["Hello", "Lodash", "Rust"]);

        let bool_arr = vec![true, false, false, true];
        assert_eq!(lodash_rust::compact(&bool_arr), [true, true]);
    }

    #[test]
    fn concat_array_test() {
        let arr1 = vec![1, 2, 3];
        let arr2 = vec![4, 5];
        assert_eq!(lodash_rust::concat(&arr1, &arr2), [1, 2, 3, 4, 5]);

        let str_arr1 = vec!["a", "b", "c"];
        let str_arr2 = vec!["d", "e", "f"];
        assert_eq!(
            lodash_rust::concat(&str_arr1, &str_arr2),
            ["a", "b", "c", "d", "e", "f"]
        );

        let char_arr1 = vec!['a', 'b', 'c'];
        let char_arr2 = vec!['a', 'b', 'c'];
        assert_eq!(
            lodash_rust::concat(&char_arr1, &char_arr2),
            ['a', 'b', 'c', 'a', 'b', 'c']
        );

        let d = lodash_rust::concat(&char_arr1, &char_arr2);

        assert_eq!(
            lodash_rust::concat(&d, &vec!['a']),
            ['a', 'b', 'c', 'a', 'b', 'c', 'a']
        );
    }

    #[test]
    fn difference_array_test() {
        let arr1 = vec![1, 2];
        let arr2 = vec![1, 2];
        assert_eq!(lodash_rust::difference(&arr1, &arr2), []);

        let arr3 = vec![1, 2, 3];
        let arr4 = vec![2, 3];
        assert_eq!(lodash_rust::difference(&arr3, &arr4), [1])
    }

    #[test]
    fn drop_array_test() {
        assert_eq!(lodash_rust::drop(&vec![1, 2, 3], 1), [2, 3]);
        assert_eq!(lodash_rust::drop(&vec![1, 2, 3], 2), [3]);
        assert_eq!(lodash_rust::drop(&vec![1, 2, 3], 0), [1, 2, 3]);
    }

    #[test]
    fn drop_right_array_test() {
        assert_eq!(lodash_rust::drop_right(&vec![1, 2, 3], 1), [1, 2]);
        assert_eq!(lodash_rust::drop_right(&vec![1, 2, 3], 2), [1]);
        assert_eq!(lodash_rust::drop_right(&vec![1, 2, 3], 0), [1, 2, 3]);
    }

    #[test]
    fn fill_array_test() {
        assert_eq!(lodash_rust::fill(&vec![1, 2, 3], 2), vec![2, 2, 2]);
        assert_eq!(lodash_rust::fill(&vec![1, 2, 3], 0), vec![0, 0, 0]);
    }

    #[test]
    fn head_array_test() {
        assert_eq!(lodash_rust::head(&vec![1, 2, 3]), 1);
        assert_eq!(lodash_rust::head(&vec!["Hello", "Rust"]), "Hello");
        assert_eq!(lodash_rust::head(&vec![vec![1], vec![2]]), vec![1]);
    }

    #[test]
    fn join_array_test() {
        assert_eq!(lodash_rust::join(&vec![1, 2, 3], "~"), "1~2~3");
        assert_eq!(lodash_rust::join(&vec![1, 2, 3], "+"), "1+2+3");
        assert_eq!(lodash_rust::join(&vec![1, 2, 3], ""), "123");
    }

    #[test]
    fn last_array_test() {
        assert_eq!(lodash_rust::last(&vec![1, 2, 3]), 3);
    }

    #[test]
    fn pull_array_test() {
        assert_eq!(lodash_rust::pull(&vec![1, 2, 3], &vec![1, 4, 5]), [2, 3]);
        assert_eq!(lodash_rust::pull(&vec![1, 2, 3], &vec![2, 3]), [1]);
    }

    #[test]
    fn remove_if_array_test() {
        assert_eq!(
            lodash_rust::remove_if(&vec![1, 2, 3], |&n| n % 2 == 0),
            [1, 3]
        );
        assert_eq!(
            lodash_rust::remove_if(&vec![1, 2, 3, 4, 5, 6], |&n| n == 6),
            [1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn reverse_array_test() {
        assert_eq!(lodash_rust::reverse(&vec![1, 2, 3]), [3, 2, 1]);
    }

    #[test]
    fn slice_array_test() {
        let animals = vec!["ant", "bison", "camel", "duck", "elephant"];
        assert_eq!(
            lodash_rust::slice(&animals, Some(2), None),
            ["camel", "duck", "elephant"]
        );
        assert_eq!(
            lodash_rust::slice(&animals, Some(2), Some(4)),
            ["camel", "duck"]
        );
    }

    #[test]
    fn sort_array_test() {
        let numbers = vec![1, 3, 4, 2, 5];
        assert_eq!(lodash_rust::sort(&numbers), [1, 2, 3, 4, 5]);
    }

    #[test]
    fn uniq_array_test() {
        let numbers = vec![1, 3, 3, 4, 4, 2, 5];
        assert_eq!(lodash_rust::uniq(&numbers), [1, 3, 4, 2, 5]);
    }

    #[test]
    fn xor_array_test() {
        let arr1 = vec![3, 1];
        let arr2 = vec![1, 2];
        assert_eq!(lodash_rust::xor(&[&arr1, &arr2]), [3, 2]);
        assert_eq!(lodash_rust::xor(&[&arr2, &arr1]), [2, 3]);
    }
}
