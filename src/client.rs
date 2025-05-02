use crate::{Operation, Result};
use std::io::{Read, Write};
use std::net::TcpStream;

pub struct KvsClient {
    stream: TcpStream,
}

impl KvsClient {
    pub fn connect(addr: &str) -> Result<KvsClient> {
        let stream = TcpStream::connect(addr).expect("Couldn't connect to the server");

        Ok(KvsClient { stream })
    }

    pub fn get(&mut self, key: String) -> Result<String> {
        let operation = Operation::Get { key };
        let msg = serde_json::to_vec(&operation)?;
        self.stream.write(&msg)?;

        let mut res = [0; 1024];
        self.stream.read(&mut res).expect("Failed reading response");
        let val = String::from_utf8_lossy(&res);

        Ok(val.to_string())
    }
}
