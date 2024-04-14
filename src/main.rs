use std::net;
use pasternakd::handle_client;

fn main() {
  let listener = net::TcpListener::bind("0.0.0.0:8492")
    .expect("Failed to listen on port 8492");

  println!("Listening on port 8492");

  for stream in listener.incoming() {
    println!("Handling Client");
    handle_client();
  }

  main();
}
