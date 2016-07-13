use std::io::Write;
use std::net::{TcpStream, TcpListener, SocketAddr};
use bincode::SizeLimit;
use bincode::rustc_serialize::encode;
use byteorder::{NetworkEndian, WriteBytesExt};
use super::DebugInformation;

/// The `Server` type.
pub struct Server {
    /// The connected client.
    pub client: Option<(TcpStream, SocketAddr)>,
}

/// The `Server` implementation.
impl Server {
    /// Constructs a new `Server`.
    pub fn new() -> Server {
        Server { client: None }
    }

    /// Binds the socket and accepts a connection.
    pub fn bind_and_accept(&mut self) -> bool {
        let sock = TcpListener::bind(("127.0.0.1", super::DEBUGGER_PORT)).unwrap();
        self.client = sock.accept().ok();
        self.client.is_some()
    }

    /// Updates the debug information.
    ///
    /// In other words, sends the most recent
    /// debugging information to the connected client.
    pub fn update(&mut self, data: DebugInformation) -> bool {
        // Get the client tcp stream
        let tpl = self.client.as_mut().unwrap();
        let mut sock: &TcpStream = &tpl.0;

        // Encode and send the debug information
        let mut length_prefix = vec![];
        let mut encoded_data: Vec<u8> = encode(&data, SizeLimit::Infinite).unwrap();
        length_prefix.write_u32::<NetworkEndian>(encoded_data.len() as u32).unwrap();
        match sock.write(length_prefix.as_mut_slice()).ok() {
            None => return false,
            _ => (),
        }
        match sock.write(encoded_data.as_mut_slice()).ok() {
            None => return false,
            _ => (),
        }
        sock.flush().ok().is_some()
    }
}
