/*!
 * CouncilEventBus with CRC64 Checksums for Stronger Data Integrity.
 *
 * Uses CRC-64 (ECMA-182) for significantly better error detection than CRC32.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use std::fs::{self, File, OpenOptions, rename};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use flume::{Receiver, Sender};
use serde::{Deserialize, Serialize};

use crc::{Crc, CRC_64_ECMA_182};

use crate::world::AgentId;

#[cfg(feature = "bevy")]
use bevy::prelude::Resource;

const CRC64: Crc<u64> = Crc::<u64>::new(&CRC_64_ECMA_182);

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

#[derive(Clone, Debug)]
pub struct LogRotationConfig {
    pub max_size_bytes: u64,
    pub max_files: usize,
}

impl Default for LogRotationConfig {
    fn default() -> Self {
        Self { max_size_bytes: 100 * 1024 * 1024, max_files: 5 }
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
        Self { tx, rx, persistence: None, log_path: None, rotation_config: None, wal_mode: false, fsync_interval: 0, write_count: Arc::new(Mutex::new(0)) }
    }

    pub fn new_bounded_with_persistence<P: AsRef<Path>>(capacity: usize, path: P) -> std::io::Result<Self> {
        Self::new_with_rotation_internal(capacity, path, None, false, 0)
    }

    pub fn new_bounded_with_rotation<P: AsRef<Path>>(capacity: usize, path: P, config: LogRotationConfig) -> std::io::Result<Self> {
        Self::new_with_rotation_internal(capacity, path, Some(config), true, 50)
    }

    pub fn new_bounded_with_write_ahead_log<P: AsRef<Path>>(capacity: usize, path: P, fsync_interval: u32) -> std::io::Result<Self> {
        Self::new_with_rotation_internal(capacity, path, None, true, fsync_interval)
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
                if let (Some(path), Some(config)) = (&self.log_path, &self.rotation_config) {
                    if let Ok(metadata) = fs::metadata(path) {
                        if metadata.len() > config.max_size_bytes {
                            let _ = self.rotate_logs(path, config);
                            if let Ok(new_file) = OpenOptions::new().create(true).append(true).open(path) {
                                *guard = BufWriter::new(new_file);
                            }
                        }
                    }
                }

                if let Ok(json) = serde_json::to_string(&event) {
                    let checksum = compute_crc64(&json);
                    let line = format!("{}|crc64:{:016x}", json, checksum);

                    let _ = writeln!(guard, "{}", line);
                    let _ = guard.flush();

                    let mut count = self.write_count.lock().unwrap();
                    *count += 1;

                    let should_sync = if self.fsync_interval == 0 { self.wal_mode } else { *count % self.fsync_interval == 0 };
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
        let oldest = current_path.with_extension(format!("jsonl.{}", config.max_files));
        if oldest.exists() { let _ = fs::remove_file(oldest); }

        for i in (1..config.max_files).rev() {
            let src = current_path.with_extension(format!("jsonl.{}", i));
            let dst = current_path.with_extension(format!("jsonl.{}", i + 1));
            if src.exists() { let _ = rename(src, dst); }
        }

        let rotated = current_path.with_extension("jsonl.1");
        rename(current_path, rotated)?;
        Ok(())
    }

    pub fn try_recv(&self) -> Result<CouncilEvent, flume::TryRecvError> { self.rx.try_recv() }
    pub fn recv(&self) -> Result<CouncilEvent, flume::RecvError> { self.rx.recv() }
    pub fn sender(&self) -> Sender<CouncilEvent> { self.tx.clone() }
    pub fn receiver(&self) -> Receiver<CouncilEvent> { self.rx.clone() }
}

impl Default for CouncilEventBus {
    fn default() -> Self { Self::new_bounded(1024) }
}

fn compute_crc64(data: &str) -> u64 {
    let mut digest = CRC64.digest();
    digest.update(data.as_bytes());
    digest.finalize()
}

pub fn load_events_from_log<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<CouncilEvent>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut events = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() { continue; }

        let json_part = if let Some(pos) = line.rfind("|crc64:") {
            &line[..pos]
        } else if let Some(pos) = line.rfind("|crc32:") { // backward compat
            &line[..pos]
        } else {
            &line
        };

        if let Ok(event) = serde_json::from_str::<CouncilEvent>(json_part) {
            events.push(event);
        }
    }

    Ok(events)
}

pub fn load_all_events_with_rotation<P: AsRef<Path>>(base_path: P) -> std::io::Result<Vec<CouncilEvent>> {
    let base = base_path.as_ref();
    let mut all_events = Vec::new();

    for i in (1..=20).rev() {
        let rotated = base.with_extension(format!("jsonl.{}", i));
        if rotated.exists() {
            if let Ok(mut evs) = load_events_from_log(&rotated) {
                all_events.append(&mut evs);
            }
        }
    }

    if base.exists() {
        if let Ok(mut evs) = load_events_from_log(base) {
            all_events.append(&mut evs);
        }
    }

    Ok(all_events)
}
