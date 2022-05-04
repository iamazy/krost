use crate::{KrostError, Serializable};
use integer_encoding::{VarIntReader, VarIntWriter};
use std::io::{Read, Write};
use std::string::String as StdString;

macro_rules! impl_num_type {
    ($ty:ty, $inner:ty, $sz:expr) => {
        impl Serializable for $ty {
            fn decode<D: Read>(buf: &mut D) -> Result<Self, KrostError> {
                let mut buffer = [0u8; $sz];
                buf.read_exact(&mut buffer)?;
                Ok(Self(<$inner>::from_be_bytes(buffer)))
            }

            fn encode<E: Write>(&self, buf: &mut E) -> Result<usize, KrostError> {
                let buffer = self.0.to_be_bytes();
                buf.write_all(&buffer)?;
                Ok($sz)
            }
        }
    };
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Bool(pub bool);

impl Serializable for Bool {
    fn decode<D: Read>(buf: &mut D) -> Result<Self, KrostError> {
        let mut buffer = [0u8; 1];
        buf.read_exact(&mut buffer)?;
        match buffer[0] {
            0 => Ok(Self(false)),
            _ => Ok(Self(true)),
        }
    }

    fn encode<E: Write>(&self, buf: &mut E) -> Result<usize, KrostError> {
        match self.0 {
            true => buf.write_all(&[1])?,
            false => buf.write_all(&[0])?,
        };
        Ok(1)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Int8(pub i8);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Int16(pub i16);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Int32(pub i32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Int64(pub i64);

impl_num_type!(Int8, i8, 1);
impl_num_type!(Int16, i16, 2);
impl_num_type!(Int32, i32, 4);
impl_num_type!(Int64, i64, 8);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct VarInt(pub i32);

impl Serializable for VarInt {
    fn decode<D: Read>(buf: &mut D) -> Result<Self, KrostError> {
        let i: i64 = buf.read_varint()?;
        Ok(Self(i32::try_from(i)?))
    }

    fn encode<E: Write>(&self, buf: &mut E) -> Result<usize, KrostError> {
        Ok(buf.write_varint(self.0)?)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct VarLong(pub i64);

impl Serializable for VarLong {
    fn decode<D: Read>(buf: &mut D) -> Result<Self, KrostError> {
        Ok(Self(buf.read_varint()?))
    }

    fn encode<E: Write>(&self, buf: &mut E) -> Result<usize, KrostError> {
        Ok(buf.write_varint(self.0)?)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct UnsignedVarInt(pub u64);

impl Serializable for UnsignedVarInt {
    fn decode<D: Read>(buffer: &mut D) -> Result<Self, KrostError> {
        let mut buf = [0u8; 1];
        let mut res: u64 = 0;
        let mut shift = 0;
        loop {
            buffer.read_exact(&mut buf)?;
            let c: u64 = buf[0].into();

            res |= (c & 0x7f) << shift;
            shift += 7;

            if (c & 0x80) == 0 {
                break;
            }
            if shift > 63 {
                return Err(KrostError::Malformed(
                    "Overflow while reading unsigned varint".into(),
                ));
            }
        }
        Ok(Self(res))
    }

    fn encode<E: Write>(&self, buf: &mut E) -> Result<usize, KrostError> {
        let mut curr = self.0;
        let mut len = 0;
        loop {
            let mut c = u8::try_from(curr & 0x7f).map_err(KrostError::Overflow)?;
            curr >>= 7;
            if curr > 0 {
                c |= 0x80;
            }
            buf.write_all(&[c])?;
            len += 1;

            if curr == 0 {
                break;
            }
        }
        Ok(len)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NullableString(pub Option<StdString>);

impl Serializable for NullableString {
    fn decode<D: Read>(buf: &mut D) -> Result<Self, KrostError> {
        let len = Int16::decode(buf)?;
        match len.0 {
            l if l < -1 => Err(KrostError::Malformed(
                format!("Invalid negative length for nullable string: {}", l).into(),
            )),
            -1 => Ok(Self(None)),
            l => {
                let len = usize::try_from(l)?;
                let mut buffer = Vec::with_capacity(len);
                buf.read_exact(&mut buffer)?;
                let s =
                    StdString::from_utf8(buffer).map_err(|e| KrostError::Malformed(Box::new(e)))?;
                Ok(Self(Some(s)))
            }
        }
    }

    fn encode<E: Write>(&self, buf: &mut E) -> Result<usize, KrostError> {
        match &self.0 {
            Some(s) => {
                let len = i16::try_from(s.len()).map_err(|e| KrostError::Malformed(Box::new(e)))?;
                let len = Int16(len).encode(buf)?;
                buf.write_all(s.as_bytes())?;
                Ok(len + s.len())
            }
            None => Int16(-1).encode(buf),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct String(pub StdString);

impl Serializable for String {
    fn decode<D: Read>(buf: &mut D) -> Result<Self, KrostError> {
        let len = Int16::decode(buf)?;
        let len = usize::try_from(len.0).map_err(|e| KrostError::Malformed(Box::new(e)))?;
        let mut buffer = Vec::with_capacity(len);
        buf.read_exact(&mut buffer)?;
        let s = StdString::from_utf8(buffer).map_err(|e| KrostError::Malformed(Box::new(e)))?;
        Ok(Self(s))
    }

    fn encode<E: Write>(&self, buf: &mut E) -> Result<usize, KrostError> {
        let len = i16::try_from(self.0.len()).map_err(KrostError::Overflow)?;
        let len = Int16(len).encode(buf)?;
        buf.write_all(self.0.as_bytes())?;
        Ok(len + self.0.len())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CompactString(pub StdString);

impl Serializable for CompactString {
    fn decode<D: Read>(buf: &mut D) -> Result<Self, KrostError> {
        let len = UnsignedVarInt::decode(buf)?;
        match len.0 {
            0 => Err(KrostError::Malformed(
                "CompactString must have non-zero length".into(),
            )),
            len => {
                let len = usize::try_from(len)?;
                let len = len - 1;

                let mut buffer = Vec::with_capacity(len);
                buf.read_exact(&mut buffer)?;
                let s =
                    StdString::from_utf8(buffer).map_err(|e| KrostError::Malformed(Box::new(e)))?;
                Ok(Self(s))
            }
        }
    }

    fn encode<E: Write>(&self, buf: &mut E) -> Result<usize, KrostError> {
        let len = u64::try_from(self.0.len() + 1).map_err(KrostError::Overflow)?;
        let len = UnsignedVarInt(len).encode(buf)?;
        buf.write_all(self.0.as_bytes())?;
        Ok(len + self.0.len())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompactNullableString(pub Option<StdString>);

impl Serializable for CompactNullableString {
    fn decode<D: Read>(buf: &mut D) -> Result<Self, KrostError> {
        let len = UnsignedVarInt::decode(buf)?;
        match len.0 {
            0 => Ok(Self(None)),
            len => {
                let len = usize::try_from(len)?;
                let len = len - 1;

                let mut buffer = Vec::with_capacity(len);
                buf.read_exact(&mut buffer)?;
                let s =
                    StdString::from_utf8(buffer).map_err(|e| KrostError::Malformed(Box::new(e)))?;
                Ok(Self(Some(s)))
            }
        }
    }

    fn encode<E: Write>(&self, buf: &mut E) -> Result<usize, KrostError> {
        match &self.0 {
            Some(s) => {
                let len = u64::try_from(s.len() + 1).map_err(KrostError::Overflow)?;
                let len = UnsignedVarInt(len).encode(buf)?;
                buf.write_all(s.as_bytes())?;
                Ok(len + s.len())
            }
            None => UnsignedVarInt(0).encode(buf),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NullableBytes(pub Option<Vec<u8>>);

impl Serializable for NullableBytes {
    fn decode<D: Read>(buf: &mut D) -> Result<Self, KrostError> {
        let len = Int32::decode(buf)?;
        match len.0 {
            l if l < -1 => Err(KrostError::Malformed(
                format!("Invalid negative length for nullable bytes: {}", l).into(),
            )),
            -1 => Ok(Self(None)),
            l => {
                let len = usize::try_from(l)?;
                let mut buffer = Vec::with_capacity(len);
                buf.read_exact(&mut buffer)?;
                Ok(Self(Some(buffer)))
            }
        }
    }

    fn encode<E: Write>(&self, buf: &mut E) -> Result<usize, KrostError> {
        match &self.0 {
            Some(s) => {
                let len = i32::try_from(s.len()).map_err(|e| KrostError::Malformed(Box::new(e)))?;
                let len = Int32(len).encode(buf)?;
                buf.write_all(s)?;
                Ok(len + s.len())
            }
            None => Int32(-1).encode(buf),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TaggedFields(pub Vec<(UnsignedVarInt, Vec<u8>)>);

impl Serializable for TaggedFields {
    fn decode<D: Read>(buf: &mut D) -> Result<Self, KrostError> {
        let len = UnsignedVarInt::decode(buf)?;
        let len = usize::try_from(len.0).map_err(KrostError::Overflow)?;
        let mut res = Vec::with_capacity(len);
        for _ in 0..len {
            let tag = UnsignedVarInt::decode(buf)?;
            let data_len = UnsignedVarInt::decode(buf)?;
            let data_len = usize::try_from(data_len.0).map_err(KrostError::Overflow)?;
            let mut data_builder = Vec::with_capacity(data_len);
            buf.read_exact(&mut data_builder)?;
            res.push((tag, data_builder));
        }
        Ok(Self(res))
    }

    fn encode<E: Write>(&self, buf: &mut E) -> Result<usize, KrostError> {
        let len = u64::try_from(self.0.len()).map_err(KrostError::Overflow)?;
        let mut len = UnsignedVarInt(len).encode(buf)?;

        for (tag, data) in &self.0 {
            len += tag.encode(buf)?;
            let data_len = u64::try_from(data.len()).map_err(KrostError::Overflow)?;
            len += UnsignedVarInt(data_len).encode(buf)?;
            buf.write_all(data)?;
            len += data_len as usize;
        }

        Ok(len)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Array<T>(pub Option<Vec<T>>);

impl<T: Serializable> Serializable for Array<T> {
    fn decode<D: Read>(buf: &mut D) -> Result<Self, KrostError> {
        let len = Int32::decode(buf)?;
        if len.0 == -1 {
            Ok(Self(None))
        } else {
            let len = usize::try_from(len.0)?;
            let mut res = Vec::with_capacity(len);
            for _ in 0..len {
                res.push(T::decode(buf)?);
            }
            Ok(Self(Some(res)))
        }
    }

    fn encode<E: Write>(&self, buf: &mut E) -> Result<usize, KrostError> {
        match &self.0 {
            None => Int32(-1).encode(buf),
            Some(inner) => {
                let len = i32::try_from(inner.len())?;
                let mut len = Int32(len).encode(buf)?;
                for element in inner {
                    len += element.encode(buf)?;
                }
                Ok(len)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uuid(pub uuid::Uuid);

impl Serializable for Uuid {
    fn decode<D: Read>(buf: &mut D) -> Result<Self, KrostError> {
        let mut bytes = [0; 16];
        buf.read_exact(&mut bytes)?;
        Ok(Self(uuid::Uuid::from_bytes(bytes)))
    }

    fn encode<E: Write>(&self, buf: &mut E) -> Result<usize, KrostError> {
        buf.write_all(&self.0.as_bytes()[..])?;
        Ok(16)
    }
}
