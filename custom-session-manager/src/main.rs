use racoon::core::path::Path;
use racoon::core::request::Request;
use racoon::core::response::status::ResponseStatus;
use racoon::core::response::{HttpResponse, Response};

use racoon::core::server::Server;
use racoon::core::session::{AbstractSessionManager, SessionResult};
use racoon::view;

async fn home(request: Request) -> Response {
    let session = request.session;
    let name = session.get("Hello World").await;
    // Reads session value
    println!("name: {:?}", name);

    let _ = session.set("location", "Ktm").await;

    // Removes session value
    let _ = session.remove("name").await;

    // Destory session
    let _ = session.destroy().await;

    HttpResponse::ok().body("Hello World")
}

struct RedisSessionManager;

impl AbstractSessionManager for RedisSessionManager {
    fn set(
        &self,
        session_id: &String,
        name: &str,
        value: &str,
    ) -> SessionResult<std::io::Result<()>> {
        todo!()
    }

    fn get(&self, session_id: &String, name: &str) -> SessionResult<Option<String>> {
        todo!()
    }

    fn remove(&self, session_id: &String, name: &str) -> SessionResult<std::io::Result<()>> {
        todo!()
    }

    fn destroy(&self, session_id: &String) -> SessionResult<std::io::Result<()>> {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    let paths = vec![Path::new("/", view!(home))];

    let custom_session_manager = RedisSessionManager {};
    let _ = Server::bind("127.0.0.1:8080")
        .urls(paths)
        .set_session_manager(custom_session_manager)
        .run()
        .await
        .unwrap();
}
