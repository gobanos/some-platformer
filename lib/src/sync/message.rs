use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Client {
    Test,             // An empty message, to test protocoles
    Ping(SystemTime), // Current time, to synchronize client and server
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Server {
    Test,
    Pong {
        client: SystemTime,
        server: SystemTime,
    },
}
