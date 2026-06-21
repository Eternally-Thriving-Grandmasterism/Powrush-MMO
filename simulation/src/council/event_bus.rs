/*!
 * CouncilEventBus with Log Rotation support.
 *
 * Prevents unbounded growth of the persistence log by automatically rotating files
 * when they reach a configured size limit.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use std::fs::{self, File, OpenOptions, rename};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use flume::{Receiver, Sender};
use serde::{Deserialize, Serialize};

use crate::world::AgentId;

#[cfg(feature = "bevy")]
use bevy::prelude::Resource;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CouncilEvent {
    ProposalSubmitted {
        proposal_id: u64,
        proposer: AgentId,
        proposal_type: crate::council::ProposalType,
        title: String,
    },
    ProposalPassed {
        proposal_id: u64,
        votes_for: u32,
        votes_against: u32,
        mercy_factor: f32,
    },
    DecisionApplied {
        proposal_id: u64,
        effect_type: String,
        magnitude: f32,
        applied_tick: u64,
    },
    AuditLogAppended {
        decision_id: u64,
        proposer: AgentId,
        mercy_factor: f32,
    },
}

/// Configuration for log rotation.
#[derive(Clone, Debug)]
pub struct LogRotationConfig {
    pub max_size_bytes: u64,     // Rotate when file exceeds this size
    pub max_files: usize,        // How many rotated files to keep (e.g. 5)
}

impl Default for LogRotationConfig {
    fn default() -> Self {
        Self {
            max_size_bytes: 100 * 1024 * 1024, // 100 MB default
            max_files: 5,
        }
    }
}

#[cfg_attr(feature = "bevy", derive(Resource))]
#[derive(Clone)]
pub struct CouncilEventBus {
    tx: Sender<CouncilEvent>,
    rx: Receiver<CouncilEvent>,
    persistence: Option<Arc<Mutex<BufWriter<File>>>>,
    log_path: Option<PathBuf>,
    rotation_config: Option<LogRotationConfig>,
    wal_mode: bool,
    fsync_interval: u32,
    write_count: Arc<Mutex<u32>>,
}

impl CouncilEventBus {
    pub fn new_bounded(capacity: usize) -> Self {
        let (tx, rx) = flume::bounded(capacity);
        Self {
            tx, rx,
            persistence: None,
            log_path: None,
            rotation_config: None,
            wal_mode: false,
            fsync_interval: 0,
            write_count: Arc::new(Mutex::new(0)),
        }
    }

    pub fn new_bounded_with_persistence<P: AsRef<Path>>(capacity: usize, path: P) -> std::io::Result<Self> {
        Self::new_with_rotation_internal(capacity, path, None, false, 0)
    }

    pub fn new_bounded_with_batched_durability<P: AsRef<Path>>(capacity: usize, path: P, fsync_interval: u32) -> std::io::Result<Self> {
        Self::new_with_rotation_internal(capacity, path, None, true, fsync_interval)
    }

    pub fn new_bounded_with_write_ahead_log<P: AsRef<Path>>(capacity: usize, path: P, fsync_interval: u32) -> std::io::Result<Self> {
        Self::new_with_rotation_internal(capacity, path, None, true, fsync_interval)
    }

    /// Creates a persistent event bus with automatic log rotation.
    pub fn new_bounded_with_rotation<P: AsRef<Path>>(capacity: usize, path: P, config: LogRotationConfig) -> std::io::Result<Self> {
        Self::new_with_rotation_internal(capacity, path, Some(config), true, 50)
    }

    fn new_with_rotation_internal<P: AsRef<Path>>(capacity: usize, path: P, rotation: Option<LogRotationConfig>, durable: bool, fsync_interval: u32) -> std::io::Result<Self> {
        let path_buf = path.as_ref().to_path_buf();
        let file = OpenOptions::new().create(true).append(true).open(&path_buf)?;
        let writer = BufWriter::new(file);

        let (tx, rx) = flume::bounded(capacity);

        Ok(Self {
            tx, rx,
            persistence: Some(Arc::new(Mutex::new(writer))),
            log_path: Some(path_buf),
            rotation_config: rotation,
            wal_mode: durable,
            fsync_interval,
            write_count: Arc::new(Mutex::new(0)),
        })
    }

    pub fn send(&self, event: CouncilEvent) -> Result<(), flume::SendError<CouncilEvent>> {
        if let Some(writer) = &self.persistence {
            if let Ok(mut guard) = writer.lock() {
                // Check for rotation before writing
                if let (Some(path), Some(config)) = (&self.log_path, &self.rotation_config) {
                    if let Ok(metadata) = fs::metadata(path) {
                        if metadata.len() > config.max_size_bytes {
                            // Perform rotation
                            let _ = self.rotate_logs(path, config);
                            // Reopen new file
                            if let Ok(new_file) = OpenOptions::new().create(true).append(true).open(path) {
                                *guard = BufWriter::new(new_file);
                            }
                        }
                    }
                }

                if let Ok(json) = serde_json::to_string(&event) {
                    let _ = writeln!(guard, "{}", json);
                    let _ = guard.flush();

                    // Durability (WAL / batched fsync)
                    let mut count = self.write_count.lock().unwrap();
                    *count += 1;

                    let should_sync = if self.fsync_interval == 0 {
                        self.wal_mode
                    } else {
                        *count % self.fsync_interval == 0
                    };

                    if should_sync {
                        if let Ok(file) = guard.get_ref().try_clone() {
                            let _ = file.sync_all();
                        }
                    }
                }
            }
        }

        self.tx.send(event)
    }

    fn rotate_logs(&self, current_path: &Path, config: &LogRotationConfig) -> std::io::Result<()> {
        // Remove oldest rotated file if we have too many
        let oldest = current_path.with_extension(format!("jsonl.{}", config.max_files));
        if oldest.exists() {
            fs::remove_file(oldest)?;
        }

        // Shift existing rotated files
        for i in (1..config.max_files).rev() {
            let src = current_path.with_extension(format!("jsonl.{}", i));
            let dst = current_path.with_extension(format!("jsonl.{}", i + 1));
            if src.exists() {
                rename(src, dst)?;
            }
        }

        // Rename current file to .1
        let rotated = current_path.with_extension("jsonl.1");
        rename(current_path, rotated)?;

        Ok(())
    }

    pub fn try_recv(&self) -> Result<CouncilEvent, flume::TryRecvError> {
        self.rx.try_recv()
    }

    pub fn recv(&self) -> Result<CouncilEvent, flume::RecvError> {
        self.rx.recv()
    }

    pub fn sender(&self) -> Sender<CouncilEvent> {
        self.tx.clone()
    }

    pub fn receiver(&self) -> Receiver<CouncilEvent> {
        self.rx.clone()
    }
}

impl Default for CouncilEventBus {
    fn default() -> Self {
        Self::new_bounded(1024)
    }
}

pub fn load_events_from_log<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<CouncilEvent>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut events = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() { continue; }
        if let Ok(event) = serde_json::from_str::<CouncilEvent>(&line) {
            events.push(event);
        }
    }

    Ok(events)
}
