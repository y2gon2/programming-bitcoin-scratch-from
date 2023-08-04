
use std::io::{Read, BufReader, Write};
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use std::net::TcpStream;
use std::collections::HashMap;

use rand::{thread_rng, Rng};
use log::info;

use crate::models::helper::*;
use crate::models::block::Block;

const NETWORK_MAGIC: [u8; 4] = [0xf9, 0xbe, 0xb4, 0xd9];
const TESTNET_NETWORK_MAGIC: [u8; 4] = [0x0b, 0x11, 0x09, 0x07];


//------------------------------------------------------------------------------
//                                 NetworkEnvelope
//------------------------------------------------------------------------------

/// P2P network 통신 패킷  전송 내용인 payload 를 포함하고 있는 Network Message
#[derive(Clone, Debug)]
pub struct NetworkEnvelope {
    magic: [u8; 4],
    command: Vec<u8>,
    payload: Vec<u8>, 
}

impl NetworkEnvelope {
    pub fn new(command: Vec<u8>, payload: Vec<u8>, testnet: bool) -> Self {
        let mut magic = NETWORK_MAGIC;
        if testnet {
            magic = TESTNET_NETWORK_MAGIC;
        }

        Self {
            magic,
            command,
            payload,
        }
    }

    /// Takes a stream and creates a NetworkEnvelope
    /// 
    /// 송수신 되는 Network message 의 예시
    /// 
    /// f9beb4d976657273696f6e0000000000650000005f1a69d2721101000100000000000000\
    /// bc8f5e5400000000010000000000000000000000000000000000ffffc61b6409208d0100\
    /// 00000000000000000000000000000000ffffcb0071c0208d128035cbc97953f80f2f5361\
    /// 746f7368693a302e392e332fcf05050001
    /// 
    /// 1. f9beb4d9 (4bytes): network magic 
    ///    통신연결이 끊어졌을 때 재접속을 위해 신호의 시작점을 알아내는데 유용
    ///    네트워크 식별 (ex. Litecoin 여부, mainnet (f9beb4d9) testnet (0b110907) 구분)
    /// 
    /// 2. 76657273696f6e0000000000 (12 bytes): command 
    ///    payload 정보 제공
    ///    ex. version 정보만 담고 있는 경우, 남은 공간은 0x00 으로 채움
    /// 
    /// 3. 65000000 (4 bytes): payload length (little-endian)
    ///    가변길이를 가진 payload 의 길이 정보를 담고 있음
    ///    4 bytes 로 최대 4GB 길이까지 표현가능하지만 실제 payload 의 최대 길이는 32MB
    /// 
    /// 4. 5f1a69d2 (4 bytes): checksum
    ///    payload 의 hash256 hash 값의 앞 4 bytes 를 표시
    ///    일반적 'checksum' 오류 정정 기능을 포함하도록 설계되지만, bitcoin 에서 사용하는
    ///    hash256 알고리즘에는 해당 기능은 없음
    /// 
    /// 5. 7211...001 : payload
    pub fn parse<R: Read>(mut reader: R, testnet: bool) -> Result<Self, Box<dyn Error>> {
        let mut magic = [0u8; 4];
        let _= reader.read_exact(&mut magic);

        if magic == [0u8; 4] {
            return Err("Connection reset!".into())
        }   

        #[allow(unused_assignments)]
        let mut expected_magic = [0u8; 4];
        if testnet {
            expected_magic = TESTNET_NETWORK_MAGIC;
        } else {
            expected_magic = NETWORK_MAGIC;
        }

        if magic != expected_magic {
            return Err(format!("matic is not right {:?} vs {:?}", magic, expected_magic).into())
        }
        
        let mut command_array = [0u8; 12];
        let _= reader.read_exact(&mut command_array);
        let mut command_len = 12usize;
        for i in (0..12).rev() {
            if command_array[i] != 0 { 
                command_len = i + 1;
                break;
            }
        }
        let command: Vec<u8> = command_array[..command_len].try_into().unwrap();
        
        let mut payload_length_buf = [0u8; 4];
        let _= reader.read_exact(&mut payload_length_buf);
        let _payload_length = u32::from_le_bytes(payload_length_buf);

        let mut checksum_buf = [0u8; 4];
        let _= reader.read_exact(&mut checksum_buf);

        let mut payload = Vec::<u8>::new();
        let _= reader.read_to_end(&mut payload);

        Ok(Self {
            magic,
            command,
            payload,
        })
    }

