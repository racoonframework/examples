use std::time::Duration;

use racoon::core::path::Path;
use racoon::core::request::Request;
use racoon::core::response::status::ResponseStatus;
use racoon::core::response::{HttpResponse, Response};

use racoon::core::server::Server;
use racoon::core::shortcuts::SingleText;
use racoon::view;

async fn home(request: Request) -> Response {
    let cookies = request.cookies;
    println!("{:?}", cookies.value("sessionid"));

    let mut response = HttpResponse::ok().body("Hello World");
    response.set_cookie("sessionid", "123", Duration::from_secs(86400));

    // To remove cookie uncomment
    // response.remove_cookie("sessionid");
    response
}

#[tokio::main]
async fn main() {
    let paths = vec![Path::new("/", view!(home))];
    let _ = Server::bind("127.0.0.1:8080")
        .urls(paths)
        .run()
        .await
        .unwrap();
}
