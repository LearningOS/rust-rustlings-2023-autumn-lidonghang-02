// iterators3.rs
//
// This is a bigger exercise than most of the others! You can do it! Here is
// your mission, should you choose to accept it:
// 1. Complete the divide function to get the first four tests to pass.
// 2. Get the remaining tests to pass by completing the result_with_list and
//    list_of_results functions.
//
// Execute `rustlings hint iterators3` or use the `hint` watch subcommand for a
// hint.
// if a%b!=0, return a/b + 1
// else return a/b
// if you can`t divide a by b, return a/b

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// 当除法不是偶数时，除法函数需要返回正确的错误
// 可能的。

// Division_results 变量需要收集到集合类型中。

// result_with_list 函数需要返回单个 Result，其中成功
// case 是一个整数向量，失败的情况是一个 DivisionError。

// list_of_results 函数需要返回结果向量。
// See https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect for how
// the `FromIterator` trait is used in `collect()`. This trait is REALLY powerful! It
// can make the solution to this exercise infinitely easier.

// Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    // todo!();
    if b == 0 {
        Err(DivisionError::DivideByZero)
    } else if a % b != 0 {
        // Ok(a / b + 1)
        Err(DivisionError::NotDivisible(NotDivisibleError {
            dividend: a,
            divisor: b,
        }))
    } else {
        Ok(a / b)
    }
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: Ok([1, 11, 1426, 3])
fn result_with_list() -> Result<Vec<i32>, DivisionError> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    let res = division_results.collect::<Result<Vec<i32>, DivisionError>>();
    res
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    let res = division_results.collect();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