    /// Returns the byte serialization of the entire networ message
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();

        let mut magic: Vec<u8> = self.magic.try_into().unwrap();
        result.append(&mut magic);

        let command_len = self.command.len();
        let mut zeros = vec![0u8; 12 - command_len];
        result.append(&mut self.command.clone());
        result.append(&mut zeros);

        let payload_length = self.payload.len() as u32;
        let payload_length_array = payload_length.to_le_bytes();
        let mut payload_length_vec = payload_length_array.try_into().unwrap();
        result.append(&mut payload_length_vec);

        let mut hash256 = hash256(&self.payload)[0..4].to_vec();
        result.append(&mut hash256);

        result.append(&mut self.payload.clone());

        result
    }

    /// Returns a stream for parsing the payload
    pub fn strea(&self) -> Vec<u8> {
        return self.payload.clone()
    }
}


#[cfg(test)]
mod test_networkenvelope {
    use std::io::Cursor;

    use super::*;
    use crate::models::helper::str_to_vec_u8;

    const MSG1: &str = "f9beb4d976657261636b000000000000000000005df6e0e2";
    const MSG2: &str = "f9beb4d976657273696f6e0000000000650000005f1a69d272\
        1101000100000000000000bc8f5e54000000000100000000000000000000000000\
        00000000ffffc61b6409208d010000000000000000000000000000000000ffffcb\
        0071c0208d128035cbc97953f80f2f5361746f7368693a302e392e332fcf05050001";
    
    #[test]
    fn test_parse() {
        let msg1 = str_to_vec_u8(MSG1);
        let cursor = Cursor::new(msg1);
        let envelope = NetworkEnvelope::parse(cursor, false).unwrap();
        assert_eq!(envelope.command, [b'v', b'e', b'r', b'a', b'c', b'k']);
        assert_eq!(envelope.payload, Vec::<u8>::new());

        let msg2 = str_to_vec_u8(MSG2);
        let mut cursor2 = Cursor::new(msg2.clone());
        let envelope = NetworkEnvelope::parse(&mut cursor2, false).unwrap();
        
        assert_eq!(envelope.command, vec![b'v', b'e', b'r', b's', b'i', b'o', b'n']);
        assert_eq!(envelope.payload, msg2[24..].to_vec());
    }
    
    #[test]
    fn test_serialize() {
        let msg1 = str_to_vec_u8(MSG1);
        let cursor1 = Cursor::new(msg1.clone());
        let envelope1 = NetworkEnvelope::parse(cursor1, false).unwrap();

        assert_eq!(envelope1.serialize(), msg1);
        //--------------------

        let msg2 = str_to_vec_u8(MSG2);
        let cursor2 = Cursor::new(msg2.clone());
        let envelope = NetworkEnvelope::parse(cursor2, false).unwrap();

        assert_eq!(envelope.serialize(), msg2);   
    }
}

//------------------------------------------------------------------------------
//                                 Message trait
//------------------------------------------------------------------------------

pub trait Message {
    fn command(&self) -> Vec<u8>;
    fn serialize(&self) -> Vec<u8>;
    fn parse<R: Read>(reader: &mut R) -> Self where Self: Sized;
}
//------------------------------------------------------------------------------
//                                 VersionMessage
//------------------------------------------------------------------------------

