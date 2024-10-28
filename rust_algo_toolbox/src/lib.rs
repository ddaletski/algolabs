pub mod counting_sort;
pub mod hashtable;
pub mod kdtree;
pub mod ordered_counter;
pub mod pqueue;
pub mod radix_sort;
pub mod stateful_iterator;
pub mod string_search;
pub mod trie;
pub mod union_find;
pub mod weighted_graph;

#[macro_export]
macro_rules! format_expr_kv {
    ($key:literal) => {
        ""
    };
    ($key:expr) => {
        format!("\n  {}: `{}`", stringify!($key), format!("{:?}", $key))
    };
}

#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr, $fmt_str:literal, $($fmt_args:expr),*) => {
        let left_lit = stringify!($left);
        let right_lit = stringify!($right);

        if ($left != $right) {
            let custom_message = format!($fmt_str, $($fmt_args),*);
            let appendix = if custom_message.is_empty() {
                "".into()
            } else {
                format!("\nmessage: {}", custom_message)
            };

            let left_kv = $crate::format_expr_kv!($left);
            let right_kv = $crate::format_expr_kv!($right);

            let where_msg = if left_kv.is_empty() && right_kv.is_empty() {
                ""
            } else {
                "\nwhere"
            };

            panic!(
                "assertion failed:\n`{left_lit} == {right_lit}`{where_msg}{left_kv}{right_kv}{appendix}"
            );
        }
    };

    ($left:expr, $right:expr) => {
        assert_eq!($left, $right, "{}", "");
    }
}
#[macro_export]
macro_rules! assert_returns {
    ($ret_value:expr, $func:expr, $($args:expr),*) => {
        let mut args_str: String = "".into();
        $(
            args_str += format!("{:?}, ", $args).as_str();
        )*
        args_str.pop();
        args_str.pop();

        let result = $func($($args),*);

        // `stringify!` will convert the expression *as it is* into a string.
        let error_msg = format!(
            "{}({}) returned {:?}\nexpected result: {:?}\n",
            stringify!($func),
            args_str,
            result,
            $ret_value,
            );

        assert!(result == $ret_value, "\n{:}", error_msg);
    };
}

/// 2d vector literal macro
#[macro_export]
macro_rules! vec2d {
    [] => { Vec::new() };

    [$([$($value:expr),*]),*] => {
        vec![$(vec![$($value),*]),*]
    };
}

pub fn parse_2d_array(input_str: &str) -> Vec<Vec<i32>> {
    let mut result = vec![];

    let mut current_vec: Vec<i32> = vec![];

    let mut current_num = 0;
    let mut is_num = false;
    let mut is_arr = false;
    for ch in input_str.chars().skip(1) {
        match ch {
            '[' => {
                is_arr = true;
            }
            ']' => {
                if is_arr {
                    is_arr = false;
                    if is_num {
                        is_num = false;
                        current_vec.push(current_num);
                    }
                    current_num = 0;
                    result.push(std::mem::replace(&mut current_vec, vec![]));
                } else {
                    break;
                }
            }
            ',' => {
                if is_arr {
                    is_num = false;
                    current_vec.push(current_num);
                    current_num = 0;
                }
            }
            digit @ '0'..='9' => {
                if !is_num {
                    is_num = true;
                    current_num = digit.to_digit(10).unwrap() as i32;
                } else {
                    current_num = current_num * 10 + digit.to_digit(10).unwrap() as i32;
                }
            }
            ' ' => {}
            _ => {
                unreachable!()
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    mod vec2d_tests {
        #[test]
        fn empty_vec() {
            let actual: Vec<Vec<i32>> = vec2d![];
            let expected: Vec<Vec<i32>> = vec![];
            assert_eq!(actual, expected);
        }

        #[test]
        fn non_empty_vec() {
            let expected: Vec<Vec<i32>> = vec![vec![], vec![1], vec![1, 2]];
            assert_eq!(vec2d![[], [1], [1, 2]], expected);
        }
    }

    mod parse_2d_array_tests {
        use crate::parse_2d_array;

        #[test]
        fn case1() {
            let input_str = "[[1,2,3],[4,5,6],[7,8,9]]";
            let expected = vec2d![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
            let parsed = parse_2d_array(input_str);
            assert_eq!(expected, parsed);
        }

        #[test]
        fn case2() {
            let input_str = "[[112,72],[6],[],[70124234,8,9]]";
            let expected = vec2d![[112, 72], [6], [], [70124234, 8, 9]];
            let parsed = parse_2d_array(input_str);
            assert_eq!(parsed, expected);
        }

        #[test]
        fn parser_works_on_empty_array() {
            let input_str = "[]";
            let expected: Vec<Vec<i32>> = vec2d![];
            let parsed = parse_2d_array(input_str);
            assert_eq!(parsed, expected);
        }
    }
}
