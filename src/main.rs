use std::net;
use pasternakd::handle_client;

fn main() {
  let listener = net::TcpListener::bind("127.0.0.1:8492")
    .expect("Failed to listen on port 8492");

  for stream in listener.incoming() {
    handle_client();
  }

  main();
}