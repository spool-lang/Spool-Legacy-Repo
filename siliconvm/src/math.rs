// Mathematical operations for Silicon VM

use std::ops::{Add, Sub, Mul, Div};
use std::i8::MAX;

fn add<T>(left: T, right: T) -> MathResult<T> where T: Add<Output = T> + Ord + MathExtensions
{
    let min = T::get_min();
    let max = T::get_max();
    return MathResult::Ok(left + right)
}

fn subtract<T>(left: T, right: T) -> MathResult<T> where T: Sub<Output = T> + Ord + MathExtensions
{
    return MathResult::Ok(left - right)
}

fn multiply<T>(left: T, right: T) -> MathResult<T> where T: Mul<Output = T> + Ord + MathExtensions
{
    return MathResult::Ok(left * right)
}

fn divide<T>(left: T, right: T, zero: T) -> MathResult<T> where T: Div<Output = T> + Ord + MathExtensions
{
    return MathResult::Ok(left / right)
}

pub enum MathResult<T> {
    Ok(T),
    Overflow,
    Underflow
}

impl <T> MathResult<T> {
    fn unwrap<F>(self, over: F, under: F) -> T
        where F: Fn() -> T
    {
        match self {
            MathResult::Overflow => over(),
            MathResult::Underflow => under(),
            MathResult::Ok(t) => t
        }
    }
}

trait MathExtensions {
    fn get_min() -> Self;

    fn get_max() -> Self;
}

impl MathExtensions for i8 {
    fn get_min() -> Self {
        i8::min_value()
    }

    fn get_max() -> Self {
        i8::max_value()
    }
}

impl MathExtensions for u8 {
    fn get_min() -> Self {
        u8::min_value()
    }

    fn get_max() -> Self {
        u8::max_value()
    }
}

impl MathExtensions for i16 {
    fn get_min() -> Self {
        i16::min_value()
    }

    fn get_max() -> Self {
        i16::max_value()
    }
}

impl MathExtensions for u16 {
    fn get_min() -> Self {
        u16::min_value()
    }

    fn get_max() -> Self {
        u16::max_value()
    }
}

impl MathExtensions for i32 {
    fn get_min() -> Self {
        i32::min_value()
    }

    fn get_max() -> Self {
        i32::max_value()
    }
}

impl MathExtensions for u32 {
    fn get_min() -> Self {
        u32::min_value()
    }

    fn get_max() -> Self {
        u32::max_value()
    }
}

impl MathExtensions for i64 {
    fn get_min() -> Self {
        i64::min_value()
    }

    fn get_max() -> Self {
        i64::max_value()
    }
}

impl MathExtensions for u64 {
    fn get_min() -> Self {
        u64::min_value()
    }

    fn get_max() -> Self {
        u64::max_value()
    }
}

impl MathExtensions for i128 {
    fn get_min() -> Self {
        i128::min_value()
    }

    fn get_max() -> Self {
        i128::max_value()
    }
}

impl MathExtensions for u128 {
    fn get_min() -> Self {
        u128::min_value()
    }

    fn get_max() -> Self {
        u128::max_value()
    }
}

impl MathExtensions for f32 {
    fn get_min() -> Self {
        std::f32::MIN
    }

    fn get_max() -> Self {
        std::f32::MAX
    }
}

impl MathExtensions for f64 {
    fn get_min() -> Self {
        std::f64::MIN
    }

    fn get_max() -> Self {
        std::f64::MAX
    }
}