use grenade::*;

fn hello_world(_: Context) -> String {
    "Hello World! from the api".to_string()
}

fn main() {
    let mut app = App::build();

    let mut router = Router::new();

    router.get("/", hello_world);

    app.bind("/api", router);

    app.listen(8080).unwrap();
}
