use context::Context;
use route::Route;

use std::collections::HashMap;

pub struct Router {
    routes: HashMap<String, Route>,
}

impl Router {
    /// Creates a new Router
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut router = Router::new();
    /// ```
    ///
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    /// Pushes to routes with a method "POST"
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

    /// Pushes to routes with a method "GET"
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

    /// Pushes to routes with a method "PUT"
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

    /// Pushes to routes with a method "PATCH"
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

    /// Pushes to routes with a method "DELETE"
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

    /// Transforms the router to the routes made in it
    pub fn to_routes(self) -> HashMap<String, Route> {
        self.routes
    }
}