/// payload 구성
/// 
/// 7f1101000000000000000000ad17835b00000000000000000000000000000000000000000000ffff00000000\
/// 208d000000000000000000000000000000000000ffff00000000208df6a8d7a440ec27a11b2f70726f677261\
/// 6d6d696e67626c6f636b636861696e3a302e312f0000000001
/// 
/// 1. 7f110100 (4bytes, little-endian): Protocol version (70015)
///                 protocol 에 따라서 통신 가능한 message 가 제한됨. 
/// 2. 0000000000000000 (8 bytes, little-endian): Network services 
/// 3. ad17835b00000000 (8 bytes, little-endian): timestamp
/// 4. 0000000000000000 (8 bytes, little-endian): Network services of receiver
/// 5. 00000000000000000000ffff00000000 (16byets): Network address of receiver
///         (앞의 12 bytes 는 IPv6 를 위한 값으로 실제로 사용  X) & IPv4 0.0.0.0
/// 6. 208d (2 bytes, big-endian) : Network port of receiver (8333)
/// 7. 0000000000000000 (8 bytes, little-endian): Network services of sender
/// 8. 00000000000000000000ffff00000000 (16byets): Network address of sender
///         (앞의 12 bytes 는 IPv6 를 위한 값으로 실제로 사용  X) & IPv4 0.0.0.0
/// 9. 208d (2 bytes, big-endian) : Network port of sender (8333)
/// 
/// 10.f6a8d7a440ec27a1 (8 bytes) : Nonce, 노드간 연결되었을때, 자신과의 연결 식별에 사용
/// 11.1b2f70726f6772616d6d696e67626c6f636b636861696e3a302e312f : User Agent
///                                                    실행중인 소프트웨어에 관한 정보
/// 12.00000000 (4 bytes) : Height 해당 message 를 보내는 노드가 가장 최근에 동기화한 block 을 알려줌
/// 13. 01 : Optional flg for relay based on BIP37 Bloom fliter 와 관련 field 
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VersionMessage {
    version: u32,
    services: u64,
    timestamp: u64,
    receiver_services: u64,
    receiver_ip: [u8; 4],
    receiver_port: u16,
    sender_services: u64,
    sender_ip: [u8; 4],
    sender_port: u16,
    nonce: [u8; 8],
    user_agent: Vec<u8>,
    latest_block: u32,
    relay: bool,
}

impl VersionMessage {
    // pub const COMMAND: &'static str = "version";

    pub fn new(
        version: u32, services: u64, timestamp: Option<u64>,
        receiver_services: u64, receiver_ip: [u8; 4],
        receiver_port: u16, 
        sender_services: u64, sender_ip: [u8; 4],
        sender_port: u16,
        nonce: Option<[u8; 8]>, user_agent: Vec<u8>,
        latest_block: u32, relay: bool,
    ) -> Self {
        #[allow(unused_assignments)]
        let mut new_timestamp = 0u64;
        match timestamp {
            Some(ts) => new_timestamp = ts,
            None => {
                let start = SystemTime::now();
                let since_the_epoch = start
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards");

                new_timestamp = since_the_epoch.as_secs();
            }
        }
        #[allow(unused_assignments)]
        let mut new_nonce = [0u8; 8];
        match nonce {
            Some(non) => new_nonce = non,
            None => {
                let mut rng = thread_rng();
                let new_u64 = rng.gen_range(0..u64::MAX);
                new_nonce = new_u64.to_le_bytes();
            }
        }

        Self {
            version,
            services,
            timestamp: new_timestamp,
            receiver_services,
            receiver_ip,
            receiver_port,
            sender_services,
            sender_ip,
            sender_port,
            nonce: new_nonce,
            user_agent,
            latest_block,
            relay
        }
    }
}

impl Default for VersionMessage {
    fn default() -> Self {
        Self {
            version:70015,
            services: 0,
            timestamp: 0,
            receiver_services: 0,
            receiver_ip: [0, 0, 0, 0],
            receiver_port: 8333,
            sender_services: 0,
            sender_ip: [0, 0, 0, 0],
            sender_port: 8333,
            nonce: [0, 0, 0, 0, 0, 0, 0, 0],
            user_agent: vec![0x18, b'/', b'p', b'r', b'o', b'g', b'r', b'a', b'm', b'm', b'i', b'n', b'g', b'b', b'i', b't', b'c', b'o', b'i', b'n', b':', b'0', b'.', b'1', b'/'],
            latest_block: 0,
            relay: false,
        }
    }
}

