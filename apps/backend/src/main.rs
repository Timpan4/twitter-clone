#![deny(clippy::all, clippy::pedantic, clippy::style)]
use poem::{get, handler, listener::TcpListener, web::Path, IntoResponse, Route, Server};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}\n", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/hello/:name", get(hello));
    let result = Server::new(TcpListener::bind("0.0.0.0:3000")).run(app);
    println!("Listening on port 3000");
    result.await
}
