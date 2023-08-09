use context::Context;

pub struct Route {
    pub method: String,
    pub handler: Box<dyn Fn(Context) -> String + Send + Sync + 'static>
}