impl Message for VersionMessage {
    fn command(&self) -> Vec<u8> {
        vec![b'v', b'e', b'r', b's', b'i', b'o', b'n']
    }

    /// Serialize this message to send over the network
    fn serialize(&self) -> Vec<u8> {
        let mut unused_ipv6 = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255];

        let mut result = Vec::<u8>::new();

        let mut version = self.version.to_le_bytes().to_vec();
        result.append(&mut version);

        let mut services = self.services.to_le_bytes().to_vec();
        result.append(&mut services);

        let mut timestamp = self.timestamp.to_le_bytes().to_vec();
        result.append(&mut timestamp);

        let mut receiver_services = self.receiver_services.to_le_bytes().to_vec();
        result.append(&mut receiver_services);

        result.append(&mut unused_ipv6.clone());
        result.append(&mut self.receiver_ip.clone().to_vec());

        let mut receiver_port = self.receiver_port.to_be_bytes().to_vec();
        result.append(&mut receiver_port);

        let mut sender_services = self.sender_services.to_le_bytes().to_vec();
        result.append(&mut sender_services);

        result.append(&mut unused_ipv6);
        result.append(&mut self.sender_ip.to_vec());

        let mut sender_port = self.sender_port.to_be_bytes().to_vec();
        result.append(&mut sender_port);

        result.append(&mut self.nonce.clone().to_vec());

        result.append(&mut self.user_agent.clone().to_vec());

        let mut latest_block = self.latest_block.to_le_bytes().to_vec();
        result.append(&mut latest_block);

        if self.relay {
            result.push(1);
        } else {
            result.push(0);
        }

        result
    }

    #[allow(unused_variables)]
    fn parse<R: Read>(reader: &mut R) -> Self {
        Self::default()
    }
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod version_message_test {
    use super::*;

    #[test]
    fn test_serialize() {
        let v = VersionMessage::default();
        let s = v.serialize();

        let msg = "7f11010000000000000000000000000000000000000000000000000000000000000000000000ffff00000000208d000000000000000000000000000000000000ffff00000000208d0000000000000000182f70726f6772616d6d696e67626974636f696e3a302e312f0000000000";

        let m = str_to_vec_u8(msg);

        // println!("{:?}", s);
        // println!("{:?}", m);
        
        assert_eq!(s, m);
    }
}

//------------------------------------------------------------------------------
//                                 VerAckMessage
//------------------------------------------------------------------------------

#[derive(Clone)]
pub struct VerAckMessage ();

impl VerAckMessage {
    pub fn new() -> Self  {
        Self()
    } 
}

impl Message for VerAckMessage {
    fn command(&self) -> Vec<u8> {
        vec![b'v', b'e', b'r', b'a', b'c', b'k']
    }

    fn serialize(&self) -> Vec<u8> {
        Vec::<u8>::new()
    }

    #[allow(unused_variables)]
    fn parse<R: Read>(reader: &mut R) -> Self {
        Self()
    }
}

//------------------------------------------------------------------------------
//                                PingMessage
//------------------------------------------------------------------------------

#[derive(Clone)]
pub struct PingMessage {
    nonce: [u8; 8]
}

impl PingMessage {
    pub fn new(nonce: [u8; 8]) -> Self {
        Self { nonce }
    }
}

impl Message for PingMessage {
    fn command(&self) -> Vec<u8> {
        vec![b'p', b'i', b'n', b'g']
    }

    fn serialize(&self) -> Vec<u8> {
        self.nonce.to_vec()
    }
    
