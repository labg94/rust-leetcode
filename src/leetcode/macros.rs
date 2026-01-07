#[macro_export]
macro_rules! leetcode_solution {
    (
        fn $func_name:ident($($param:ident: $param_type:ty),*) -> $return_type:ty {
            $($body:tt)*
        }
        
        tests {
            $(
                $test_name:ident: ($($arg:expr),*) => $expected:expr
            ),* $(,)?
        }
    ) => {
        struct Solution;

        impl Solution {
            pub fn $func_name($($param: $param_type),*) -> $return_type {
                $($body)*
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            $(
                #[test]
                fn $test_name() {
                    assert_eq!(Solution::$func_name($($arg),*), $expected);
                }
            )*
        }
    };
}