use crate::operations::{Add, Exp, Mul, Sub, TypeAdd, TypeExp, TypeMul, TypeSub};
use super::{zahl::InnerZahl, Types, Value};

#[derive(Debug, Clone)]
pub struct InnerReal(f32);

impl InnerReal {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn get(&self) -> f32 {
        self.0
    }
}

pub type Real = Value<InnerReal>;

impl Real {
    pub fn new(value: f32) -> Self {
        Self::Constant(InnerReal(value))
    }

    pub fn get_type(&self) -> &str {
        "Real"
    }
}

impl<T: Into<InnerZahl>> From<T> for InnerReal {
    fn from(value: T) -> Self {
        InnerReal::new(value.into().get() as f32)
    }
}

impl Add for InnerReal {
    type Output = Real;

    fn add(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Real::new(self.get() + rhs.get()))
    }
}

impl TypeAdd for Real {
    fn type_add(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.add(rhs.enlarge()),
            Types::Zahl(rhs) => self.add(rhs.enlarge()),
            Types::Real(rhs) => self.add(rhs),
        }
    }
}

impl Sub for InnerReal {
    type Output = Real;

    fn sub(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Real::new(self.get() - rhs.get()))
    }
}

impl TypeSub for Real {
    fn type_sub(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.sub(rhs.enlarge()),
            Types::Zahl(rhs) => self.sub(rhs.enlarge()),
            Types::Real(rhs) => self.sub(rhs),
        }
    }
}

impl Mul for InnerReal {
    type Output = Real;

    fn mul(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Real::new(self.get() * rhs.get()))
    }
}

impl TypeMul for Real {
    fn type_mul(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.mul(rhs.enlarge()),
            Types::Zahl(rhs) => self.mul(rhs.enlarge()),
            Types::Real(rhs) => self.mul(rhs),
        }
    }
}

impl Exp for InnerReal {
    type Output = Real;

    fn exp(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Real::new((self.get()).powf(rhs.get())))
    }
}

impl TypeExp for Real {
    fn type_exp(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.exp(rhs.enlarge()),
            Types::Zahl(rhs) => self.exp(rhs.enlarge()),
            Types::Real(rhs) => self.exp(rhs),
        }
    }
}


impl std::fmt::Display for InnerReal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl From<Real> for Types {
    fn from(real: Real) -> Self {
        Types::Real(real)
    }
}