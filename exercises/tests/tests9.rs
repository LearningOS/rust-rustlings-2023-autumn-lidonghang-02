// tests9.rs
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// languages, and it can even link within the code itself! It makes it through the extern
// block, just like the code below.
//
// The short string after the `extern` keyword indicates which ABI the externally imported
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// "C" for standard C ABI, "stdcall" for the Windows ABI.
//
// The externally imported functions are declared in the extern blocks, with a semicolon to
// mark the end of signature instead of curly braces. Some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
//
// If you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same ABI string note. The default ABI
// for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// the whole extern term can be omitted.
//
// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
//
// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module Foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//
// You should NOT modify any existing code except for adding two lines of attributes.

//外部导入的函数在extern块中声明，并用分号表示
//标记签名的结尾而不是大括号。一些属性可以应用于那些
//修改链接行为的函数声明，例如#[link_name = ".."] 到
//修改实际的符号名称。
//
//如果要将符号导出到链接环境，可以使用 `extern` 关键字
//也可以在函数定义之前使用相同的 ABI 字符串注释进行标记。默认 ABI
//对于 Rust 函数来说，字面意思是“Rust”，所以如果你想链接到纯 Rust 函数，
//整个外部术语可以省略。
//
//Rust 默认情况下会损坏符号，就像 C++ 一样。为了抑制这种行为并使
//那些可通过名称寻址的函数，可以应用属性#[no_mangle]。

// You should NOT modify any existing code except for adding two lines of attributes.

extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}
mod Foo {
    // No `extern` equals `extern "Rust"`.
    #[no_mangle]
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.

        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
