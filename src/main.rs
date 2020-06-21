use bincode::serialize;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::time::Duration;

fn main() {
    println!("{:?}", String::from_utf8(serialize("").unwrap()));
}

struct Member {
    peers: Vec<String>,
}

impl Member {
    fn heartbeat(&self) {
        let mut rng = rand::thread_rng();
        let chosen = self.peers.choose(&mut rng);
    }
}

struct Config {
    fanout: u32,
    period: Duration,
    timeout: Duration,
}

#[derive(Deserialize, Serialize)]
enum Message {
    Ping,
    PingReq,
    Ack,
}
