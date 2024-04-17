// try_from_into.rs
//
// TryFrom is a simple and safe type conversion that may fail in a controlled
// way under some circumstances. Basically, this is the same as From. The main
// difference is that this should return a Result type instead of the target
// type itself. You can read more about it at
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html
//
// Execute `rustlings hint try_from_into` or use the `hint` watch subcommand for
// a hint.

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// We will use this error type for these `TryFrom` conversions.
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // Incorrect length of slice
    BadLen,
    // Integer conversion error
    IntConversion,
}

// Your task is to complete this implementation and return an Ok result of inner
// type Color. You need to create an implementation for a tuple of three
// integers, an array of three integers, and a slice of integers.
//
// Note that the implementation for tuple and array will be checked at compile
// time, but the slice implementation needs to check the slice length! Also note
// that correct RGB color values must be integers in the 0..=255 range.
//你的任务是完成这个实现并返回inner的Ok结果
//输入颜色。您需要为三元组创建一个实现
//整数、三个整数的数组和整数切片。
//
//请注意，元组和数组的实现将在编译时检查
//时间，但是切片实现需要检查切片长度！另请注意
//正确的 RGB 颜色值必须是 0..=255 范围内的整数

// Follow the steps provided right before the `TryFrom` implementation.
// You can also use the example at https://doc.rust-lang.org/std/convert/trait.TryFrom.html

// Is there an implementation of `TryFrom` in the standard library that
// can both do the required integer conversion and check the range of the input?

// Another hint: Look at the test cases to see which error variants to return.

// Yet another hint: You can use the `map_err` or `or` methods of `Result` to
// convert errors.

// Yet another hint: If you would like to propagate errors by using the `?`
// operator in your solution, you might want to look at
// https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html

// Challenge: Can you make the `TryFrom` implementations generic over many integer types?

fn check(num: &i16) -> bool {
    if *num > 255 || *num < 0 {
        return false;
    }
    true
}

// Tuple implementation
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        match tuple {
            (i, j, k) => {
                if check(&i) && check(&j) && check(&k) {
                    Ok(Color {
                        red: i as u8,
                        green: j as u8,
                        blue: k as u8,
                    })
                } else {
                    Err(IntoColorError::IntConversion)
                }
            }
        }
    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        for val in arr.iter() {
            if !check(val) {
                return Err(IntoColorError::IntConversion);
            }
        }

        Ok(Color {
            red: arr[0] as u8,
            green: arr[1] as u8,
            blue: arr[2] as u8,
        })
    }
}

// Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }
        for val in slice.iter() {
            if !check(val) {
                return Err(IntoColorError::IntConversion);
            }
        }
        Ok(Color {
            red: slice[0] as u8,
            green: slice[1] as u8,
            blue: slice[2] as u8,
        })
    }
}

fn main() {
    // Use the `try_from` function
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    // Since TryFrom is implemented for Color, we should be able to use TryInto
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    // With slice we should use `try_from` function
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    // or take slice within round brackets and use TryInto
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(
            Color::try_from((256, 1000, 10000)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(
            Color::try_from((-1, -10, -256)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_sum() {
        assert_eq!(
            Color::try_from((-1, 255, 255)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
}
