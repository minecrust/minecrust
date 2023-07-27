use std::io;
use std::ops::Index;
use bytes::{Buf, BytesMut};
use crate::error::ProtocolError;

const SEGMENT_BIT: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

pub(crate) fn read_varint_without_cursor(src: &mut BytesMut) -> Result<(i32, usize), ProtocolError> {
    let mut value: i32 = 0;
    let mut position: i32 = 0;
    let mut consumed_bytes: usize = 0;

    loop {
        let current_byte = src.get(consumed_bytes)
            .ok_or::<ProtocolError>(
                io::Error::new(io::ErrorKind::InvalidData, "not enough bytes to read varint").into()
            )?;
        consumed_bytes += 1;

        value |= ((current_byte & SEGMENT_BIT) as i32) << position;

        if (current_byte & CONTINUE_BIT) == 0 { break; }

        position += 7;

        if position >= 32 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "varint is longer than 32 bits").into());
        }
    }

    Ok((value, consumed_bytes))
}

pub(crate) fn read_varint(src: &mut BytesMut) -> Result<i32, ProtocolError> {
    let mut value: i32 = 0;
    let mut position: i32 = 0;

    loop {
        let current_byte = src.get_u8();
        value |= ((current_byte & SEGMENT_BIT) as i32) << position;

        if (current_byte & CONTINUE_BIT) == 0 { break; }

        position += 7;

        if position >= 32 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "varint is longer than 32 bits").into());
        }
    }

    Ok(value)
}

pub(crate) fn read_varlong(src: &mut BytesMut) -> Result<i64, ProtocolError> {
    let mut value = 0;
    let mut position: u8 = 0;

    loop {
        let current_byte = src.get_u8();
        value |= ((current_byte & SEGMENT_BIT) as i64) << position;

        if (current_byte & CONTINUE_BIT) == 0 { break; }

        position += 7;
        if position >= 64 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "varlong is longer than 64 bits").into());
        }
    }

    Ok(value)
}

pub(crate) fn read_bool(src: &mut BytesMut) -> Result<bool, ProtocolError> {
    Ok(src.get_u8() == 1)
}

pub(crate) fn read_byte(src: &mut BytesMut) -> Result<i8, ProtocolError> {
    Ok(src.get_i8())
}

pub(crate) fn read_unsigned_byte(src: &mut BytesMut) -> Result<u8, ProtocolError> {
    Ok(src.get_u8())
}

pub(crate) fn read_short(src: &mut BytesMut) -> Result<i16, ProtocolError> {
    Ok(src.get_i16())
}

pub(crate) fn read_unsigned_short(src: &mut BytesMut) -> Result<u16, ProtocolError> {
    Ok(src.get_u16())
}

pub(crate) fn read_int(src: &mut BytesMut) -> Result<i32, ProtocolError> {
    Ok(src.get_i32())
}

pub(crate) fn read_long(src: &mut BytesMut) -> Result<i64, ProtocolError> {
    Ok(src.get_i64())
}

pub(crate) fn read_float(src: &mut BytesMut) -> Result<f32, ProtocolError> {
    Ok(src.get_f32())
}

pub(crate) fn read_double(src: &mut BytesMut) -> Result<f64, ProtocolError> {
    Ok(src.get_f64())
}

pub(crate) fn read_string(src: &mut BytesMut) -> Result<String, ProtocolError> {
    let length = read_varint(src)? as usize;
    let chars = src.index(..length);
    Ok(String::from_utf8_lossy(chars).into())
}

#[cfg(test)]
mod test {
    use bytes::{BufMut, BytesMut};
    use super::{read_varint, read_varlong};

