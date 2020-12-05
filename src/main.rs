mod server;
mod client;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("usage: {} <server/client>", &args[0]);
    }

    let mode = &args[1];

    if mode.eq("server") {
        server::main();
    } else if mode.eq("client") {
        client::main();
    } else {
        panic!("usage: {} <server/client>", &args[0]);
    }
}
