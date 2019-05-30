extern crate mio;
use mio::*;

// Implements WebSocketServer as an implementation of the Handler interface
struct WebSocketServer;
impl Handler for WebSocketServer {
    type Timeout = usize;
    type Message = ();
}

fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    let mut handler = WebSocketServer;
    event_loop.run(&mut handler).unwrap();
}
