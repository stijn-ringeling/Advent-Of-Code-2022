use std::fmt::Display;

#[allow(dead_code)]
pub enum Solution{
    I8(i8),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U32(u32),
    U64(u64),
    U128(u128),
    Str(String),
    USize(usize),
}

impl Display for Solution{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::I8(x) => x.fmt(f),
            Self::I32(x) => x.fmt(f),
            Self::I64(x) => x.fmt(f),
            Self::I128(x) => x.fmt(f),
            Self::U8(x) => x.fmt(f),
            Self::U32(x) => x.fmt(f),
            Self::U64(x) => x.fmt(f),
            Self::U128(x) => x.fmt(f),
            Self::Str(x) => x.fmt(f),
            Self::USize(x) => x.fmt(f),
        }
    }
}

impl From<i32> for Solution {
    fn from(i:i32) -> Solution{
        Solution::I32(i)
    }
}
impl From<i64> for Solution {
    fn from(i:i64) -> Solution{
        Solution::I64(i)
    }
}
impl From<i128> for Solution {
    fn from(i:i128) -> Solution{
        Solution::I128(i)
    }
}
impl From<u32> for Solution {
    fn from(i:u32) -> Solution{
        Solution::U32(i)
    }
}
impl From<u64> for Solution {
    fn from(i:u64) -> Solution{
        Solution::U64(i)
    }
}
impl From<u128> for Solution {
    fn from(i:u128) -> Solution{
        Solution::U128(i)
    }
}
impl From<String> for Solution {
    fn from(i:String) -> Solution{
        Solution::Str(i)
    }
}

impl From<usize> for Solution {
    fn from(i:usize) -> Solution{
        Solution::USize(i)
    }
}

pub type SolutionPair = (Solution, Solution);

pub trait Solver{
    fn solve(&self) -> SolutionPair{
        (0.into(), 0.into())
    }
}