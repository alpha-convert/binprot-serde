use std::{fmt::Display, io::Write};

use serde::{de::value::Error, ser::Impossible};

use crate::error::BPErr;
use crate::common::{CODE_INT16,CODE_INT32,CODE_INT64,CODE_NEG_INT8};

pub struct BPSerializer<W : Write> {
    writer : W
}

impl<W : Write> BPSerializer<W> {

    

    pub fn new(w : W) -> Self {
        BPSerializer::<W> { writer : w }

    }

    fn serialize_nat0(&mut self, v : u64) -> Result<(),std::io::Error> {
        let bytes = v.to_le_bytes();
        if v < 0x000000080 {
            self.writer.write_all(&bytes[0..1])
        } else if v < 0x000010000 {
            self.writer.write_all(&[CODE_INT16])?;
            self.writer.write_all(&bytes[0..2])
        } else if v < 0x100000000 {
            self.writer.write_all(&[CODE_INT32])?;
            self.writer.write_all(&bytes[0..4])
        } else {
            self.writer.write_all(&[CODE_INT64])?;
            self.writer.write_all(&bytes)
        }
    }


}



impl<W : Write> serde::Serializer for &mut BPSerializer<W> {
    type Ok = ();

    type Error = BPErr;

    type SerializeSeq = Impossible<Self::Ok,Self::Error>;

    type SerializeTuple = Impossible<Self::Ok,Self::Error>;

    type SerializeTupleStruct = Impossible<Self::Ok,Self::Error>;

    type SerializeTupleVariant = Impossible<Self::Ok,Self::Error>;

    type SerializeMap = Impossible<Self::Ok,Self::Error>;

    type SerializeStruct = Impossible<Self::Ok,Self::Error>;

    type SerializeStructVariant = Impossible<Self::Ok,Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        let b = v as u8;
        Ok(self.writer.write_all(&[b])?)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        let bytes = v.to_le_bytes();
        if v >= 0 {
            if v < 0x00000080 {
                Ok(self.writer.write_all(&bytes[0..1])?)
            } else if v < 0x00008000 {
                self.writer.write_all(&[CODE_INT16])?;
                Ok(self.writer.write_all(&bytes[0..2])?)
            } else if v < 0x80000000 {
                self.writer.write_all(&[CODE_INT32])?;
                Ok(self.writer.write_all(&bytes[0..4])?)
            } else {
                self.writer.write_all(&[CODE_INT64])?;
                Ok(self.writer.write_all(&bytes)?)
            }
        } else {
            if v >= -0x00000080 {
                self.writer.write_all(&bytes[0..1])?;
                Ok(self.writer.write_all(&bytes[0..1])?)
            } else if v >= -0x00008000 {
                self.writer.write_all(&[CODE_INT16])?;
                Ok(self.writer.write_all(&bytes[0..2])?)
            } else if v >= 0x80000000 {
                self.writer.write_all(&[CODE_INT32])?;
                Ok(self.writer.write_all(&bytes[0..4])?)
            } else {
                self.writer.write_all(&[CODE_INT64])?;
                Ok(self.writer.write_all(&bytes)?)
            }
        }
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Err(BPErr::NotSpecified)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Err(BPErr::NotSpecified)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Err(BPErr::NotSpecified)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Err(BPErr::NotSpecified)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(self.writer.write_all(&v.to_le_bytes())?)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let len = v.len();
        self.serialize_nat0(len as u64)?;
        Ok(self.writer.write_all(v.as_bytes())?)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(BPErr::NotSpecified)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.writer.write_all(&[0x00 as u8])?)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize {
            self.writer.write_all(&[0x01 as u8])?;
            value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.writer.write_all(&[0x00 as u8])?)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}
