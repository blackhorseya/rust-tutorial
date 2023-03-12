use std::net::TcpListener;

const ADDR: &str = "0.0.0.0:8080";
const MSG_SIZE: usize = 32;

fn main() {
    let server = TcpListener::bind(ADDR).expect("Listener failed to bind");

    println!("Hello, world!");
}
