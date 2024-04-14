mod addition;
pub use addition::*;

mod multiplication;
pub use multiplication::*;

pub trait Add<Rhs = Self> {
    type Output;

    fn add(self, other: Rhs) -> Self::Output;
}