use context::Context;
use route::Route;

use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

use pond::Pool;

pub struct App {
    routes: HashMap<String, Route>,
}

impl App {
    pub fn build() -> App {
        App {
            routes: HashMap::new(),
        }
    }

    pub fn post<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Context) -> String + Send + Sync + 'static,
    {
        self.routes.insert(
            path.to_string(),
            Route {
                method: String::from("POST"),
                path: path.to_string(),
                handler: Box::new(handler),
            },
        );
    }

    pub fn get<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Context) -> String + Send + Sync + 'static,
    {
        self.routes.insert(
            path.to_string(),
            Route {
                method: String::from("GET"),
                path: path.to_string(),
                handler: Box::new(handler),
            },
        );
    }

    pub fn put<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Context) -> String + Send + Sync + 'static,
    {
        self.routes.insert(
            path.to_string(),
            Route {
                method: String::from("PUT"),
                path: path.to_string(),
                handler: Box::new(handler),
            },
        );
    }

    pub fn patch<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Context) -> String + Send + Sync + 'static,
    {
        self.routes.insert(
            path.to_string(),
            Route {
                method: String::from("PATCH"),
                path: path.to_string(),
                handler: Box::new(handler),
            },
        );
    }

    pub fn delete<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Context) -> String + Send + Sync + 'static,
    {
        self.routes.insert(
            path.to_string(),
            Route {
                method: String::from("DELETE"),
                path: path.to_string(),
                handler: Box::new(handler),
            },
        );
    }

    pub fn listen(&self, port: u16) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
        let mut pool = Pool::new();

        for stream in listener.incoming() {
            pool.scoped(|s| {
                s.execute(move || {
                    self.find_handler(stream.unwrap());
                });
            })
        }

        Ok(())
    }

    fn find_handler(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);

        let request_line = match buf_reader.lines().next() {
            Some(Ok(line)) => line,
            _ => "".to_string(),
        };
        let request_line = request_line.split(" ").collect::<Vec<&str>>();

        let method = request_line.get(0).unwrap_or(&"").to_string();
        let path = request_line.get(1).unwrap_or(&"").to_string();

        if let Some(route) = self.routes.get(&path) {
            if route.method == method {
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
    }
}