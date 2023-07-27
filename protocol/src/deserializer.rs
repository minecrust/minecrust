use std::fmt::Display;
use bytes::{Buf, BytesMut};
use serde::{Deserialize, Deserializer};
use serde::de::Visitor;
use crate::error::ProtocolError;
use crate::{read_bool, read_byte, read_double, read_float, read_int, read_long, read_short, read_string, read_unsigned_byte, read_unsigned_short, read_varint, read_varlong};

impl serde::de::Error for ProtocolError {
    fn custom<T>(msg: T) -> Self where T: Display {
        todo!()
    }
}

pub struct PacketDeserializer<'de> {
    bytes: &'de mut BytesMut,
}

impl<'de> PacketDeserializer<'de> {
    pub fn from_bytes_mut(bytes: &'de mut BytesMut) -> Self {
        PacketDeserializer { bytes }
    }
}

pub fn from_bytes_mut<'a, T>(bytes: &'a mut BytesMut) -> Result<T, ProtocolError>
    where T: Deserialize<'a> {
    let mut deserializer = PacketDeserializer::from_bytes_mut(bytes);
    let t = T::deserialize(&mut deserializer)?;
    if !deserializer.bytes.has_remaining() {
        Ok(t)
    } else {
        Err(ProtocolError::RemainingBytes(deserializer.bytes.remaining()))
    }
}

impl<'de, 'a> Deserializer<'de> for &'a mut PacketDeserializer<'de> {
    type Error = ProtocolError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        read_bool(self.bytes).and_then(|v| visitor.visit_bool(v))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        read_byte(self.bytes).and_then(|v| visitor.visit_i8(v))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        read_short(self.bytes).and_then(|v| visitor.visit_i16(v))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        read_int(self.bytes).and_then(|v| visitor.visit_i32(v))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        read_long(self.bytes).and_then(|v| visitor.visit_i64(v))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        read_unsigned_byte(self.bytes).and_then(|v| visitor.visit_u8(v))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        read_unsigned_short(self.bytes).and_then(|v| visitor.visit_u16(v))
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(ProtocolError::UnsupportedType("u32"))
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(ProtocolError::UnsupportedType("u64"))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        read_float(self.bytes).and_then(|v| visitor.visit_f32(v))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        read_double(self.bytes).and_then(|v| visitor.visit_f64(v))
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(ProtocolError::UnsupportedType("char"))
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(ProtocolError::UnsupportedType("str"))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        read_string(self.bytes).and_then(|v| visitor.visit_string(v))
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(ProtocolError::UnsupportedType("bytes"))
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(ProtocolError::UnsupportedType("str"))
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(ProtocolError::UnsupportedType("unit"))
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(ProtocolError::UnsupportedType("unit struct"))
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        match name {
            "MC_VARINT" => {
                read_varint(self.bytes).and_then(|v| visitor.visit_i32(v))
            }
            "MC_VARLONG" => {
                read_varlong(self.bytes).and_then(|v| visitor.visit_i64(v))
            }
            _ => {
                Err(ProtocolError::UnsupportedType(name))
            }
        }
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(ProtocolError::UnsupportedType("seq"))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(ProtocolError::UnsupportedType("tuple struct"))
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(ProtocolError::UnsupportedType("map"))
    }

    fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(ProtocolError::UnsupportedType("struct"))
    }

    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(ProtocolError::UnsupportedType("enum"))
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(ProtocolError::UnsupportedType("identifier"))
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(ProtocolError::UnsupportedType("ignored any"))
    }
}