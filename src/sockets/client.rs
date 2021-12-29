use std::net::{Ipv4Addr, SocketAddr};

use async_trait::async_trait;
use mio::net::UdpSocket;
use prost::Message;

use crate::{types, CandelaClient, CandelaClientConfig, Result};

pub struct CandelaSocketClient {
    context: zmq::Context,
    socket: zmq::Socket,
    setup_port: u16,
    outgoing_buf: Vec<u8>,
    incoming_buf: Vec<u8>,
}

#[async_trait]
impl CandelaClient for CandelaSocketClient {
    fn new<T: CandelaClientConfig>(config: T) -> Result<Self> {
        let context = zmq::Context::new();
        let socket = context.socket(zmq::REP)?;
        Ok(CandelaSocketClient {
            context,
            socket,
            setup_port: config.get_setup_port(),
            outgoing_buf: Vec::<u8>::with_capacity(10000),
            incoming_buf: Vec::<u8>::with_capacity(1000 * 1000),
        })
    }
    
    async fn setup(&mut self) -> Result<()> {
        // Set up the UDP socket for the initial handshake.
        let addr = Ipv4Addr::new(0, 0, 0, 0);
        let bind_addr = SocketAddr::from((addr, self.setup_port));
        let setup_socket = UdpSocket::bind(bind_addr)?;
        setup_socket.set_broadcast(true)?;

        // Listen for broadcast packet

        // Create controller info message

        // Send controller info message

        // Create ZMQ socket

        // Wait for a bounded amount of time for a ZMQ acknowledgement

        unimplemented!()
    }

    async fn recv(&mut self) -> Result<types::ClientMessage> {
        let recv_length = self.socket.recv_into(&mut self.incoming_buf, 0)?;
        return Ok(types::ClientMessage::decode_length_delimited(
            &self.incoming_buf[0..recv_length],
        )?);
    }
}
