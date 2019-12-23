// Mathematical operations for Silicon VM

use std::ops::{Add, Sub, Mul, Div};
use std::i8::MAX;

fn add<T>(left: T, right: T) -> T where T: Add<Output = T> + Ord
{
    return MathResult::Ok(left + right)
}

fn subtract<T>(left: T, right: T) -> T where T: Sub<Output = T> + Ord
{
    return MathResult::Ok(left - right)
}

fn multiply<T>(left: T, right: T) -> T where T: Mul<Output = T> + Ord
{
    return MathResult::Ok(left * right)
}

fn divide<T>(left: T, right: T, zero: T) -> MathResult<T> where T: Div<Output = T> + Ord
{
    return MathResult::Ok(left / right)
}

pub enum MathResult<T> {
    Ok(T),
    Overflow,
    Underflow
}

pub trait MaxMin {
    fn get_max() -> Self;

    fn get_min() -> Self;
}

impl MaxMin for i8 {
    fn get_max() -> Self {

    }

    fn get_min() -> Self {
        unimplemented!()
    }
}