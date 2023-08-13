use path::DynamicPath;
use context::Context;

pub struct Route {
    pub path: DynamicPath,
    pub method: String,
    pub handler: Box<dyn Fn(Context) -> String + Send + Sync + 'static>
}
