use bincode::serialize;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::net::UdpSocket;
use std::time::Duration;
use anyhow::{anyhow, Error};

fn main() {
    println!("{:?}", String::from_utf8(serialize("").unwrap()));
}

struct Member {
    peers: Vec<String>,
    socket: UdpSocket,
}

impl Member {
    fn new(config: Config) -> anyhow::Result<Self> {
        let mut socket = UdpSocket::bind(config.address)?;
        socket.set_write_timeout(Some(Duration::from_millis(config.timeout)))?;

        
        Ok(Member {
            socket: socket,
            peers: Vec::new(),
        })
    }

    fn heartbeat(&self) {
        let mut rng = rand::thread_rng();
        let chosen = self.peers.choose(&mut rng);
        match chosen {
            Some(peer) => {}
            None => {}
        }
    }

    fn ping(&self, peer: String) -> anyhow::Result<()> {
        let data = serialize(&Message::Ping)?;
        match self.socket.send_to(&data, peer) {
            Err(e) => return Err(Error::new(e)),
            Ok(n) => {
                if n != data.len() {
                    return Err(anyhow!("expected to write {} bytes but wrote {}", data.len(), n));
                }   
            }
        }
        Ok(())
    }
}

struct Config {
    address: String,
    fanout: u32,
    period: u64,
    timeout: u64,
}

#[derive(Deserialize, Serialize)]
enum Message {
    Ping,
    PingReq,
    Ack,
}
