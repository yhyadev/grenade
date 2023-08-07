use std::io::prelude::*;
use std::net::TcpStream;

pub struct Context<'a> {
    stream: &'a mut TcpStream,
}

impl<'a> Context<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Context<'a> {
        Context { stream }
    }

    pub fn send_status(&mut self, status_code: u16) {
        self.stream
            .write(format!("HTTP/1.1 {}", status_code).as_bytes())
            .unwrap();
    }
}
