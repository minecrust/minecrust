use std::fmt::Formatter;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer};

pub struct VarInt(i32);

impl<'de> Deserialize<'de> for VarInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct VarIntVisitor;
        impl<'de> Visitor<'de> for VarIntVisitor {
            type Value = VarInt;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("varint")
            }

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where E: Error {
                Ok(VarInt(v))
            }
        }
        deserializer.deserialize_newtype_struct("MC_VARINT", VarIntVisitor)
    }
}

pub struct VarLong(i64);

impl<'de> Deserialize<'de> for VarLong {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct VarLongVisitor;
        impl<'de> Visitor<'de> for VarLongVisitor {
            type Value = VarLong;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("varlong")
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: Error {
                Ok(VarLong(v))
            }
        }
        deserializer.deserialize_newtype_struct("MC_VARLONG", VarLongVisitor)
    }
}
