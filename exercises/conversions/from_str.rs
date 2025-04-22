// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}


// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an
//    error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age with something like `"4".parse::<usize>()`
// 6. If while extracting the name and the age something goes wrong, an error
//    should be returned
// If everything goes well, then return a Result of a Person object
//
// As an aside: `Box<dyn Error>` implements `From<&'_ str>`. This means that if
// you want to return a string error message, you can do so via just using
// return `Err("my error message".into())`.

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // 1. 如果输入字符串长度为0，返回Empty错误
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }
        
        // 2. 按逗号分割字符串
        let parts: Vec<&str> = s.split(',').collect();
        
        // 3. 检查分割后的元素数量是否为2，否则返回BadLen错误
        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }
        
        // 4. 提取名字
        let name = parts[0];
        
        // 检查名字是否为空
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }
        
        // 5. 提取并解析年龄
        let age_str = parts[1];
        
        // 尝试将年龄解析为usize，如果失败则包装错误
        let age = age_str.parse::<usize>().map_err(ParsePersonError::ParseInt)?;
        
        // 6. 如果一切正常，返回Person对象
        Ok(Person {
            name: String::from(name),
            age,
        })
    }
}