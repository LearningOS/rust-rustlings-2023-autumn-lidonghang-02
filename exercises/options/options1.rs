// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
//这个函数返回冰箱里还剩下多少冰淇淋。
//如果是晚上 10 点之前，则还剩 5 件。晚上10点，有人吃掉它们
//全部，所以不会再剩下了:(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a
    // value of 0 The Option output should gracefully handle cases where
    // time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!
    //我们这里使用 24 小时制，因此 10PM 的值为 22，12AM 的值为
    //值为 0 选项输出应该妥善处理以下情况
    //当天时间 > 23。
    //TODO：完成函数体 -记住返回一个选项！

    if time_of_day > 24 || time_of_day < 0 {
        return None;
    } else if time_of_day < 22 {
        return Some(5);
    } else {
        return Some(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the
        // Option?

        let icecreams = maybe_icecream(12);
        match icecreams {
            Option::Some(n) => {
                assert_eq!(n, 5);
            }
            Option::None => {}
        }
    }
}
