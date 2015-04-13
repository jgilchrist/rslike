//! Various utilities

pub mod units;

pub trait FromChar {
    type Ret;

    fn from_char(c: char) -> Self::Ret;
}
