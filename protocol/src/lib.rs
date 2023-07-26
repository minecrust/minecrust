mod deserializer;

use std::io;
use bytes::{Buf, BytesMut};
use thiserror::Error;
use tokio_util::codec::Decoder;

struct MinecraftCodec;

impl Decoder for MinecraftCodec {
    type Item = ();
    type Error = MinecraftCodecError;

    fn decode(&mut self, _src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}

struct Position {
    x: i32,
    y: i32,
    z: i32,
}

struct Angle(u8);

#[derive(Debug, Error)]
enum MinecraftCodecError {
    #[error("io Error: {0}")]
    Io(#[from] io::Error),
}

const SEGMENT_BIT: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

fn read_varint(src: &mut BytesMut) -> Result<i32, MinecraftCodecError> {
    let mut value: i32 = 0;
    let mut position: i32 = 0;


    loop {
        let current_byte = src.get_u8();
        value |= ((current_byte & SEGMENT_BIT) as i32) << position;

        if (current_byte & CONTINUE_BIT) == 0 { break; }

        position += 7;

        if position >= 32 {
            return Err(
                MinecraftCodecError::Io(
                    io::Error::new(io::ErrorKind::InvalidData, "varint is longer than 32 bits")
                )
            );
        }
    }

    Ok(value)
}

fn read_varlong(src: &mut BytesMut) -> Result<i64, MinecraftCodecError> {
    let mut value = 0;
    let mut position: u8 = 0;

    loop {
        let current_byte = src.get_u8();
        value |= ((current_byte & SEGMENT_BIT) as i64) << position;

        if (current_byte & CONTINUE_BIT) == 0 { break; }

        position += 7;
        if position >= 64 {
            return Err(
                MinecraftCodecError::Io(
                    io::Error::new(io::ErrorKind::InvalidData, "varlong is longer than 64 bits")
                )
            );
        }
    }

    Ok(value)
}

fn read_bool(src: &mut BytesMut) -> Result<bool, MinecraftCodecError> {
    Ok(src.get_u8() == 1)
}

fn read_position(src: &mut BytesMut) -> Result<Position, MinecraftCodecError> {
    let coordinates = src.get_u64();

    let x = (coordinates >> 38) as i32;
    let y = (coordinates << 52 >> 52) as i32;
    let z = (coordinates << 26 >> 38) as i32;

    Ok(Position {
        y,
        x,
        z,
    })
}

fn read_angle(src: &mut BytesMut) -> Result<Angle, MinecraftCodecError> {
    let angle = src.get_u8();
    Ok(Angle(angle))
}

#[cfg(test)]
mod test {
    use bytes::{BufMut, BytesMut};
    use crate::read_varint;

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
    fn test_read_varlong() {}
}



