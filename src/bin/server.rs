extern crate some_platformer;
extern crate tokio;
#[macro_use]
extern crate futures;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener
        .incoming()
        .for_each(|socket| {
            println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

            let connection = io::write_all(socket, "hello from server\n")
                .then(|res| {
                    println!("wrote message; success={:?}", res.is_ok());
                    let (socket, _) = res.unwrap();
                    Ok(socket)
                }).map(|_| ());

            tokio::spawn(connection);

            Ok(())
        })
        .map_err(|err| {
            println!("accept error = {:?}", err);
        });

    println!("server running on localhost:3000");
    tokio::run(server);
}
