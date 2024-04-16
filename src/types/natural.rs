use crate::{operations::{Add, Exp, Mul, TypeAdd, TypeExp, TypeMul}, WrappedReal, WrappedZahl};
use super::{MyFrom, MyInto, Types, Wrapper};

#[derive(Debug, Clone)]
pub struct Natural(u32);

impl Natural {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}

pub type WrappedNatural = Wrapper<Natural>;

impl WrappedNatural {
    pub fn new(value: u32) -> Self {
        Self::Constant(Natural(value))
    }

    pub fn get_type(&self) -> &str {
        "Natural"
    }
}

impl MyFrom<Natural> for Natural {
    fn my_from(from: Natural) -> Self {
        from
    }
}

impl Add for Natural {
    type Output = Natural;

    fn add(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Natural::new(self.get() + rhs.get()))
    }
}

impl TypeAdd for WrappedNatural {
    fn type_add(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.add(rhs),
            Types::Zahl(rhs) => MyInto::<WrappedZahl>::my_into(self).add(rhs),
            Types::Real(rhs) => MyInto::<WrappedReal>::my_into(self).add(rhs),
        }
    }
}

impl Mul for Natural {
    type Output = Natural;

    fn mul(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Natural::new(self.get() * rhs.get()))
    }
}

impl TypeMul for WrappedNatural {
    fn type_mul(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.mul(rhs),
            Types::Zahl(rhs) => MyInto::<WrappedZahl>::my_into(self).mul(rhs),
            Types::Real(rhs) => MyInto::<WrappedReal>::my_into(self).mul(rhs),
        }
    }
}

impl Exp for Natural {
    type Output = Natural;

    fn exp(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Natural::new((self.get()).pow(rhs.get())))
    }
}

impl TypeExp for WrappedNatural {
    fn type_exp(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.exp(rhs),
            Types::Zahl(rhs) => MyInto::<WrappedReal>::my_into(self).exp(MyInto::<WrappedReal>::my_into(rhs)),
            Types::Real(rhs) => MyInto::<WrappedReal>::my_into(self).exp(rhs),
        }
    }
}

impl std::fmt::Display for Natural {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}