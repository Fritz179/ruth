use crate::{Add, Mul, TypeAdd, TypeMul, Types, Value};

#[derive(Debug, Clone)]
pub struct InnerReal(f64);

pub type Real = Value<InnerReal>;

impl Real {
    pub fn new(value: f64) -> Self {
        Self::Constant(InnerReal(value))
    }

    pub fn get_type(&self) -> &str {
        "Real"
    }
}

impl Add for InnerReal {
    type Output = Real;

    fn add(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Real::new(self.0 + rhs.0))
    }
}

impl TypeAdd for Real {
    fn type_add(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Real(rhs) => self.add(rhs),
        }
    }
}

impl Mul for InnerReal {
    type Output = Real;

    fn mul(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Real::new(self.0 * rhs.0))
    }
}


impl TypeMul for Real {
    fn type_mul(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Real(rhs) => self.mul(rhs),
        }
    }
}

impl std::fmt::Display for InnerReal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Real> for Types {
    fn from(real: Real) -> Self {
        Types::Real(real)
    }
}