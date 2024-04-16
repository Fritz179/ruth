use crate::operations::{Add, Exp, Mul, TypeAdd, TypeExp, TypeMul};
use super::{Types, Value, natural::InnerNatural};

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

impl Add for InnerReal {
    type Output = Real;

    fn add(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Real::new(self.0 + rhs.0))
    }
}

impl Add<InnerNatural> for InnerReal {
    type Output = Real;

    fn add(self, rhs: InnerNatural) -> Result<Self::Output, String> {
        Ok(Real::new(self.get() + rhs.get() as f32))
    }
}

impl TypeAdd for Real {
    fn type_add(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.add(rhs),
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

impl Mul<InnerNatural> for InnerReal {
    type Output = Real;

    fn mul(self, rhs: InnerNatural) -> Result<Self::Output, String> {
        Ok(Real::new(self.0 * rhs.get() as f32))
    }
}


impl TypeMul for Real {
    fn type_mul(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.mul(rhs),
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

impl Exp<InnerNatural> for InnerReal {
    type Output = Real;

    fn exp(self, rhs: InnerNatural) -> Result<Self::Output, String> {
        Ok(Real::new(self.get().powf(rhs.get() as f32)))
    }
}

impl TypeExp for Real {
    fn type_exp(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.exp(rhs),
            Types::Real(rhs) => self.exp(rhs),
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