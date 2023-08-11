use path::DynamicPath;
use context::Context;
use route::Route;

use std::collections::HashMap;

/// A Router that used to store a collection of routes
/// and then it can be bind to the main routes in some app
///
/// # Example
///
/// ```rust 
/// let mut router = Router::new();
///
/// router.get("/", ...);
/// router.post("/", ...);
///
/// app.bind("/users", router);
/// ```
///
pub struct Router {
    routes: HashMap<DynamicPath, Route>,
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
            DynamicPath::parse(path),
            Route {
                method: String::from("POST"),
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
            DynamicPath::parse(path),
            Route {
                method: String::from("GET"),
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
            DynamicPath::parse(path),
            Route {
                method: String::from("PUT"),
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
            DynamicPath::parse(path),
            Route {
                method: String::from("PATCH"),
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
            DynamicPath::parse(path),
            Route {
                method: String::from("DELETE"),
                handler: Box::new(handler),
            },
        );
    }

    /// Transforms the router to the routes made in it
    pub fn to_routes(self) -> HashMap<DynamicPath, Route> {
        self.routes
    }
}
