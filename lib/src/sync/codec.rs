use std::net::SocketAddr;

use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

use serde_json;

use bytes::{BufMut, BytesMut};

use std::marker::PhantomData;

use serde::Serialize;
use serde::de::DeserializeOwned;

/// The codec allowing framed communication
pub struct Lines<S: Serialize, D: DeserializeOwned> {
    socket: TcpStream,
    rd: BytesMut,
    wr: BytesMut,
    serializer: PhantomData<S>,
    deserializer: PhantomData<D>,
}

impl<S: Serialize, D: DeserializeOwned> Lines<S, D> {
    /// Create a new `Lines` codec backed by the socket
    pub fn new(socket: TcpStream) -> Self {
        Lines {
            socket,
            rd: BytesMut::new(),
            wr: BytesMut::new(),
            serializer: PhantomData,
            deserializer: PhantomData,
        }
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.socket.peer_addr()
    }

    pub fn buffer(&mut self, data: &S) -> Result<(), serde_json::Error> {
        let data = serde_json::to_vec(data)?;
        // Push the line onto the end of the write buffer?
        //
        // The `put` function if from the `BufMut` trait.
        self.wr.put(data);
        self.wr.put("\r\n");

        Ok(())
    }

    pub fn poll_flush(&mut self) -> Poll<(), io::Error> {
        // As long as there is buffered data to write, try to write it.
        while !self.wr.is_empty() {
            // Try to write some bytes from the socket
            let n = try_ready!(self.socket.poll_write(&self.wr));

            // As long as the wr is not empty, a successful write should
            // never write 0 bytes.
            assert!(n > 0);

            // This discards the first `n` bytes of the buffer?
            let _ = self.wr.split_to(n);
        }

        Ok(Async::Ready(()))
    }

    fn fill_read_buf(&mut self) -> Poll<(), io::Error> {
        loop {
            // Ensure the read buffer has capacity
            //
            // This might result in an internal allocation.
            self.rd.reserve(1024);

            // Read data into the buffer.
            //
            // The `read_buf` fn is provided by `AsyncRead`?
            let n = try_ready!(self.socket.read_buf(&mut self.rd));

            if n == 0 {
                return Ok(Async::Ready(()));
            }
        }
    }
}

impl<S: Serialize, D: DeserializeOwned> Stream for Lines<S, D> {
    type Item = D;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        // First, read any new data that might have been received
        // off the socket
        //
        // We track if the socket is closed here and will be used
        // to inform the return value below.
        let sock_closed = self.fill_read_buf()?.is_ready();

        // Now, try finding lines
        let pos = self.rd
            .windows(2)
            .enumerate()
            .find(|&(_, bytes)| bytes == b"\r\n")
            .map(|(i, _)| i);

        if let Some(pos) = pos {
            // Remove the line form the read buffer and set it
            // to `line`.
            let mut line = self.rd.split_to(pos + 2);

            // Drop the trailing \r\n
            line.split_off(pos);

            let data: D = serde_json::from_slice(&line)
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, Box::new(err)))?;

            // Return the line
            return Ok(Async::Ready(Some(data)));
        }

        if sock_closed {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}
