use crate::operations::{Add, Exp, Mul, Sub, TypeAdd, TypeExp, TypeMul, TypeSub};
use super::{natural::InnerNatural, real::{InnerReal, Real}, Types, Value};

#[derive(Debug, Clone)]
pub struct InnerZahl(i32);

impl InnerZahl {
    pub fn new(value: i32) -> Self {
        Self(value)
    }

    pub fn get(&self) -> i32 {
        self.0
    }
}

impl<T: Into<InnerNatural>> From<T> for InnerZahl {
    fn from(value: T) -> Self {
        InnerZahl::new(value.into().get() as i32)
    }
}

pub type Zahl = Value<InnerZahl>;

impl Zahl {
    pub fn new(value: i32) -> Self {
        Self::Constant(InnerZahl(value))
    }

    pub fn get_type(&self) -> &str {
        "Zahl"
    }
}

impl Add for InnerZahl {
    type Output = Zahl;

    fn add(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Zahl::new(self.get() + rhs.get()))
    }
}

impl TypeAdd for Zahl {
    fn type_add(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.add(rhs.enlarge::<InnerZahl>()),
            Types::Zahl(rhs) => self.add(rhs),
            Types::Real(rhs) => self.enlarge::<InnerReal>().add(rhs),
        }
    }
}

impl Sub for InnerZahl {
    type Output = Zahl;

    fn sub(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Zahl::new(self.get() - rhs.get()))
    }
}

impl TypeSub for Zahl {
    fn type_sub(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.sub(rhs.enlarge()),
            Types::Zahl(rhs) => self.sub(rhs),
            Types::Real(rhs) => self.enlarge::<InnerReal>().sub(rhs),
        }
    }
}

impl Mul for InnerZahl {
    type Output = Zahl;

    fn mul(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Zahl::new(self.get() * rhs.get()))
    }
}

impl TypeMul for Zahl {
    fn type_mul(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.mul(rhs.enlarge()),
            Types::Zahl(rhs) => self.mul(rhs),
            Types::Real(rhs) => self.enlarge::<InnerReal>().mul(rhs),
        }
    }
}

impl Exp for InnerZahl {
    type Output = Real;

    fn exp(self, rhs: Self) -> Result<Self::Output, String> {
        Ok(Real::new((<InnerZahl as Into<InnerReal>>::into(self).get()).powf(<InnerZahl as Into<InnerReal>>::into(rhs).get())))
    }
}

impl Exp<InnerNatural> for InnerZahl {
    type Output = Real;

    fn exp(self, rhs: InnerNatural) -> Result<Self::Output, String> {
        Ok(Real::new((<InnerZahl as Into<InnerReal>>::into(self).get()).powf(<InnerNatural as Into<InnerReal>>::into(rhs).get())))
    }
}


impl Exp<InnerReal> for InnerZahl {
    type Output = Real;

    fn exp(self, rhs: InnerReal) -> Result<Self::Output, String> {
        Ok(Real::new((<InnerZahl as Into<InnerReal>>::into(self).get()).powf(rhs.get())))
    }
}

impl TypeExp for Zahl {
    fn type_exp(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => self.exp(rhs),
            Types::Zahl(rhs) => self.exp(rhs),
            Types::Real(rhs) => self.exp(rhs),
        }
    }
}

impl std::fmt::Display for InnerZahl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl From<Zahl> for Types {
    fn from(real: Zahl) -> Self {
        Types::Zahl(real)
    }
}