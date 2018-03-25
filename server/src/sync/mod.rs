pub mod peer;
pub mod state;

use lib::sync::message::{Client, Server};
use lib::sync::codec::Lines;

use std::sync::mpsc::{Receiver, Sender};
use std::net::SocketAddr;

use lib::futures::sync::mpsc::{UnboundedReceiver, UnboundedSender};

/// Shorthand for the transmit half of the message channel
pub type Tx = UnboundedSender<Server>;

/// Shorthand for the receive half of the message channel
pub type Rx = UnboundedReceiver<Server>;

pub type CTx = Sender<(Client, SocketAddr)>;
pub type CRx = Receiver<(Client, SocketAddr)>;

pub type Codec = Lines<Server, Client>;
