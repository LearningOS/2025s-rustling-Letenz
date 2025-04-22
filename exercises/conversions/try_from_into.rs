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

// Tuple implementation
// Tuple implementation
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        // 检查每个颜色值是否在有效范围内 (0..=255)
        let (red, green, blue) = tuple;
        
        if red < 0 || red > 255 || green < 0 || green > 255 || blue < 0 || blue > 255 {
            return Err(IntoColorError::IntConversion);
        }
        
        // 如果所有值都有效，创建并返回 Color 实例
        Ok(Color {
            red: red as u8,
            green: green as u8,
            blue: blue as u8,
        })
    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        // 检查每个颜色值是否在有效范围内
        if arr.iter().any(|&x| x < 0 || x > 255) {
            return Err(IntoColorError::IntConversion);
        }
        
        // 如果所有值都有效，创建并返回 Color 实例
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
        // 首先检查切片长度是否为3
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }
        
        // 然后检查每个颜色值是否在有效范围内
        if slice.iter().any(|&x| x < 0 || x > 255) {
            return Err(IntoColorError::IntConversion);
        }
        
        // 如果所有检查都通过，创建并返回 Color 实例
        Ok(Color {
            red: slice[0] as u8,
            green: slice[1] as u8,
            blue: slice[2] as u8,
        })
    }
}