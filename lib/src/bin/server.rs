extern crate some_platformer_lib;
extern crate tokio;
extern crate futures;
extern crate flexi_logger;
#[macro_use]
extern crate log;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

use flexi_logger::Logger;

fn main() {
    Logger::with_env_or_str("some_platformer=warn")
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    let addr = "127.0.0.1:3000".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener
        .incoming()
        .for_each(|socket| {
            debug!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

            let connection = io::write_all(socket, "hello from server\n")
                .then(|res| {
                    debug!("wrote message; success={:?}", res.is_ok());
                    let (socket, _) = res.unwrap();
                    Ok(socket)
                }).map(|_| ());

            tokio::spawn(connection);

            Ok(())
        })
        .map_err(|err| {
            debug!("accept error = {:?}", err);
        });

    info!("server running on localhost:3000");
    tokio::run(server);
}
