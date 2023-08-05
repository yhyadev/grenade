use std::net::TcpListener;

pub struct App {}

impl App {
    pub fn build() -> App {
        App {}
    }

    pub fn listen(&self, port: u16) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

        for stream in listener.incoming() {}

        Ok(())
    }
}
