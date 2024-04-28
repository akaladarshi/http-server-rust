use std::fmt::Error;
use std::io::Write;

use crate::header::{CRLF, PROTOCOL_VERSION};
use crate::header::status::Status;

pub struct ResponseHeader {
    protocol_version: String,
    status: Status,
    content_type: String,
    content: Vec<u8>,
}

impl ResponseHeader {
    pub fn new(code: isize, content_type: &str, content: Vec<u8>) -> ResponseHeader {
        ResponseHeader{
            protocol_version: PROTOCOL_VERSION.to_string(),
            status:  Status::get_status(code).unwrap(),
            content_type: content_type.to_string(),
            content,
        }
    }
    pub fn serialize<W: Write>(&self, writer:  &mut W) -> Result<usize, Error> {
        // write status line with end of status CRLF
        let n = writer.write(format!("{} {}{}", self.protocol_version, self.status.string(), CRLF).as_bytes()).map_err(|_| Error)?;
        if self.content.is_empty() {
            return Ok(n)
        }

        writer.write(format!("{}: {}{}", "Content-Type", self.content_type, CRLF).as_bytes()).map_err(|_| Error)?;
        writer.write(format!("{}: {}{}", "Content-Length", self.content.len(), CRLF).as_bytes()).map_err(|_| Error)
    }

}