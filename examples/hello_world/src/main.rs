use grenade::*;

fn hello_world(_: Context) -> String {
    "Hello World!".to_string()
}

fn main() {
    let mut app = App::build();

    app.get("/", hello_world);

    app.listen(8080).unwrap();
}