    fn parse<R: Read>(reader: &mut R) -> Self {
        let mut nonce = [0u8; 8];
        let _ = reader.read_exact(&mut nonce);

        Self { nonce }
    } 
}

//------------------------------------------------------------------------------
//                                PongMessage
//------------------------------------------------------------------------------

#[derive(Clone)]
pub struct PongMessage {
    nonce: [u8; 8]
}

impl PongMessage {
    pub fn new(nonce: [u8; 8]) -> Self {
        Self { nonce }
    }
}

impl Message for PongMessage {
    fn command(&self) -> Vec<u8> {
        vec![b'p', b'o', b'n', b'g']
    }

    fn serialize(&self) -> Vec<u8> {
        self.nonce.to_vec()
    }

    fn parse<R: Read>(reader: &mut R) -> Self {
        let mut nonce = [0u8; 8];
        let _ = reader.read_exact(&mut nonce);

        Self { nonce }
    }
}

//------------------------------------------------------------------------------
//                              GetHeaderMessage
//------------------------------------------------------------------------------

/// 처음 네트워크에 연걸 후 Block Header download 
/// Block Header size 는 전체 Blockchain 의 0.023% 로 빠르게 다운로드 가능
/// Block Header 를 먼저 받고 나머지 full node 는 이후 병렬 처리로 다운로드하여 효율적으로 처리 가능
/// Light Node 의 경우 Block Header 만으로 작업 증명 가능
/// 
/// * getheader message payload 예시
/// 7f11010001ad3...000000..000
/// 
/// 7f110100 (4 bytes, little-endian) : Protocol version (70015) 
/// 01 : Number of hashes, varint
/// a34b...000 (32 bytes, little-endian) : Starting block
/// 0000...000 (32 bytes, little-endian) : End Block
#[derive(Clone)]
pub struct GetHeaderMessage {
    version: u32,
    num_hashes: u32,
    start_block: [u8; 32],
    end_block: [u8; 32],
}

impl GetHeaderMessage {
    pub fn new(version: u32, num_hashes: u32, start_block: [u8; 32], end_block: [u8; 32]) -> Self {
        Self {
            version, 
            num_hashes,
            start_block,
            end_block 
        }   
    }

    pub fn default(start_block: [u8; 32]) -> Self {
        Self {
            version: 70015, 
            num_hashes: 1,
            start_block,
            end_block: [0u8; 32],
        }
    }
}


impl Message for GetHeaderMessage {
    fn command(&self) -> Vec<u8> {
        vec![b'g', b'e', b't', b'h', b'e', b'a', b'd', b'e', b'r', b's']
    }

    /// Serialize this message to send over the network
    fn serialize(&self) -> Vec<u8> {
        let mut result = self.version.to_le_bytes().to_vec();

        let mut num_hashes = encode_varint(self.num_hashes).unwrap();
        result.append(&mut num_hashes);

        let mut start_block = self.start_block.to_vec();
        start_block.reverse();
        result.append(&mut start_block);

        let mut end_block = self.end_block.to_vec();
        end_block.reverse();
        result.append(&mut end_block);

        result
    }

    #[allow(unused_variables)]
    fn parse<R: Read>(reader: &mut R) -> Self where Self: Sized {
        GetHeaderMessage::default([0u8; 32])
    }
}

#[cfg(test)]
mod get_header_message_test {
    use super::*;

