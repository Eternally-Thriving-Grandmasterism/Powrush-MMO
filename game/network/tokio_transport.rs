// game/network/tokio_transport.rs
// Powrush-MMO — Tokio-based Hybrid TCP + UDP Transport (v2)
// AG-SML v1.0 License

use crate::game::network::transport::{
    ConnectionId, NetworkEvent, NetworkTransport, SendReliability,
};
use crate::game::networking::{ClientMessage, ServerMessage};
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::sync::mpsc;

struct Connection {
    tcp_stream: TcpStream,
    udp_address: Option<SocketAddr>,
}

pub struct TokioTransport {
    tcp_listener: Option<TcpListener>,
    udp_socket: Option<UdpSocket>,
    connections: HashMap<ConnectionId, Connection>,
    next_connection_id: ConnectionId,
    event_sender: mpsc::UnboundedSender<NetworkEvent>,
    event_receiver: mpsc::UnboundedReceiver<NetworkEvent>,
}

impl TokioTransport {
    pub fn new() -> Self {
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        Self {
            tcp_listener: None,
            udp_socket: None,
            connections: HashMap::new(),
            next_connection_id: 1,
            event_sender,
            event_receiver,
        }
    }
}

impl NetworkTransport for TokioTransport {
    fn start(&mut self) -> Result<(), String> {
        let listener = tokio::runtime::Handle::current()
            .block_on(async {
                TcpListener::bind("0.0.0.0:7777")
                    .await
                    .map_err(|e| format!("Failed to bind TCP: {}", e))
            })?;

        let udp = tokio::runtime::Handle::current()
            .block_on(async {
                UdpSocket::bind("0.0.0.0:7777")
                    .await
                    .map_err(|e| format!("Failed to bind UDP: {}", e))
            })?;

        self.tcp_listener = Some(listener);
        self.udp_socket = Some(udp);

        println!("[TokioTransport] Listening on TCP + UDP :7777");
        Ok(())
    }

    fn send(
        &mut self,
        connection_id: ConnectionId,
        message: ServerMessage,
        reliability: SendReliability,
    ) -> Result<(), String> {
        let conn = self
            .connections
            .get_mut(&connection_id)
            .ok_or_else(|| format!("Connection {} not found", connection_id))?;

        let bytes = bincode::serialize(&message)
            .map_err(|e| format!("Serialize error: {}", e))?;

        match reliability {
            SendReliability::Reliable => {
                let stream = &mut conn.tcp_stream;
                let len = bytes.len() as u32;
                let len_bytes = len.to_be_bytes();

                tokio::runtime::Handle::current().block_on(async {
                    stream.write_all(&len_bytes).await.map_err(|e| e.to_string())?;
                    stream.write_all(&bytes).await.map_err(|e| e.to_string())?;
                    Ok(())
                })
            }
            SendReliability::Unreliable => {
                if let Some(addr) = conn.udp_address {
                    let udp = self.udp_socket.as_ref().ok_or("UDP socket not ready")?;
                    tokio::runtime::Handle::current().block_on(async {
                        udp.send_to(&bytes, addr).await.map_err(|e| e.to_string())?;
                        Ok(())
                    })
                } else {
                    Err("UDP address not associated with connection".to_string())
                }
            }
        }
    }

    fn broadcast(&mut self, message: ServerMessage, reliability: SendReliability) {
        let ids: Vec<_> = self.connections.keys().cloned().collect();
        for id in ids {
            let _ = self.send(id, message.clone(), reliability);
        }
    }

    fn poll_events(&mut self) -> Vec<NetworkEvent> {
        let mut events = vec![];
        while let Ok(ev) = self.event_receiver.try_recv() {
            events.push(ev);
        }
        events
    }

    fn disconnect(&mut self, connection_id: ConnectionId) {
        if let Some(conn) = self.connections.remove(&connection_id) {
            // Best effort close
            let _ = conn.tcp_stream;
            let _ = self.event_sender.send(NetworkEvent::ClientDisconnected { connection_id });
        }
    }

    fn is_connected(&self, connection_id: ConnectionId) -> bool {
        self.connections.contains_key(&connection_id)
    }

    fn connected_clients(&self) -> usize {
        self.connections.len()
    }
}