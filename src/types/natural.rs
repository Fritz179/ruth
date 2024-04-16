use crate::operations::{Add, Mul, TypeAdd, TypeMul};
use super::{real::{InnerReal, Real}, Types, Value};

#[derive(Debug, Clone)]
pub struct InnerNatural(u32);

impl InnerNatural {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}

pub type Natural = Value<InnerNatural>;

impl Natural {
    pub fn new(value: u32) -> Self {
        Self::Constant(InnerNatural(value))
    }

    pub fn get_type(&self) -> &str {
        "Natural"
    }
}

impl Add for InnerNatural {
    type Output = Natural;

    fn add(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Natural::new(self.0 + rhs.0))
    }
}

impl Add<InnerReal> for InnerNatural {
    type Output = Real;

    fn add(self, rhs: InnerReal) -> Result<Self::Output, String> {
        rhs.add(self)
    }
}


impl TypeAdd for Natural {
    fn type_add(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.add(rhs),
            Types::Real(rhs) => self.add(rhs),
        }
    }
}

impl Mul for InnerNatural {
    type Output = Natural;

    fn mul(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Natural::new(self.0 * rhs.0))
    }
}

impl Mul<InnerReal> for InnerNatural {
    type Output = Real;

    fn mul(self, rhs: InnerReal) -> Result<Self::Output, String> {
        rhs.mul(self)
    }
}

impl TypeMul for Natural {
    fn type_mul(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.mul(rhs),
            Types::Real(rhs) => self.mul(rhs),
        }
    }
}

impl std::fmt::Display for InnerNatural {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Natural> for Types {
    fn from(real: Natural) -> Self {
        Types::Natural(real)
    }
}