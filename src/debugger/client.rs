use std::io::{Read, Cursor};
use std::net::TcpStream;
use bincode::rustc_serialize::decode;
use byteorder::{NetworkEndian, ReadBytesExt};
use super::DebugInformation;

/// The `Client` type.
pub struct Client {
    stream: Option<TcpStream>,
}

/// The `Client` implementation.
impl Client {
    /// Constructs a new `Client`.
    pub fn new() -> Client {
        Client { stream: None }
    }

    /// Attempts to connect to a debug server.
    pub fn connect(&mut self) -> bool {
        self.stream = TcpStream::connect(("127.0.0.1", super::DEBUGGER_PORT)).ok();
        self.stream.is_some()
    }

    /// Attempts to receive updates from the server.
    pub fn get_update(&mut self) -> Option<DebugInformation> {
        // Return `Option::None` if no connection is active.
        if self.stream.is_none() {
            return None;
        }

        // Get the stream
        let stream = self.stream.as_mut().unwrap();

        // Read the length prefix
        let mut length_prefix_buffer = vec![0u8; 4];
        match stream.read_exact(length_prefix_buffer.as_mut_slice()).ok() {
            None => return None,
            _ => (),
        }
        let mut reader = Cursor::new(&length_prefix_buffer);
        let length_prefix = reader.read_u32::<NetworkEndian>()
            .expect("Unable to read length prefix as u32.");

        // Read the data
        let mut raw_data: Vec<u8> = vec![0u8; length_prefix as usize];
        stream.read_exact(&mut raw_data).expect("Unable to read data.");
        let data: Option<DebugInformation> = decode(&mut raw_data).ok();
        data
    }
}
