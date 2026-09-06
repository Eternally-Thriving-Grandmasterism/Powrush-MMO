//! Loopback WS accept loop for powrush-shard (F8).
//! JSON envelopes only. No TLS theatre. Contact: info@Rathor.ai

use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use futures_util::{SinkExt, StreamExt};
use shared::hex_listen::{
    apply_client_op, client_drop, handle_hello, hello_reply_envelope, presence_house_count,
    HelloSession,
};
use shared::hex_protocol::{Envelope, Snapshot};
use shared::hex_shard_apply::{
    write_snapshot_json, ShardLedger, SNAPSHOT_FILE,
};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::accept_async;

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn persist(data: &Path, snap: &Snapshot) -> Result<(), String> {
    std::fs::create_dir_all(data).map_err(|e| e.to_string())?;
    let path = data.join(SNAPSHOT_FILE);
    let raw = write_snapshot_json(snap).map_err(|e| e.to_string())?;
    std::fs::write(&path, raw).map_err(|e| e.to_string())?;
    Ok(())
}

struct ShardState {
    ledger: ShardLedger,
    data: PathBuf,
    /// conn_id → house id (presence = real connections only)
    seats: HashMap<u64, String>,
    next_conn: u64,
}

impl ShardState {
    fn presence_len(&self) -> usize {
        presence_house_count(&self.ledger.presence)
    }
}

pub async fn run_loopback_ws(addr: SocketAddr, ledger: ShardLedger, data: PathBuf) -> Result<(), String> {
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| format!("bind {addr} failed: {e}"))?;
    eprintln!(
        "powrush-shard: WS listen {} (loopback only) hex={} soft presence={}",
        addr,
        ledger.hex_id,
        presence_house_count(&ledger.presence)
    );
    let state = Arc::new(Mutex::new(ShardState {
        ledger,
        data,
        seats: HashMap::new(),
        next_conn: 1,
    }));

    loop {
        let (stream, peer) = listener
            .accept()
            .await
            .map_err(|e| format!("accept failed: {e}"))?;
        if !peer.ip().is_loopback() {
            eprintln!("powrush-shard: refusing non-loopback peer {peer}");
            continue;
        }
        let state = Arc::clone(&state);
        tokio::spawn(async move {
            if let Err(e) = handle_conn(stream, peer, state).await {
                eprintln!("powrush-shard: conn {peer} end: {e}");
            }
        });
    }
}

async fn handle_conn(
    stream: TcpStream,
    peer: SocketAddr,
    state: Arc<Mutex<ShardState>>,
) -> Result<(), String> {
    let ws = accept_async(stream)
        .await
        .map_err(|e| format!("ws handshake: {e}"))?;
    let (mut sink, mut stream) = ws.split();
    let conn_id = {
        let mut g = state.lock().await;
        let id = g.next_conn;
        g.next_conn += 1;
        id
    };
    eprintln!("powrush-shard: peer {peer} conn={conn_id} open");

    let mut seated_house: Option<String> = None;

    let mut read_err: Option<String> = None;
    while let Some(msg) = stream.next().await {
        let msg = match msg {
            Ok(m) => m,
            Err(e) => {
                read_err = Some(format!("ws read: {e}"));
                break;
            }
        };
        if msg.is_close() {
            break;
        }
        let text = match msg {
            Message::Text(t) => t,
            Message::Binary(b) => String::from_utf8_lossy(&b).into_owned(),
            Message::Ping(p) => {
                let _ = sink.send(Message::Pong(p)).await;
                continue;
            }
            Message::Pong(_) | Message::Frame(_) => continue,
            Message::Close(_) => break,
        };
        let env: Envelope = match serde_json::from_str(&text) {
            Ok(e) => e,
            Err(e) => {
                let rej = Envelope::new(
                    "reject",
                    "?",
                    "?",
                    0,
                    now_ms(),
                    serde_json::json!({ "code": "PROTO", "reason": format!("bad json: {e}") }),
                );
                let raw = serde_json::to_string(&rej).unwrap_or_default();
                sink.send(Message::Text(raw)).await.ok();
                continue;
            }
        };

        let reply = {
            let mut g = state.lock().await;
            if env.kind == "hello" {
                let session = handle_hello(&mut g.ledger, &env);
                if let HelloSession::Ok { house, .. } = &session {
                    seated_house = Some(house.clone());
                    g.seats.insert(conn_id, house.clone());
                }
                let out = hello_reply_envelope(&g.ledger.hex_id, &env.house, &session, now_ms());
                if let Err(e) = persist(&g.data, &g.ledger.snapshot) {
                    eprintln!("powrush-shard: persist after hello: {e}");
                }
                eprintln!(
                    "powrush-shard: hello → {} presence={}",
                    out.kind,
                    g.presence_len()
                );
                out
            } else {
                let out = apply_client_op(&mut g.ledger, &env, now_ms());
                if out.kind == "apply" {
                    if let Err(e) = persist(&g.data, &g.ledger.snapshot) {
                        eprintln!("powrush-shard: persist after apply: {e}");
                    }
                }
                eprintln!(
                    "powrush-shard: {} seq={} → {} presence={}",
                    env.kind,
                    env.seq,
                    out.kind,
                    g.presence_len()
                );
                out
            }
        };

        match serde_json::to_string(&reply) {
            Ok(raw) => {
                if let Err(e) = sink.send(Message::Text(raw)).await {
                    read_err = Some(format!("ws write: {e}"));
                    break;
                }
            }
            Err(e) => {
                read_err = Some(e.to_string());
                break;
            }
        }
    }

    // Client drop → Offline; book/house intact on disk (always, even on abrupt reset).
    {
        let mut g = state.lock().await;
        if let Some(house) = seated_house.or_else(|| g.seats.remove(&conn_id)) {
            g.seats.remove(&conn_id);
            let leave = client_drop(&mut g.ledger, &house);
            let _ = leave;
            if let Err(e) = persist(&g.data, &g.ledger.snapshot) {
                eprintln!("powrush-shard: persist after drop: {e}");
            }
            eprintln!(
                "powrush-shard: drop house={house} presence={} (offline; book intact)",
                g.presence_len()
            );
        }
    }
    if let Some(e) = read_err {
        Err(e)
    } else {
        Ok(())
    }
}
