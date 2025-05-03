use serde::Deserialize;

use crate::{GetResponse, Request, Result};
use std::io::{BufReader, BufWriter, Write};
use std::net::TcpStream;

pub struct KvsClient {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl KvsClient {
    pub fn connect(addr: &str) -> Result<KvsClient> {
        let stream = TcpStream::connect(addr).expect("Couldn't connect to the server");

        Ok(KvsClient {
            reader: BufReader::new(stream.try_clone()?),
            writer: BufWriter::new(stream),
        })
    }

    pub fn get(&mut self, key: String) -> Result<GetResponse> {
        let operation = Request::Get { key };
        serde_json::to_writer(&mut self.writer, &operation)?;
        self.writer.flush()?;

        let mut de = serde_json::de::Deserializer::from_reader(&mut self.reader);
        let res = GetResponse::deserialize(&mut de)?;

        Ok(res)
    }
}
