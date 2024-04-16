use crate::operations::{Add, Exp, Mul, TypeAdd, TypeExp, TypeMul};
use super::{real::InnerReal, zahl::InnerZahl, Types, Value};

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
        Ok(Natural::new(self.get() + rhs.get()))
    }
}

impl TypeAdd for Natural {
    fn type_add(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.add(rhs),
            Types::Zahl(rhs) => self.enlarge::<InnerZahl>().add(rhs),
            Types::Real(rhs) => self.enlarge::<InnerReal>().add(rhs),
        }
    }
}

impl Mul for InnerNatural {
    type Output = Natural;

    fn mul(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Natural::new(self.get() * rhs.get()))
    }
}

impl TypeMul for Natural {
    fn type_mul(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.mul(rhs),
            Types::Zahl(rhs) => self.enlarge::<InnerZahl>().mul(rhs),
            Types::Real(rhs) => self.enlarge::<InnerReal>().mul(rhs),
        }
    }
}

impl Exp for InnerNatural {
    type Output = Natural;

    fn exp(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Natural::new((self.get()).pow(rhs.get())))
    }
}

impl TypeExp for Natural {
    fn type_exp(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.exp(rhs),
            Types::Zahl(rhs) => self.enlarge::<InnerZahl>().exp(rhs),
            Types::Real(rhs) => self.enlarge::<InnerReal>().exp(rhs),
        }
    }
}

impl std::fmt::Display for InnerNatural {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl From<Natural> for Types {
    fn from(real: Natural) -> Self {
        Types::Natural(real)
    }
}