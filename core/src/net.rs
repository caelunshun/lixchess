use bytes::{Buf, BufMut, BytesMut, Bytes};
use protobuf::ProtobufError;
use protobuf::Message;
use ring::aead::{Algorithm, OpeningKey, SealingKey, AES_256_GCM, Nonce, Aad};
use byteorder::{BigEndian, WriteBytesExt};
use std::io::Write;

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
            let mut nonce = Vec::with_capacity(12);
            nonce.write_u64::<BigEndian>(self.packet_receive_counter).unwrap();
            nonce.write(&[0, 0, 0, 0]).unwrap();
            self.packet_receive_counter += 1;

            let mut _aad = Vec::with_capacity(2);
            _aad.write_u16::<BigEndian>(packet_len).unwrap();
            let aad = Aad::from(&_aad);

            let mut buf = buf.try_mut().unwrap();

            ring::aead::open_in_place(
                &self.opening_key,
                Nonce::try_assume_unique_for_key(&nonce).unwrap(),
                aad,
                0,
                unsafe { &mut buf.bytes_mut()[2..(packet_len as usize)] }
            ).map_err(|_| Error::BadEncryption)?;

            let buf = buf.freeze();

            match protobuf::parse_from_carllerche_bytes(&buf) {
                Ok(msg) => {
                    return Ok(Some(msg));
                }
                Err(e) => match e {
                    ProtobufError::WireError(_)
                    | ProtobufError::Utf8(_)
                    | ProtobufError::MessageNotInitialized(_) => {
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
                    | ProtobufError::MessageNotInitialized(_) => {
                        return Err(Error::InvalidPacket);
                    }
                    _ => panic!("Protobuf error: {:?}", e),
                },
            }
        }
    }

    pub fn serialize_packet<M: Message>(&mut self, packet: M) -> Bytes {
        let mut bytes = BytesMut::from(packet.write_to_bytes().unwrap());

        if !self.encryption_enabled {
            let mut r = BytesMut::with_capacity(bytes.len() + 2);
            r.put_u16(bytes.len() as u16);
            r.put(bytes);
            r.freeze()
        } else {
            // Need to encrypt
            let len = bytes.len() as u16;
            let mut _aad = vec![0; 2];
            _aad.write_u16::<BigEndian>(len).unwrap();
            let aad = Aad::from(&_aad);

            let mut nonce = Vec::with_capacity(12);
            nonce.write_u64::<BigEndian>(self.packet_send_counter).unwrap();
            nonce.write(&[0, 0, 0, 0]).unwrap();
            self.packet_send_counter += 1;

            ring::aead::seal_in_place(
                &self.sealing_key,
                Nonce::try_assume_unique_for_key(&nonce).unwrap(),
                aad,
                &mut bytes,
                0
            ).unwrap();

            let mut r = BytesMut::with_capacity(bytes.len() + 2);
            r.put_u16(len);
            r.put(bytes);
            r.freeze()
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
