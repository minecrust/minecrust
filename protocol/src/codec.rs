use bytes::{Buf, BytesMut};
use tokio_util::codec::Decoder;
use crate::error::ProtocolError;
use crate::reader;

enum State {
    Handshaking = 0,
    Status = 1,
    Login = 2,
    Play = 3,
}

pub struct MinecraftCodec {
    state: State,
}

impl MinecraftCodec {
    pub fn new() -> Self {
        Self {
            state: State::Handshaking
        }
    }
}

impl Decoder for MinecraftCodec {
    type Item = ();
    type Error = ProtocolError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }

        let (packet_length, consumed_bytes) = reader::read_varint_without_cursor(src)?;

        if src.len() - consumed_bytes < packet_length as usize {
            src.reserve(packet_length as usize + consumed_bytes - src.len());
            return Ok(None);
        }

        src.advance(consumed_bytes);
        let mut packet_with_id = src.split_to(packet_length as usize);

        let packet_id = reader::read_varint(&mut packet_with_id)?;
        log::debug!("packet id: {}", packet_id);

        Ok(Some(()))
    }
}
