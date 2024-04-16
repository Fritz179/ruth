use crate::{operations::{Add, Exp, Mul, Sub, TypeAdd, TypeExp, TypeMul, TypeSub}, MyInto};
use super::{natural::Natural, real::WrappedReal, MyFrom, Types, Wrapper};

#[derive(Debug, Clone)]
pub struct Zahl(i32);

impl Zahl {
    pub fn new(value: i32) -> Self {
        Self(value)
    }

    pub fn get(&self) -> i32 {
        self.0
    }
}

impl<From: MyInto<Natural>> MyFrom<From> for Zahl {
    fn my_from(from: From) -> Zahl {
        Self::new(from.my_into().get() as i32)
    }
}

impl MyFrom<Zahl> for Zahl {
    fn my_from(from: Zahl) -> Self {
        from
    }
}

pub type WrappedZahl = Wrapper<Zahl>;

impl WrappedZahl {
    pub fn new(value: i32) -> Self {
        Self::Constant(Zahl(value))
    }

    pub fn get_type(&self) -> &str {
        "Zahl"
    }
}

impl Add for Zahl {
    type Output = Zahl;

    fn add(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Zahl::new(self.get() + rhs.get()))
    }
}

impl TypeAdd for WrappedZahl {
    fn type_add(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.add(rhs.my_into()),
            Types::Zahl(rhs) => self.add(rhs),
            Types::Real(rhs) => MyInto::<WrappedReal>::my_into(self).add(rhs),
        }
    }
}

impl Sub for Zahl {
    type Output = Zahl;

    fn sub(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Zahl::new(self.get() - rhs.get()))
    }
}

impl TypeSub for WrappedZahl {
    fn type_sub(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.sub(rhs.my_into()),
            Types::Zahl(rhs) => self.sub(rhs),
            Types::Real(rhs) => MyInto::<WrappedReal>::my_into(self).sub(rhs),
        }
    }
}

impl Mul for Zahl {
    type Output = Zahl;

    fn mul(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Zahl::new(self.get() * rhs.get()))
    }
}

impl TypeMul for WrappedZahl {
    fn type_mul(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.mul(rhs.my_into()),
            Types::Zahl(rhs) => self.mul(rhs),
            Types::Real(rhs) => MyInto::<WrappedReal>::my_into(self).mul(rhs),
        }
    }
}

impl TypeExp for WrappedZahl {
    fn type_exp(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => MyInto::<WrappedReal>::my_into(self).exp(MyInto::<WrappedReal>::my_into(rhs)),
            Types::Zahl(rhs) => MyInto::<WrappedReal>::my_into(self).exp(MyInto::<WrappedReal>::my_into(rhs)),
            Types::Real(rhs) => MyInto::<WrappedReal>::my_into(self).exp(rhs),
        }
    }
}

impl std::fmt::Display for Zahl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}