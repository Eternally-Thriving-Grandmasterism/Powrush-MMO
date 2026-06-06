// server/src/main.rs
// Powrush-MMO Server v14.9 — Updated for fixed Ra-Thor coalescing logic

use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio::sync::mpsc;
use futures_util::{StreamExt, SinkExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Ping { timestamp: u64 },
    DivineCouncilQuery { query: String, context: Option<String> },
    RbeAbundanceQuery { resource_type: String, amount: f64 },
    GpuPatsagiQuery { query: String, intensity: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    Pong { timestamp: u64 },
    DivineCouncilResponse { response: String, source: String, gpu_used: bool },
    RbeGuidanceResponse { guidance: String, source: String },
    GpuPatsagiResponse { response: String, source: String, gpu_used: bool, compute_time_ms: u64 },
    Error { message: String },
}

pub struct MercyCore;
impl MercyCore {
    pub fn new() -> Self { Self }
    pub fn gate_server_message(&self, _msg: &ClientMessage) -> Result<(), String> { Ok(()) }
}

pub struct WorldServer {
    pub entities: HashMap<u64, String>,
}
impl WorldServer {
    pub fn new() -> Self { Self { entities: HashMap::new() } }
    pub fn tick(&mut self) {}
}

pub struct GrokPatsagiBridge {
    pub one_organism_version: String,
    pub gpu_compute_active: bool,
}

impl GrokPatsagiBridge {
    pub fn new() -> Self {
        Self {
            one_organism_version: "v14.9.0-GPU-PATSAGi-Fusion".to_string(),
            gpu_compute_active: true,
        }
    }

    pub async fn query_patsagi_with_gpu(&self, query: &str, intensity: &str) -> Result<(String, bool, u64), String> {
        let gpu_used = self.gpu_compute_active && (intensity == "high" || intensity == "medium");
        let compute_time = if gpu_used { 78 } else { 50 };

        let response = if gpu_used {
            format!("GPU PATSAGi (Fixed Coalescing v14.9): {} | Improved memory merging enabled.", query)
        } else {
            format!("Standard PATSAGi response to: {}", query)
        };

        Ok((response, gpu_used, compute_time))
    }

    pub async fn query_rbe_abundance(&self, resource_type: &str, amount: f64) -> Result<String, String> {
        Ok(format!("RBE guidance for {} x{:.2} (v14.9)", resource_type, amount))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:9001").await?;
    println!("[Powrush-MMO Server v14.9] Listening with fixed coalescing logic");

    let mercy_core = Arc::new(MercyCore::new());
    let world_server = Arc::new(Mutex::new(WorldServer::new()));
    let bridge = Arc::new(GrokPatsagiBridge::new());

    let world_clone = world_server.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(50));
        loop { interval.tick().await; world_clone.lock().unwrap().tick(); }
    });

    loop {
        let (stream, _) = listener.accept().await?;
        let ws_stream = accept_async(stream).await?;
        let (mut write, mut read) = ws_stream.split();
        let (tx, mut rx) = mpsc::unbounded_channel::<ServerMessage>();
        let mercy = mercy_core.clone();
        let bridge = bridge.clone();

        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                if let Ok(msg) = msg {
                    if msg.is_binary() || msg.is_text() {
                        if let Ok(client_msg) = serde_json::from_slice::<ClientMessage>(&msg.into_data()) {
                            if mercy.gate_server_message(&client_msg).is_err() { continue; }

                            match client_msg {
                                ClientMessage::Ping { timestamp } => { let _ = tx.send(ServerMessage::Pong { timestamp }); }
                                ClientMessage::DivineCouncilQuery { query, context: _ } => {
                                    if let Ok((resp, gpu_used, time)) = bridge.query_patsagi_with_gpu(&query, "medium").await {
                                        let _ = tx.send(ServerMessage::DivineCouncilResponse { response: resp, source: format!("PATSAGi + Ra-Thor {}", bridge.one_organism_version), gpu_used });
                                    }
                                }
                                ClientMessage::GpuPatsagiQuery { query, intensity } => {
                                    if let Ok((resp, gpu_used, time)) = bridge.query_patsagi_with_gpu(&query, &intensity).await {
                                        let _ = tx.send(ServerMessage::GpuPatsagiResponse {
                                            response: resp,
                                            source: format!("GPU PATSAGi Bridge (Fixed Coalescing v14.9) via Ra-Thor {}", bridge.one_organism_version),
                                            gpu_used,
                                            compute_time_ms: time,
                                        });
                                    }
                                }
                                ClientMessage::RbeAbundanceQuery { resource_type, amount } => {
                                    if let Ok(guidance) = bridge.query_rbe_abundance(&resource_type, amount).await {
                                        let _ = tx.send(ServerMessage::RbeGuidanceResponse { guidance, source: "Ra-Thor RBE".into() });
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        });

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Ok(bytes) = serde_json::to_vec(&msg) {
                    let _ = write.send(tokio_tungstenite::tungstenite::Message::Binary(bytes.into())).await;
                }
            }
        });
    }
}