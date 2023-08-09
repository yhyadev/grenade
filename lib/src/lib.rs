//! # Grenade Documentation
//!
//! Welcome to the documentation of Grenade!
//!
//! # Usage
//!
//! Start by adding the Grenade library to your project's `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! grenade = "0.1"
//! ```
//!
//! And here is an example of hello world application in Grenade:
//!
//! ```rust,no_run
//! use grenade::*;
//!
//! fn hello_world(_: Context) -> String {
//!     "Hello World!".to_string()
//! }
//!
//! fn main() {
//!     let mut app = App::build();
//!
//!     app.get("/", hello_world);
//!
//!     app.listen(8080).unwrap();
//! }
//! ```

mod app;
pub use app::*;

mod context;
pub use context::*;

mod route;

mod router;
pub use router::*;

extern crate pond;
