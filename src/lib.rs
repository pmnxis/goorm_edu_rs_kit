//! # Goorm Edu Rust Kit
//!
/// A code test library for easy variable of integer or vector of integer from standard input.
extern crate num;

pub mod groom_helper {
    use num::Integer;
    use num::NumCast;
    use num::Zero;
    use std::io;
    use std::io::prelude::*;
    use std::ops::{Add, Div, Mul, Sub};

    fn int_from_vec_u8<T>(vector: &Vec<u8>) -> T
    where
        T: Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Zero<Output = T>
            + num::ToPrimitive
            + NumCast
            + Integer
            + Copy,
    {
        let mut ret = T::zero();
        let flag = vector[0] != b'-';
        let start: usize = if flag { 0 } else { 1 };

        for i in start..vector.len() {
            if b'0' <= vector[i] && vector[i] <= b'9' {
                ret = ret * NumCast::from(10usize).unwrap()
                    + NumCast::from(vector[i] - b'0').unwrap();
            } else {
                break;
            }
        }
        if flag {
            ret
        } else {
            T::zero() - ret
        }
    }

    /// Get integer number from one line stdio.
    #[cfg(target_pointer_width = "64")]
    pub fn get_int<T>() -> T
    where
        T: Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Zero<Output = T>
            + num::ToPrimitive
            + NumCast
            + Integer
            + Copy,
    {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read stdin.");
        NumCast::from(buffer.trim().parse::<i64>().unwrap()).unwrap()
    }

    #[cfg(target_pointer_width = "32")]
    pub fn get_int<T>() -> T
    where
        T: Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Zero<Output = T>
            + num::ToPrimitive
            + NumCast
            + Integer
            + Copy,
    {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read stdin.");
        NumCast::from(buffer.trim().parse::<i32>().unwrap()).unwrap()
    }

    /// Get vectorized number with fixed length from one line stdio.
    /// ### Standard Input Example
    /// ```
    /// 1 2 3 4 5 6 7 8
    /// ```
    /// ### Output Example
    /// vec![1,2,3,4,5,6,7,8]
    pub fn get_vec_intt<T>(len: usize) -> Vec<T>
    where
        T: Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Zero<Output = T>
            + num::ToPrimitive
            + NumCast
            + Integer
            + Copy,
    {
        let mut __stdin = io::stdin();
        let mut __stdinlck = __stdin.lock();
        let mut ret: Vec<T> = Vec::with_capacity(len);
        let mut slice: Vec<u8> = Vec::new();
        for _i in 0..len - 1 {
            __stdinlck.read_until(b' ', &mut slice).expect("Error");
            ret.push(int_from_vec_u8(&slice));
            slice.clear();
        }
        __stdinlck.read_until(b'\n', &mut slice).expect("Error");
        ret.push(int_from_vec_u8(&slice));
        ret
    }
}
