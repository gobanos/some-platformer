pub mod peer;
pub mod shared;

use some_platformer_lib::sync::message::{Client, Server};
use some_platformer_lib::sync::codec::Lines;

use futures::sync::mpsc::{UnboundedReceiver, UnboundedSender};

/// Shorthand for the transmit half of the message channel
type Tx = UnboundedSender<Server>;

/// Shorthand for the receive half of the message channel
type Rx = UnboundedReceiver<Server>;

pub type Codec = Lines<Server, Client>;
