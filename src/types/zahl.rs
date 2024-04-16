use crate::{operations::{Add, Exp, Mul, Sub, TypeAdd, TypeExp, TypeMul, TypeSub}, MyInto};
use super::{natural::InnerNatural, real::Real, Types, Value, MyFrom};

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

impl<From: MyInto<InnerNatural>> MyFrom<From> for InnerZahl {
    fn my_from(from: From) -> InnerZahl {
        Self::new(from.my_into().get() as i32)
    }
}

impl MyFrom<InnerZahl> for InnerZahl {
    fn my_from(from: InnerZahl) -> Self {
        from
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
            Types::Natural(rhs) => self.add(rhs.my_into()),
            Types::Zahl(rhs) => self.add(rhs),
            Types::Real(rhs) => MyInto::<Real>::my_into(self).add(rhs),
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
            Types::Natural(rhs) => self.sub(rhs.my_into()),
            Types::Zahl(rhs) => self.sub(rhs),
            Types::Real(rhs) => MyInto::<Real>::my_into(self).sub(rhs),
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
            Types::Natural(rhs) => self.mul(rhs.my_into()),
            Types::Zahl(rhs) => self.mul(rhs),
            Types::Real(rhs) => MyInto::<Real>::my_into(self).mul(rhs),
        }
    }
}

impl TypeExp for Zahl {
    fn type_exp(self, rhs: Types) -> Result<Types, String> {
        match rhs {
            Types::Natural(rhs) => MyInto::<Real>::my_into(self).exp(MyInto::<Real>::my_into(rhs)),
            Types::Zahl(rhs) => MyInto::<Real>::my_into(self).exp(MyInto::<Real>::my_into(rhs)),
            Types::Real(rhs) => MyInto::<Real>::my_into(self).exp(rhs),
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