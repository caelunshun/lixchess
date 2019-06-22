use bytes::{Buf, BufMut, Bytes, BytesMut};
use pb::ProtobufError;
use protobuf;
use protobuf::Message;
use ring::aead::{Algorithm, OpeningKey, SealingKey, AES_256_GCM, Nonce, Aad};
use byteorder::{BigEndian, WriteBytesExt};

static CIPHER: &Algorithm = &AES_256_GCM;

const MAX_PACKET_LEN: u16 = 1024;

pub struct NetManager {
    incoming: Option<BytesMut>,
    next_packet_len: Option<u16>,
    encryption_enabled: bool,
    opening_key: OpeningKey,
    sealing_key: SealingKey,

    /// These fields are used as nonces.
    packet_receive_counter: u64,
    packet_send_counter: u64,
}

pub enum Error {
    BadEncryption,
    InvalidPacketLength,
    InvalidPacket,
}

impl NetManager {
    pub fn process_data<M: Message>(&mut self) -> Result<Option<M>, Error> {
        let mut buf = self.incoming.take().unwrap().freeze();

        if self.encryption_enabled {
            if self.next_packet_len == None {
                // Read header: 2 bytes length prefix
                // This is stored as the additionally
                // authenticated data
                if buf.remaining() < 2 {
                    return Ok(None);
                }

                self.next_packet_len = Some(buf.get_u16());
                trace!("Packet length: {}", self.next_packet_len.unwrap());
            }

            let packet_len = self.next_packet_len.unwrap();
            if packet_len > MAX_PACKET_LEN {
                return Err(Error::InvalidPacketLength);
            }

            if buf.remaining() < packet_len as usize {
                return Ok(None); // Wait for the rest of the data to arrive
            }

            // Decrypt the data
            let mut nonce = [0u8; 12];
            (&mut nonce[..]).write_u64(self.packet_receive_counter);

            let mut _aad = Vec::with_capacity(2);
            _aad.write_u16(packet_len);
            let aad = Aad::from(&_aad);

            let mut buf = buf.try_mut().unwrap();

            ring::aead::open_in_place(
                &self.opening_key,
                Nonce::assume_unique_for_key(nonce),
                aad,
                0,
                unsafe { &mut buf.bytes_mut()[2..(packet_len as usize)] }
            );

            let buf = buf.freeze();

            match protobuf::parse_from_carllerche_bytes(&buf) {
                Ok(msg) => {
                    return Ok(Some(msg));
                }
                Err(e) => match e {
                    ProtobufError::WireError(_)
                    | ProtobufError::Utf8(_)
                    | ProtobufError::MessageNotInitialized => {
                        return Err(Error::InvalidPacket);
                    }
                    _ => panic!("Protobuf error: {:?}", e),
                },
            }
        } else {
            match protobuf::parse_from_carllerche_bytes(&buf) {
                Ok(msg) => {
                    return Ok(Some(msg));
                }
                Err(e) => match e {
                    ProtobufError::WireError(_)
                    | ProtobufError::Utf8(_)
                    | ProtobufError::MessageNotInitialized => {
                        return Err(Error::InvalidPacket);
                    }
                    _ => panic!("Protobuf error: {:?}", e),
                },
            }
        }
    }

    pub fn data(&mut self) -> &mut BytesMut {
        self.incoming.as_mut().unwrap()
    }

    pub fn enable_encryption(&mut self, rx: [u8; 32], tx: [u8; 32]) {
        self.encryption_enabled = true;
        self.opening_key = OpeningKey::new(CIPHER, &rx).unwrap();
        self.sealing_key = SealingKey::new(CIPHER, &tx).unwrap();
    }
}
