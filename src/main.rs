extern crate iron;
extern crate router;
extern crate websocket;

use iron::prelude::*;
use iron::status;
use router::Router;
use websocket::{Server, Message};
use std::thread;

fn main() {
    let mut router = Router::new();
    router.get("/", handler);
    router.get("/:query", handler);


    let server = Server::bind("127.0.0.1:3003").unwrap();


    for connection in server {
        // Spawn a new thread for each connection.
        thread::spawn(move || {
           let request = connection.unwrap().read_request().unwrap(); // Get the request
           let response = request.accept(); // Form a response
           let mut client = response.send().unwrap(); // Send the response

           let message = Message::Text("Hello, client!".to_string());
           let _ = client.send_message(message);
        });
    }

    Iron::new(router).http("0.0.0.0:3000").unwrap();
}

fn handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with(*query))
}
