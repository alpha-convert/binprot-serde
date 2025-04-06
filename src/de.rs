use core::str;
use std::{ascii, fmt::Display, io::Read, str::from_utf8};
use serde::{de::value::Error, ser::Impossible};

use crate::{common::{CODE_INT16, CODE_INT32, CODE_INT64}, error::BPErr};

pub struct BPDeserializer<R : Read> {
    reader : R
}

impl<R : Read> BPDeserializer<R> {
    fn read_byte(&mut self) -> Result<u8,BPErr> {
        let b = 0 as u8;
        self.reader.read_exact(&mut [b])?;
        Ok(b)
    }

    pub fn read_nat0(&mut self) -> Result<usize,BPErr> {
        let header = self.read_byte()?;
        if header == CODE_INT16 {
            let mut b16 = [0,0] as [u8;2];
            self.reader.read_exact(&mut b16)?;
            Ok(u16::from_le_bytes(b16) as usize)
        } else if header == CODE_INT32 {
            let mut b32 = [0,0,0,0] as [u8;4];
            self.reader.read_exact(&mut b32)?;
            Ok(u32::from_le_bytes(b32) as usize)
        } else if header == CODE_INT64 {
            let mut b64 = [0,0,0,0,0,0,0,0] as [u8;8];
            self.reader.read_exact(&mut b64)?;
            Ok(usize::from_le_bytes(b64))
        } else {
            let b = self.read_byte()?;
            Ok(b as usize)
        }
    }

    pub fn read_i64(&mut self) -> Result<i64,BPErr> {
        Ok(0)
    }

}

impl<'a, 'de, R : Read> serde::Deserializer<'de> for &'a mut BPDeserializer<R> {
    type Error = BPErr;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let b = self.read_byte()?;
        if b == 0x00 {
            visitor.visit_bool(false)
        } else if b == 0x01 {
            visitor.visit_bool(true)
        } else {
            Err(BPErr::NotBool)
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        visitor.visit_i8(self.read_i64()?.try_into()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        visitor.visit_i16(self.read_i64()?.try_into()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        visitor.visit_i32(self.read_i64()?.try_into()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        visitor.visit_i64(self.read_i64()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        Err(BPErr::NotSpecified)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        Err(BPErr::NotSpecified)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        Err(BPErr::NotSpecified)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        Err(BPErr::NotSpecified)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let mut b = [0 as u8];
        self.reader.read_exact(&mut b)?;
        match std::ascii::Char::from_u8(b[0]) {
            None => Err(BPErr::NonAsciiChar),
            Some(c) => visitor.visit_char(c.to_char())
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let len = self.read_nat0()?;
        let mut buf = vec![0 as u8; len];
        self.reader.read_exact(&mut buf)?;
        visitor.visit_str(str::from_utf8(&mut buf)?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let len = self.read_nat0()?;
        let mut buf = vec![0 as u8; len];
        self.reader.read_exact(&mut buf)?;
        visitor.visit_string(str::from_utf8(&mut buf)?.to_string())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let header = self.read_byte()?;
        if header == 0x00 {
            visitor.visit_none()
        } else if header == 0x01 {
            visitor.visit_some(self)
        } else {
            Err(BPErr::NotOption)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let b = self.read_byte()?;
        if b == 0x00 {
            visitor.visit_unit()
        } else {
            Err(BPErr::NotUnit)
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let v = self.read_byte()?;
        if v == 0 {
            visitor.visit_unit()
        } else {
            Err(BPErr::NotUnit)
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        todo!()
    }
        
}