    #[test]
    fn test_serialize() {
        let block_hex = "0000000000000000001237f46acddf58578a37e213d2a6edc4884a2fcad05ba3";
        let block_vec = str_to_vec_u8(block_hex);
        let start_block: [u8; 32] = block_vec.try_into().unwrap();

        let gh = GetHeaderMessage::default(start_block);
        assert_eq!(
            str_to_vec_u8("7f11010001a35bd0ca2f4a88c4eda6d213e2378a5758dfcd6af437120000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
            gh.serialize()
        )
    }
}

//------------------------------------------------------------------------------
//                               HeaderMessage
//------------------------------------------------------------------------------

/// header 요청의 응답으로 상대 노드에서 header commad 를 가진 message 를 보냄.
/// 이 응답 message payload 에는 block header list 가 담겨 있으며, 
/// 그 parsing 구조는 다음과 같다.
/// 
/// * Header Command Payload
/// 020000020...157e67000000...de09204600 (1~2000 bytes 의 가변정수(varint))
/// 02 (u8) : Number of Block Headers
/// 00...67 : Block Header
/// 00      : Number of transactions (항상 0)
/// 
/// 위 자료에서 transaction 숫자가 항상 0 인 것은 Header 에는 transation 정보를 담지 않기 때문
/// 그렇다면, 왜 의미 없는 0x00 를 header 마다 붙여주는가?
/// -> Header Message 를 block Message 의 format 과 호환되게 하기 위함 
///    Block Message 의 경우 block header + transaction 수 + transation list 로 구성
///    transaction 수를 0 으로 지정하면 block 전체 (header + transaction) 을 parsing 할 때와
///    동일한 parsing 함수를 사용하는 장점이 있음 (이해 X)
pub struct HeaderMessage {
    blocks: Vec<Block>,
}

impl HeaderMessage {
    fn new() -> Self {
        Self {
            blocks: Vec::<Block>::new(),
        }
    }
}

impl Message for HeaderMessage {
    fn command(&self) -> Vec<u8> {
        vec![ b'h', b'e', b'a', b'd', b'e', b'r', b's']
    }

    fn serialize(&self) -> Vec<u8> {
        Vec::<u8>::new()
    }

    fn parse<R: Read>(reader: &mut R) -> Self where Self: Sized {
        let num_headers = read_varint(reader).unwrap();
        println!("{num_headers}");

        let mut blocks = Vec::<Block>::new();

        for _ in 0..num_headers {
            blocks.push(Block::parse(reader));

            // read next varint
            let num_txs = read_varint(reader).unwrap();
            if num_txs != 0 {
                panic!("number of txs not 0")
            }
        }

        Self { blocks }
    }
}

#[cfg(test)]
mod header_message_test {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test_parse() {
        let hex_msg = "0200000020df3b053dc46f162a9b00c7f0d5124e2676d47bbe7c5d0\
            793a500000000000000ef445fef2ed495c275892206ca533e7411907971013ab83e3b47b\
            d0d692d14d4dc7c835b67d8001ac157e670000000002030eb2540c41025690160a1014c5\
            77061596e32e426b712c7ca00000000000000768b89f07044e6130ead292a3f51951adbd\
            2202df447d98789339937fd006bd44880835b67d8001ade09204600";
        
        let msg_vec = str_to_vec_u8(hex_msg);
        let mut cursor = Cursor::new(msg_vec);
        
        let headers = HeaderMessage::parse(&mut cursor);

        assert_eq!(headers.blocks.len() , 2);
    }
}

//------------------------------------------------------------------------------
//                                 SimpleNode
//------------------------------------------------------------------------------

pub struct SimpleNode {
    testnet: bool,
    logging: bool,
    socket: TcpStream,
    stream: BufReader<TcpStream>,
}

impl SimpleNode {
    pub fn new(host: &str, port: Option<u16>, testnet: bool, logging: bool) 
    -> Result<Self, Box<dyn Error>> {
        let port = match port {
            Some(port) => port,
            None => {
                if testnet {
                    18333
                } else {
                    8333
                }
            },
        };

        let socket = TcpStream::connect((host, port))?;
        let stream = BufReader::new(socket.try_clone()?);

        Ok(Self {
            testnet,
            logging,
            socket,
            stream,
        })
    } 

    /// Do a handshake with the other node.
    /// Handshake is sending a version message and getting a verack back.
    pub fn handshake(&mut self) -> Box<dyn Message>{

        // send a version message
        let version = VersionMessage::default();
        let _ = self.send(Box::new(version));

        // wait fora verack message
        self.wait_for(vec![Box::new(VerAckMessage::new())])
    }

