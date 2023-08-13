use context::Context;
use path::DynamicPath;
use route::Route;
use router::Router;

use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

use pond::Pool;

/// App structure for making a new web application
pub struct App {
    routes: Vec<Route>,
}

impl App {
    /// Builds an app instance
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut app = App::build();
    /// ```
    ///
    pub fn build() -> App {
        App { routes: Vec::new() }
    }

    /// Pushes to routes with a method "POST"
    pub fn post<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Context) -> String + Send + Sync + 'static,
    {
        self.routes.push(Route {
            path: DynamicPath::parse(path),
            method: String::from("POST"),
            handler: Box::new(handler),
        });
    }

    /// Pushes to routes with a method "GET"
    pub fn get<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Context) -> String + Send + Sync + 'static,
    {
        self.routes.push(Route {
            path: DynamicPath::parse(path),
            method: String::from("GET"),
            handler: Box::new(handler),
        });
    }

    /// Pushes to routes with a method "PUT"
    pub fn put<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Context) -> String + Send + Sync + 'static,
    {
        self.routes.push(Route {
            path: DynamicPath::parse(path),
            method: String::from("PUT"),
            handler: Box::new(handler),
        });
    }

    /// Pushes to routes with a method "PATCH"
    pub fn patch<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Context) -> String + Send + Sync + 'static,
    {
        self.routes.push(Route {
            path: DynamicPath::parse(path),
            method: String::from("PATCH"),
            handler: Box::new(handler),
        });
    }

    /// Pushes to routes with a method "DELETE"
    pub fn delete<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Context) -> String + Send + Sync + 'static,
    {
        self.routes.push(Route {
            path: DynamicPath::parse(path),
            method: String::from("DELETE"),
            handler: Box::new(handler),
        });
    }

    /// Binds specific router and pushes its routes to the main routes
    ///
    /// # Example
    ///
    /// ```rust
    /// app.bind("/users", router);
    /// ```
    ///
    pub fn bind(&mut self, path: &str, router: Router) {
        for mut route in router.to_routes() {
            route.path = DynamicPath::parse(&format!("{}{}", path, route.path.to_string()));

            self.routes.push(route)
        }
    }

    /// Starts the listening to requests on a specifiec port
    ///
    /// # Example
    ///
    /// ```rust
    /// app.listen(8080); // Starts to listen to "127.0.0.1:8080"
    /// ````
    ///
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
        let request_line = request_line.split(' ').collect::<Vec<&str>>();

        // "GET" /
        let method = request_line.first().unwrap_or(&"").to_string();

        // GET "/"
        let path = request_line.get(1).unwrap_or(&"");

        let mut responed = false;

        for route in self
            .routes
            .iter()
            .filter(|r| r.path == DynamicPath::parse(path) && r.method == method)
        {
            let response = (route.handler)(Context::new(&mut stream));
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                response.len(),
                response
            );

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();

            responed = true;
        }

        if !responed {
            let response = format!("Cannot {} {}", method, path);
            let response = format!(
                "HTTP/1.1 404\r\nContent-Length: {}\r\n\r\n{}",
                response.len(),
                response
            );

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
