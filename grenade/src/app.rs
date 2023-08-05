<<<<<<< HEAD
use std::net::TcpListener;

pub struct App {}

impl App {
    pub fn build() -> App {
        App {}
=======
use context::Context;
use route::Route;

use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

pub struct App {
    routes: Vec<Route>,
}

impl App {
    pub fn build() -> App {
        App { routes: Vec::new() }
    }

    pub fn post<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Context) -> String + Send + 'static,
    {
        self.routes.push(Route {
            method: String::from("POST"),
            path: path.to_string(),
            handler: Box::new(handler),
        })
    }

    pub fn get<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Context) -> String + Send + 'static,
    {
        self.routes.push(Route {
            method: String::from("GET"),
            path: path.to_string(),
            handler: Box::new(handler),
        })
    }


    pub fn put<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Context) -> String + Send + 'static,
    {
        self.routes.push(Route {
            method: String::from("PUT"),
            path: path.to_string(),
            handler: Box::new(handler),
        })
    }
    
    pub fn patch<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Context) -> String + Send + 'static,
    {
        self.routes.push(Route {
            method: String::from("PATCH"),
            path: path.to_string(),
            handler: Box::new(handler),
        })
    }
    
    pub fn delete<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Context) -> String + Send + 'static,
    {
        self.routes.push(Route {
            method: String::from("DELETE"),
            path: path.to_string(),
            handler: Box::new(handler),
        })
>>>>>>> 7f1e43f (feat: make single threaded routes handling)
    }

    pub fn listen(&self, port: u16) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

<<<<<<< HEAD
        for stream in listener.incoming() {}

        Ok(())
    }
=======
        for stream in listener.incoming() {
            self.find_handler(stream.unwrap())
        }

        Ok(())
    }

    fn find_handler(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);

        let request_line = buf_reader.lines().next().unwrap().unwrap();
        let request_line = request_line.split(" ").collect::<Vec<&str>>();

        let method = request_line.get(0).unwrap();
        let path = request_line.get(1).unwrap();

        if let Some(route) = self
            .routes
            .iter()
            .find(|r| r.method.as_str() == *method && r.path.as_str() == *path)
        {
            let response = (route.handler)(Context {});
            let response = format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                "HTTP/1.1 200 OK",
                response.len(),
                response
            );

            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
>>>>>>> 7f1e43f (feat: make single threaded routes handling)
}
