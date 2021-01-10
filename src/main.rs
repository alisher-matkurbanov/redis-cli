use async_std::net::TcpStream;
use async_std::io;
use async_std::prelude::*;


#[async_std::main]
async fn main() -> io::Result<()> {
    // creating TCP connection to redis server
    let mut stream = TcpStream::connect("localhost:6379").await?;
    let mut buffer = vec![];
    let command = RespValue::Array(vec![RespValue::BulkString(b"PING".to_vec())]);
    command.serialize(&mut buffer);
    // sending command to redis server
    stream.write_all(&buffer).await?;
    let mut buffer = vec![0; 1024];
    // read from TCP connection. Expect PONG string
    let bytes_read = stream.read(&mut buffer).await?;
    // println!("{:?}", std::str::from_utf8(&buffer[..bytes_read]).unwrap());
    println!("{:?}", parse_response(&buffer[0..bytes_read]));
    Ok(())
}

fn parse_response(buffer: &[u8]) -> Result<&str, String> {
    if buffer.is_empty() {
        return Err("Empty buffer".into());
    }
    // parsing response according to RESP syntax
    if buffer[0] == ('-' as u8) {
        return Err(format!(
            "Error Response: {:?}",
            &buffer[1..buffer.len() - 2]));
    }

    Ok(std::str::from_utf8(&buffer[1..buffer.len() - 2]).unwrap())
}

enum RespValue {
    Array(Vec<RespValue>),
    BulkString(Vec<u8>),
    SimpleString(String),
    Error(Vec<u8>),
    Integers(i64),
}

impl RespValue {
    fn serialize(self, buf: &mut Vec<u8>) {
        match self {
            RespValue::Array(values) => {
                buf.push('*' as u8);
                buf.append(&mut format!("{}", values.len()).into_bytes());
                buf.push('\r' as u8);
                buf.push('\n' as u8);
                for value in values {
                    value.serialize(buf)
                }
            }
            RespValue::BulkString(mut data) => {
                buf.push('$' as u8);
                buf.append(&mut format!("{}", data.len()).into_bytes());
                buf.push('\r' as u8);
                buf.push('\n' as u8);
                buf.append(&mut data);
                buf.push('\r' as u8);
                buf.push('\n' as u8);
            }
            _ => unimplemented!(),
        }
    }
}