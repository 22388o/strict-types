// Strict encoding schema library, implementing validation and parsing of strict
// encoded data against the schema.
//
// Written in 2022 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2022 by LNP/BP Standards Association, Switzerland.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::fmt::{self, Display, Formatter};

use strict_encoding::{StrictDecode, StrictEncode};

use crate::{AsciiString, OversizeError, StrictMap, StrictSet, StrictVec};

pub type TypeName = AsciiString<1, 32>;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictEncode, StrictDecode)]
#[strict_encoding(by_value, repr = u8)]
#[display(Debug)]
pub enum PrimitiveType {
    U8 = 0x00,
    U16 = 0x01,
    U32 = 0x02,
    U64 = 0x03,
    U128 = 0x04,
    U256 = 0x05,
    U512 = 0x06,
    U1024 = 0x07,

    I8 = 0x10,
    I16 = 0x11,
    I32 = 0x12,
    I64 = 0x13,
    I128 = 0x14,
    I256 = 0x15,
    I512 = 0x16,
    I1024 = 0x17,

    F16b = 0x30,
    F16 = 0x31,
    F32 = 0x32,
    F64 = 0x33,
    F80 = 0x34,
    F128 = 0x35,
    F256 = 0x36,
    F512 = 0x37,

    Unicode = 0xFE,

    Bytes = 0xFF,
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[derive(StrictEncode, StrictDecode)]
pub struct StructField {
    pub ty: DataType,
    pub optional: bool,
}

impl Display for StructField {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.ty, f)?;
        if self.optional {
            f.write_str("?")?;
        }
        Ok(())
    }
}

impl StructField {
    pub fn new(type_name: &'static str) -> Self {
        StructField {
            ty: DataType::Struct(type_name.try_into().expect("invalid struct type name")),
            optional: false,
        }
    }

    pub fn primitive(prim: PrimitiveType) -> Self {
        StructField {
            ty: DataType::Primitive(prim),
            optional: false,
        }
    }

    pub fn list(ty: impl Into<TypeRef>) -> Self {
        StructField {
            ty: DataType::List(ty.into()),
            optional: false,
        }
    }

    pub fn array(prim: PrimitiveType, size: u16) -> Self {
        StructField {
            ty: DataType::Array(size, TypeRef::Primitive(prim)),
            optional: false,
        }
    }

    pub fn optional(ty: DataType) -> Self { StructField { ty, optional: true } }
}

#[derive(Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, From)]
#[derive(StrictEncode, StrictDecode)]
pub struct StructType(StrictVec<StructField, 1>);

impl Display for StructType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let len = self.len() as usize;
        for (pos, field) in self.0.iter().enumerate() {
            Display::fmt(field, f)?;
            if pos < len - 1 {
                f.write_str(", ")?;
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictEncode, StrictDecode)]
pub enum KeyType {
    #[display(inner)]
    Primitive(PrimitiveType),

    #[display("{1}[{0}]")]
    Array(u16, PrimitiveType),
}

#[derive(Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, From)]
#[derive(StrictEncode, StrictDecode)]
pub struct UnionType(StrictSet<PrimitiveType, 2>);

impl Display for UnionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let len = self.len() as usize;
        for (pos, field) in self.0.iter().enumerate() {
            Display::fmt(field, f)?;
            if pos < len - 1 {
                f.write_str(" | ")?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display, From)]
#[derive(StrictEncode, StrictDecode)]
#[display(inner)]
pub enum TypeRef {
    #[from]
    Primitive(PrimitiveType),

    #[from]
    Named(TypeName),
}

impl From<&'static str> for TypeRef {
    fn from(value: &'static str) -> Self {
        TypeRef::Named(AsciiString::try_from(value).expect("incorrect typ name"))
    }
}

pub enum TypeConstr<T> {
    Plain(T),
    Array(u16, T),
    List(T),
    Set(T),
    Map(KeyType, T),
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[derive(StrictEncode, StrictDecode)]
pub enum DataType {
    Primitive(PrimitiveType),
    Union(TypeRef),
    Struct(TypeRef),
    Array(u16, TypeRef),
    List(TypeRef),
    Map(KeyType, TypeRef),
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Primitive(ty) => Display::fmt(ty, f),
            DataType::Union(ty) => Display::fmt(ty, f),
            DataType::Struct(ty) => Display::fmt(ty, f),
            DataType::Array(size, ty) => {
                Display::fmt(ty, f)?;
                write!(f, "*{}", size)
            }
            DataType::List(ty) => {
                Display::fmt(ty, f)?;
                f.write_str("*")
            }
            DataType::Map(key, ty) => {
                f.write_str("{")?;
                Display::fmt(key, f)?;
                f.write_str("} -> ")?;
                Display::fmt(ty, f)?;
                f.write_str("")
            }
        }
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
#[derive(StrictEncode)]
pub struct TypeSystem(StrictMap<TypeName, StructType>);

impl Display for TypeSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (name, ty) in &self.0 {
            Display::fmt(name, f)?;
            f.write_str(" :: ")?;
            Display::fmt(ty, f)?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl TypeSystem {
    pub fn new() -> Self { default!() }

    pub fn push(&mut self, name: TypeName, ty: StructType) -> Result<(), Error> {
        if self.0.contains_key(&name) {
            return Err(Error::DuplicatedType(name));
        }
        self.0.insert(name, ty)?;
        Ok(())
    }
}

#[macro_export]
macro_rules! type_system {
    ($($name:literal :: { $($field:expr),+ $(,)? }),+ $(,)?) => { {
        let mut ts = TypeSystem::new();
        $(
        let name = $name.try_into().expect("inline strict_vec literal contains invalid number of items");
        let ty =  StructType(strict_vec![$($field),+]);
        ts.push(name, ty).expect("invalid type declaration");
        )+
        ts
    } }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display, Error, From)]
#[display(doc_comments)]
pub enum Error {
    /// type `{0}` is already defined
    DuplicatedType(TypeName),

    #[from]
    #[display(inner)]
    Oversize(OversizeError),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::strict_vec;

    fn type_system() -> TypeSystem {
        type_system![
           "Transaction" :: {
                StructField::primitive(PrimitiveType::U32),
                StructField::list("Input"),
                StructField::list("Output"),
                StructField::primitive(PrimitiveType::U32),
            },
            "Input" :: {
                StructField::new("OutPoint"),
                StructField::primitive(PrimitiveType::Bytes),
                StructField::new("Witness"),
            },
            "Output" :: {
                StructField::primitive(PrimitiveType::U64),
                StructField::primitive(PrimitiveType::Bytes),
            },
            "OutPoint" :: {
                StructField::new("Txid"),
                StructField::primitive(PrimitiveType::U16),
            },
            "Txid" :: { StructField::array(PrimitiveType::U8, 32) },
            "Witness" :: { StructField::list(PrimitiveType::Bytes) },
        ]
    }

    #[test]
    fn display() {
        println!("{}", type_system());
    }
}
