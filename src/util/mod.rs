//! Various utilities

pub mod units;

pub trait FirstLast {
    type Ret;

    fn first(&mut self) -> &mut Self::Ret;
    fn last(&mut self) -> &mut Self::Ret;
}

impl<T> FirstLast for Vec<T> {
    type Ret = T;

    fn first(&mut self) -> &mut T {
        &mut self[0]
    }

    fn last(&mut self) -> &mut T {
        let len = self.len();
        &mut self[len - 1]
    }
}
