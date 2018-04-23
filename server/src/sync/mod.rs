pub mod peer;
pub mod state;

use std::sync::mpsc::{Receiver, Sender};
use std::net::SocketAddr;

use lib::sync::message::{Client, Server};
use lib::sync::codec::Lines;

use lib::futures::sync::mpsc::{UnboundedReceiver, UnboundedSender};

// SHORTHANDS
/// game -> client channel
pub type G2CSender = UnboundedSender<Server>;
pub type G2CReceiver = UnboundedReceiver<Server>;

// client -> game channel
pub type C2GSender = Sender<(Client, SocketAddr)>;
pub type C2GReceiver = Receiver<(Client, SocketAddr)>;

// server `Lines` codec
pub type Codec = Lines<Server, Client>;