    #[test]
    fn test_read_varint() {
        let mut bytes_mut = BytesMut::new();

        // Value 	Hex bytes 	Decimal bytes
        // 0 	0x00 	0
        bytes_mut.put_u8(0x00u8);

        // 1 	0x01 	1
        bytes_mut.put_u8(0x01u8);

        // 2 	0x02 	2
        bytes_mut.put_u8(0x02u8);

        // 127 	0x7f 	127
        bytes_mut.put_u8(0x7Fu8);

        // 128 	0x80 0x01 	128 1
        bytes_mut.put_u8(0x80u8);
        bytes_mut.put_u8(0x01u8);

        // 255 	0xff 0x01 	255 1
        bytes_mut.put_u8(0xFFu8);
        bytes_mut.put_u8(0x01u8);

        // 25565 	0xdd 0xc7 0x01 	221 199 1
        bytes_mut.put_u8(0xDDu8);
        bytes_mut.put_u8(0xC7u8);
        bytes_mut.put_u8(0x01u8);

        // 2097151 	0xff 0xff 0x7f 	255 255 127
        bytes_mut.put_u8(0xFFu8);
        bytes_mut.put_u8(0xFFu8);
        bytes_mut.put_u8(0x7Fu8);

        // 2147483647 	0xff 0xff 0xff 0xff 0x07 	255 255 255 255 7
        bytes_mut.put_u8(0xFFu8);
        bytes_mut.put_u8(0xFFu8);
        bytes_mut.put_u8(0xFFu8);
        bytes_mut.put_u8(0xFFu8);
        bytes_mut.put_u8(0x07u8);

        // -1 	0xff 0xff 0xff 0xff 0x0f 	255 255 255 255 15
        bytes_mut.put_u8(0xFFu8);
        bytes_mut.put_u8(0xFFu8);
        bytes_mut.put_u8(0xFFu8);
        bytes_mut.put_u8(0xFFu8);
        bytes_mut.put_u8(0x0Fu8);

        // -2147483648 	0x80 0x80 0x80 0x80 0x08 	128 128 128 128 8
        bytes_mut.put_u8(0x80u8);
        bytes_mut.put_u8(0x80u8);
        bytes_mut.put_u8(0x80u8);
        bytes_mut.put_u8(0x80u8);
        bytes_mut.put_u8(0x08u8);

        assert_eq!(0, read_varint(&mut bytes_mut).unwrap());
        assert_eq!(1, read_varint(&mut bytes_mut).unwrap());
        assert_eq!(2, read_varint(&mut bytes_mut).unwrap());
        assert_eq!(127, read_varint(&mut bytes_mut).unwrap());
        assert_eq!(128, read_varint(&mut bytes_mut).unwrap());
        assert_eq!(255, read_varint(&mut bytes_mut).unwrap());
        assert_eq!(25565, read_varint(&mut bytes_mut).unwrap());
        assert_eq!(2097151, read_varint(&mut bytes_mut).unwrap());
        assert_eq!(2147483647, read_varint(&mut bytes_mut).unwrap());
        assert_eq!(-1, read_varint(&mut bytes_mut).unwrap());
        assert_eq!(-2147483648, read_varint(&mut bytes_mut).unwrap());
    }

    #[test]
    fn test_read_varlong() {
        let mut bytes_mut = BytesMut::new();
        // 0x00
        bytes_mut.put_u8(0x00);

        // 0x01
        bytes_mut.put_u8(0x01);

        // 0x02
        bytes_mut.put_u8(0x02);

        // 0x7f
        bytes_mut.put_u8(0x7f);

        // 0x80 0x01
        bytes_mut.put_u8(0x80);
        bytes_mut.put_u8(0x01);

        // 0xff 0x01
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0x01);

        // 0xff 0xff 0xff 0xff 0x07
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0x07);

        // 0xff 0xff 0xff 0xff 0xff 0xff 0xff 0xff 0x7f
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0x7f);

        // 0xff 0xff 0xff 0xff 0xff 0xff 0xff 0xff 0xff 0x01
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0x01);

        // 0x80 0x80 0x80 0x80 0xf8 0xff 0xff 0xff 0xff 0x01
        bytes_mut.put_u8(0x80);
        bytes_mut.put_u8(0x80);
        bytes_mut.put_u8(0x80);
        bytes_mut.put_u8(0x80);
        bytes_mut.put_u8(0xf8);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0xff);
        bytes_mut.put_u8(0x01);

        // 0x80 0x80 0x80 0x80 0x80 0x80 0x80 0x80 0x80 0x01
        bytes_mut.put_u8(0x80);
        bytes_mut.put_u8(0x80);
        bytes_mut.put_u8(0x80);
        bytes_mut.put_u8(0x80);
        bytes_mut.put_u8(0x80);
        bytes_mut.put_u8(0x80);
        bytes_mut.put_u8(0x80);
        bytes_mut.put_u8(0x80);
        bytes_mut.put_u8(0x80);
        bytes_mut.put_u8(0x01);

        assert_eq!(0, read_varlong(&mut bytes_mut).unwrap());
        assert_eq!(1, read_varlong(&mut bytes_mut).unwrap());
        assert_eq!(2, read_varlong(&mut bytes_mut).unwrap());
        assert_eq!(127, read_varlong(&mut bytes_mut).unwrap());
        assert_eq!(128, read_varlong(&mut bytes_mut).unwrap());
        assert_eq!(255, read_varlong(&mut bytes_mut).unwrap());
        assert_eq!(2147483647, read_varlong(&mut bytes_mut).unwrap());
        assert_eq!(9223372036854775807, read_varlong(&mut bytes_mut).unwrap());
        assert_eq!(-1, read_varlong(&mut bytes_mut).unwrap());
        assert_eq!(-2147483648, read_varlong(&mut bytes_mut).unwrap());
        assert_eq!(-9223372036854775808, read_varlong(&mut bytes_mut).unwrap());
    }
}