    /// Send a message to the connected node
    pub fn send(&mut self, message: Box<dyn Message>) -> Result<(), std::io::Error> {
        let envelope = NetworkEnvelope::new(
            vec![b'v', b'e', b'r', b's', b'i', b'o', b'n'],
            message.serialize(), 
            self.testnet
        );

        if self.logging {
            info!("Sending: {:?}", &envelope);
        }

        self.socket.write_all(&envelope.serialize())
    }

    /// Read a message from the socket
    pub fn read(&mut self) -> NetworkEnvelope{    
        let envelope = NetworkEnvelope::parse(&mut self.stream, self.testnet).unwrap();

        if self.logging {
            info!("receiving: {:?}", &envelope);
        }

        return envelope
    }

    /// Wait for one of the messages in the list
    pub fn wait_for(&mut self, messages: Vec<Box<dyn Message>>) -> Box<dyn Message> {
        // initialize the command we have, which should be None
        let mut command = Vec::<u8>::new();
        let mut command_to_class = HashMap::<Vec<u8>, Box<dyn Message>>::new();
        for m in messages {
            command_to_class.insert(m.command(), m);
        }

        // loop until the command is in the commands we want
        while !command_to_class.contains_key(&command) {

            // get the next network message
            let envelope = self.read();

            // set the command to be evaluated
            command = envelope.command;

            // we know how to respond to version and ping, handle that here
            if command == vec![b'v', b'e', b'r', b's', b'i', b'o', b'n'] {
                // send verack
                let _ = self.send(Box::new(VerAckMessage::new()));
            } else if command == vec![b'p', b'i', b'n', b'g'] {
                // send pong
                let mut nonce = [0u8; 8];
                if envelope.payload.len() == 8 {
                    nonce.copy_from_slice(&envelope.payload);
                } else {
                    panic!("Length of the vector does not match the length of the array");
                }
                let _ = self.send(Box::new(PongMessage::new(nonce)));
            }
        }
        // return the envelope parsed as a member of the right message class
        let message_class = command_to_class.remove(&command).unwrap();
        
        return message_class
    }
}

#[cfg(test)]
mod simple_node_test {
    use crate::models::block::{GENESIS_BLOCK, GENESIS_BLOCK_STR, LOWEST_BITS_STR};

    use super::*;

    #[test]
    fn handshake_test() {
        let mut node = SimpleNode::new(
            "testnet.programmingbitcoin.com",
            None,
            true,
            true,
        ).unwrap();
    
        let version = VersionMessage::default();
        
        let _ = node.send(Box::new(version));
        
        let mut verack_received = false;
        let mut version_received = false;
    
        while !verack_received || !version_received {
            let message = node.wait_for(
                vec![Box::new(VersionMessage::default()), Box::new(VerAckMessage::new())]
            );
    
            if message.command() == vec![b'v', b'e', b'r', b'a', b'c', b'k'] {
                verack_received = true;
            } else {
                version_received = true;
            }
        }
        node.handshake();
    }

    // TO DO
    #[test]
    fn download_test() {
        use std::io::{Cursor, Read};

        let genesis_vec = str_to_vec_u8(GENESIS_BLOCK_STR);
        let mut genesis_cursor = Cursor::new(genesis_vec);
        let mut previous = Block::parse(&mut genesis_cursor);

        let first_epoch_timestamp = previous.get_timestamp();
        let expected_bits = str_to_vec_u8(LOWEST_BITS_STR);
        let mut count = 1;

        let mut node = SimpleNode::new(
            "mainnet.programmingbitcoin.com",
            None,
            false,
            true,
        ).unwrap();

        node.handshake();

        let previous_hash = previous.hash();

        for _ in 0..19 {
            let get_headers = GetHeaderMessage::default(
                previous_hash.clone().try_into().unwrap()
            );
            node.send(Box::new(get_headers));

            let headers = node.wait_for(
                vec![Box::new(HeaderMessage::new())]
            );

            // for header in headers.
        }
    }
}
