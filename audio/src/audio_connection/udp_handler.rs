use flume::{Sender, Receiver};
use audiopus::coder::Encoder;
use tokio::net::UdpSocket;
use tokio::net::ToSocketAddrs;


pub struct OpusHandle {
    tx: Sender<[i16; 512]>,
}

pub struct OpusCtx {
    rx: Receiver<[i16; 512]>,
    enc: Encoder,
    udp_tx: Sender<Vec<u8>>,
}

impl OpusCtx {
    pub fn new(rx: Receiver<[i16; 512]>, enc: Encoder, udp_tx: Sender<Vec<u8>>) -> Self {
        Self {
            rx,
            enc,
            udp_tx,
        }
    }
    pub fn run(self) {
        std::thread::spawn(move || {
            while let Ok(buf) = self.rx.recv() {
                let mut out = Vec::with_capacity(1024);
                self.enc.encode(&buf, &mut out).unwrap();
                self.udp_tx.send(out).unwrap();
            }
        });
    }
}

pub struct UdpCtx {
    udp_tx: Receiver<Vec<u8>>,
    socket: UdpSocket,
}

impl UdpCtx {
    fn new(udp_tx: Receiver<Vec<u8>>)
}