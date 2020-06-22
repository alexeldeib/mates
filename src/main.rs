use anyhow::{anyhow, Error};
use bincode::{deserialize, serialize};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::net::UdpSocket;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    println!("starting up");

    let a1 = String::from("localhost:8001");
    let a2 = String::from("localhost:8002");

    let c1: Config = Config {
        address: a1.clone(),
        timeout: 1000,
        ..Default::default()
    };

    let c2: Config = Config {
        address: a2.clone(),
        timeout: 1000,
        ..Default::default()
    };

    let m1 = Member::new(c1)?;
    let m2 = Member::new(c2)?;

    match m1.ack(&a2) {
        Err(e) => return Err(e),
        Ok(()) => (),
    }

    let mut buf = [0u8; 1500];
    match m2.socket.recv_from(&mut buf) {
        Err(e) => return Err(Error::new(e)),
        Ok((amt, src)) => {
            println!("amt: {:?}, src: {:?}", amt, src);
            let msg: Message = deserialize(&buf[..amt]).unwrap();
            println!("{:?}", msg);
        }
    }

    Ok(())
}

#[derive(Debug)]
struct Member {
    peers: Vec<String>,
    socket: UdpSocket,
}

impl Member {
    fn new(config: Config) -> anyhow::Result<Self> {
        let socket = UdpSocket::bind(config.address)?;
        socket.set_write_timeout(Some(Duration::from_millis(config.timeout)))?;

        Ok(Member {
            socket: socket,
            peers: Vec::new(),
        })
    }

    fn heartbeat(&self) -> anyhow::Result<()> {
        let mut rng = rand::thread_rng();
        match self.peers.choose(&mut rng) {
            Some(peer) => self.ping(peer),
            None => Ok(()),
        }
    }

    fn ping(&self, peer: &str) -> anyhow::Result<()> {
        let data = serialize(&Message::Ping)?;
        self.send(peer, &data)
    }

    fn ping_req(&self, peer: &str) -> anyhow::Result<()> {
        let data = serialize(&Message::PingReq)?;
        self.send(peer, &data)
    }

    fn ack(&self, peer: &str) -> anyhow::Result<()> {
        let data = serialize(&Message::Ack)?;
        self.send(peer, &data)
    }

    fn send(&self, peer: &str, data: &[u8]) -> anyhow::Result<()> {
        match self.socket.send_to(&data, peer) {
            Err(e) => Err(Error::new(e)),
            Ok(n) => match n {
                _ if n == data.len() => Ok(()),
                n => Err(anyhow!(
                    "expected to write {} bytes but wrote {}",
                    data.len(),
                    n,
                )),
            },
        }
    }
}

trait Sender {
    fn send(&self, peer: String, data: &[u8]);
}

#[derive(Debug, Default)]
struct Config {
    address: String,
    fanout: u32,
    period: u64,
    timeout: u64,
}

#[derive(Debug, Deserialize, Serialize)]
enum Message {
    Ping,
    PingReq,
    Ack,
}
