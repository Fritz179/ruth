use crate::operations::{Add, Exp, Mul, Sub, TypeAdd, TypeExp, TypeMul, TypeSub};
use super::{zahl::Zahl, MyFrom, MyInto, Types, Wrapper};

#[derive(Debug, Clone)]
pub struct Real(f32);

impl Real {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn get(&self) -> f32 {
        self.0
    }
}

pub type WrappedReal = Wrapper<Real>;

impl WrappedReal {
    pub fn new(value: f32) -> Self {
        Self::Constant(Real(value))
    }

    pub fn get_type(&self) -> &str {
        "Real"
    }
}

impl<From: MyInto<Zahl>> MyFrom<From> for Real {
    fn my_from(from: From) -> Real {
        Self::new(from.my_into().get() as f32)
    }
}

impl MyFrom<Real> for Real {
    fn my_from(from: Real) -> Self {
        from
    }
}

impl Add for Real {
    type Output = Real;

    fn add(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Real::new(self.get() + rhs.get()))
    }
}

impl TypeAdd for WrappedReal {
    fn type_add(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.add(rhs.my_into()),
            Types::Zahl(rhs) => self.add(rhs.my_into()),
            Types::Real(rhs) => self.add(rhs),
        }
    }
}

impl Sub for Real {
    type Output = Real;

    fn sub(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Real::new(self.get() - rhs.get()))
    }
}

impl TypeSub for WrappedReal {
    fn type_sub(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.sub(rhs.my_into()),
            Types::Zahl(rhs) => self.sub(rhs.my_into()),
            Types::Real(rhs) => self.sub(rhs),
        }
    }
}

impl Mul for Real {
    type Output = Real;

    fn mul(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Real::new(self.get() * rhs.get()))
    }
}

impl TypeMul for WrappedReal {
    fn type_mul(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.mul(rhs.my_into()),
            Types::Zahl(rhs) => self.mul(rhs.my_into()),
            Types::Real(rhs) => self.mul(rhs),
        }
    }
}

impl Exp for Real {
    type Output = Real;

    fn exp(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Real::new((self.get()).powf(rhs.get())))
    }
}

impl TypeExp for WrappedReal {
    fn type_exp(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.exp(rhs.my_into()),
            Types::Zahl(rhs) => self.exp(rhs.my_into()),
            Types::Real(rhs) => self.exp(rhs),
        }
    }
}


impl std::fmt::Display for Real {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}