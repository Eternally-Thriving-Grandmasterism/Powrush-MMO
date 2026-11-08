//! U1 — writable user-dir persist (Steam / offline)
//!
//! House, settings, GenShare, and the other `powrush_*` JSON the client
//! already writes land in an OS user-data directory, not only beside the
//! exe (Program Files is a fail).
//!
//! ## Directory rule
//!
//! 1. **`POWRUSH_USER_DIR`** — if set and non-empty, that directory
//!    (lab / lavapipe override). Same role as today's cwd `data/`:
//!    the JSON files live *in* this directory.
//! 2. Else the OS user-data dir:
//!    - Linux: `$XDG_DATA_HOME/powrush` or `~/.local/share/powrush`
//!    - Windows: `%LOCALAPPDATA%\Powrush`
//!    - macOS: `~/Library/Application Support/Powrush`
//! 3. Last resort (no HOME / LOCALAPPDATA): cwd `data/`.
//!
//! If the resolved dir has no `powrush_*` persist files and cwd `data/`
//! does, those files are copied once so current lab walks do not go blank.
//! Writes always go to the resolved dir. No Steamworks account or cloud API.
//!
//! Contact: info@Rathor.ai. Independent of xAI.

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Lab / lavapipe override. Directory that holds the persist JSON files.
pub const USER_DIR_OVERRIDE_ENV: &str = "POWRUSH_USER_DIR";

/// Linux XDG application directory name.
pub const LINUX_APP_DIR: &str = "powrush";
/// Windows / macOS application directory name.
pub const APP_DIR_DISPLAY: &str = "Powrush";

/// Cwd-relative fallback / adopt source (legacy lab walks).
pub const CWD_DATA_REL: &str = "data";

/// Resolve the persist directory (create + adopt cwd `data/` when empty).
pub fn persist_dir() -> PathBuf {
    let dir = resolve_persist_dir(override_from_env(), os_user_data_dir());
    let _ = fs::create_dir_all(&dir);
    let _ = maybe_adopt_cwd(&dir, &cwd_data_dir());
    dir
}

/// Path for a persist filename or a cwd-relative `data/powrush_*.json` name.
pub fn persist_path(named: &str) -> PathBuf {
    persist_dir().join(persist_file_name(named))
}

/// Soft-read a persist file from the resolved user dir.
///
/// Returns the live file when it parses as JSON (`serde_json::Value`).
/// When the live file is missing or does not parse, returns the sibling
/// `.bak` if that file parses. Same good-save rule as [`write_named`].
pub fn read_named(named: &str) -> std::io::Result<String> {
    read_with_bak_fallback(&persist_path(named))
}

/// Confirmed absence of one persist sibling. Other I/O failures stay `Err`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawPersist {
    Bytes(Vec<u8>),
    NotFound,
}

fn read_exact_file(path: &Path) -> io::Result<RawPersist> {
    match fs::read(path) {
        Ok(bytes) => Ok(RawPersist::Bytes(bytes)),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(RawPersist::NotFound),
        Err(err) => Err(err),
    }
}

/// Live bytes for `named`. Does not read `.bak` and does not parse.
pub fn read_live_bytes(named: &str) -> io::Result<RawPersist> {
    read_exact_file(&persist_path(named))
}

/// Backup bytes for `named`. Does not read the live file and does not parse.
pub fn read_backup_bytes(named: &str) -> io::Result<RawPersist> {
    read_exact_file(&sibling_path(&persist_path(named), ".bak"))
}

/// Soft-write a persist file into the resolved user dir (creates the dir).
///
/// A good save parses as JSON (`serde_json::from_str::<serde_json::Value>`).
/// This module does not parse a typed schema. When the live file is a good
/// save, those live bytes are staged at a sibling `.bak.tmp`, synced, and
/// renamed onto `.bak`. New bytes are staged at a sibling `.tmp`. `fs::rename`
/// moves that temp onto the live path only after `write_all` and `sync_all`
/// succeed.
pub fn write_named(named: &str, contents: impl AsRef<[u8]>) -> std::io::Result<()> {
    write_last_good(&persist_path(named), contents.as_ref())
}

/// Whether [`read_named`] can return this persist file.
///
/// The live file counts when it parses as one JSON value
/// (`serde_json::from_str::<serde_json::Value>`). When the live file is
/// missing or does not parse, the sibling `.bak` counts only when that
/// file parses the same way. A missing `.bak`, a non-JSON `.bak`, or only
/// a `.tmp` sibling is false.
///
/// A live path that exists but is not JSON is false when the `.bak` does
/// not parse. [`read_named`] returns `Err` in that case; true here means
/// that call returns `Ok`.
pub fn named_exists(named: &str) -> bool {
    let path = persist_path(named);
    parses_as_json_file(&path) || parses_as_json_file(&sibling_path(&path, ".bak"))
}

/// Filename only (`data/powrush_house.json` → `powrush_house.json`).
pub fn persist_file_name(named: &str) -> &str {
    Path::new(named)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(named)
}

/// `POWRUSH_USER_DIR` when set and non-empty.
pub fn override_from_env() -> Option<PathBuf> {
    match std::env::var(USER_DIR_OVERRIDE_ENV) {
        Ok(v) if !v.trim().is_empty() => Some(PathBuf::from(v.trim())),
        _ => None,
    }
}

/// OS user-data dir for Powrush persist (no create).
pub fn os_user_data_dir() -> PathBuf {
    #[cfg(windows)]
    {
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            if !local.trim().is_empty() {
                return PathBuf::from(local).join(APP_DIR_DISPLAY);
            }
        }
        if let Ok(profile) = std::env::var("USERPROFILE") {
            if !profile.trim().is_empty() {
                return PathBuf::from(profile)
                    .join("AppData")
                    .join("Local")
                    .join(APP_DIR_DISPLAY);
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            if !home.is_empty() {
                return PathBuf::from(home)
                    .join("Library")
                    .join("Application Support")
                    .join(APP_DIR_DISPLAY);
            }
        }
    }
    if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
        if !xdg.trim().is_empty() {
            return PathBuf::from(xdg).join(LINUX_APP_DIR);
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        if !home.is_empty() {
            return PathBuf::from(home)
                .join(".local")
                .join("share")
                .join(LINUX_APP_DIR);
        }
    }
    PathBuf::from(CWD_DATA_REL)
}

/// Cwd `data/` — adopt source only.
pub fn cwd_data_dir() -> PathBuf {
    match std::env::current_dir() {
        Ok(cwd) => cwd.join(CWD_DATA_REL),
        Err(_) => PathBuf::from(CWD_DATA_REL),
    }
}

/// Pure resolver: override wins; else OS user-data dir.
pub fn resolve_persist_dir(override_dir: Option<PathBuf>, os_dir: PathBuf) -> PathBuf {
    match override_dir {
        Some(p) if !p.as_os_str().is_empty() => p,
        _ => os_dir,
    }
}

/// True when `dir` already holds at least one live `powrush_*` persist file.
///
/// Names ending in `.bak` or `.tmp` are siblings, not saves. `powrush_*.jsonl`
/// still counts.
pub fn persist_files_present(dir: &Path) -> bool {
    let Ok(rd) = fs::read_dir(dir) else {
        return false;
    };
    rd.flatten().any(|e| {
        let name = e.file_name();
        let s = name.to_string_lossy();
        is_live_persist_name(&s) && e.path().is_file()
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdoptKind {
    SkippedUserHasFiles,
    SkippedCwdEmpty,
    Copied,
}

/// Copy live `powrush_*` files from `cwd_data` into `user_dir` when the user
/// dir has none. Skips names ending in `.bak` or `.tmp`. `.jsonl` is copied.
pub fn maybe_adopt_cwd(user_dir: &Path, cwd_data: &Path) -> AdoptKind {
    if persist_files_present(user_dir) {
        return AdoptKind::SkippedUserHasFiles;
    }
    if !persist_files_present(cwd_data) {
        return AdoptKind::SkippedCwdEmpty;
    }
    let _ = fs::create_dir_all(user_dir);
    if let Ok(rd) = fs::read_dir(cwd_data) {
        for entry in rd.flatten() {
            let name = entry.file_name();
            let s = name.to_string_lossy();
            if !is_live_persist_name(&s) || !entry.path().is_file() {
                continue;
            }
            let dest = user_dir.join(&name);
            let _ = fs::copy(entry.path(), dest);
        }
    }
    AdoptKind::Copied
}

/// Program Files (or x86) — not a writable Steam/offline save root.
pub fn is_program_files_path(path: &Path) -> bool {
    let lower = path.to_string_lossy().to_ascii_lowercase();
    lower.contains("program files") || lower.contains("programfiles")
}

/// Beside the current exe (or `exe/data`) — Program Files install fail mode.
pub fn is_beside_exe_path(path: &Path) -> bool {
    let Ok(exe) = std::env::current_exe() else {
        return false;
    };
    let Some(parent) = exe.parent() else {
        return false;
    };
    let canon = |p: &Path| fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
    let a = canon(path);
    a == canon(parent) || a == canon(&parent.join(CWD_DATA_REL))
}

/// Writable-user-dir rule: not Program Files-only.
pub fn is_writable_user_dir_rule(path: &Path) -> bool {
    !is_program_files_path(path)
}

/// Resolved dir is not the F-book fixture tree.
pub fn is_f_book_fixture_dir(path: &Path) -> bool {
    let s = path.to_string_lossy();
    s.contains("f-book") || s.contains("tests/fixtures")
}

/// Live persist names are `powrush_*`, except sibling `.bak` and `.tmp`.
fn is_live_persist_name(name: &str) -> bool {
    name.starts_with("powrush_") && !name.ends_with(".bak") && !name.ends_with(".tmp")
}

fn sibling_path(path: &Path, suffix: &str) -> PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(suffix);
    PathBuf::from(name)
}

/// A good save reads back fully and parses as one JSON value.
fn parses_as_json_file(path: &Path) -> bool {
    match fs::read_to_string(path) {
        Ok(raw) => serde_json::from_str::<serde_json::Value>(&raw).is_ok(),
        Err(_) => false,
    }
}

fn read_with_bak_fallback(path: &Path) -> std::io::Result<String> {
    match fs::read_to_string(path) {
        Ok(raw) if serde_json::from_str::<serde_json::Value>(&raw).is_ok() => Ok(raw),
        live => match fs::read_to_string(sibling_path(path, ".bak")) {
            Ok(raw) if serde_json::from_str::<serde_json::Value>(&raw).is_ok() => Ok(raw),
            _ => match live {
                Err(err) => Err(err),
                Ok(_) => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "persist file is not json",
                )),
            },
        },
    }
}

fn ensure_parent(path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}

/// Live stage file `<live>.tmp`. Not the backup temp and not `.bak`.
fn live_stage_path(path: &Path) -> PathBuf {
    sibling_path(path, ".tmp")
}

/// Backup stage file `<live>.bak.tmp`. Distinct from `<live>.tmp`.
fn bak_stage_path(path: &Path) -> PathBuf {
    sibling_path(path, ".bak.tmp")
}

fn bak_dest_path(path: &Path) -> PathBuf {
    sibling_path(path, ".bak")
}

/// Open a stage file for replacement. `truncate` makes a preexisting temp
/// stale staging, not a save: those old bytes are discarded and never adopted.
fn create_truncating(path: &Path) -> io::Result<fs::File> {
    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
}

/// Production uses an owned [`fs::File`]. A writer that fails `write_all` or
/// `sync_all` exists only in tests. Non-test builds do not compile that type,
/// and a test hits it only after installing a fault.
trait SyncedWrite {
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()>;
    fn sync_all(&mut self) -> io::Result<()>;
}

impl SyncedWrite for fs::File {
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        Write::write_all(self, buf)
    }

    fn sync_all(&mut self) -> io::Result<()> {
        fs::File::sync_all(self)
    }
}

/// Shared by the live temp and the backup temp. `Err` stops the caller
/// before that temp's rename. Live and backup still differ: the live temp
/// receives the new bytes and is renamed onto `<live>` only from
/// [`write_last_good`]; the backup temp receives the pre-write live bytes
/// and is renamed onto `.bak`.
fn write_all_and_sync(writer: &mut impl SyncedWrite, bytes: &[u8]) -> io::Result<()> {
    writer.write_all(bytes)?;
    writer.sync_all()
}

/// Write `bytes` to `tmp` and `sync_all`. Does not rename.
///
/// Open, `write_all`, or `sync_all` failure returns here, so the caller does
/// not rename this temp. Test builds may substitute a failing writer. That
/// branch is compiled out of non-test builds, and it stays inactive unless a
/// test installs a fault.
fn write_synced_temp(tmp: &Path, bytes: &[u8]) -> io::Result<()> {
    #[cfg(test)]
    if let Some(result) = tests::apply_installed_fault(tmp, bytes) {
        return result;
    }
    let mut file = create_truncating(tmp)?;
    write_all_and_sync(&mut file, bytes)
}

/// Rename `tmp` onto `dest` only after that temp's sync succeeded, then sync
/// the parent on Unix.
///
/// A failure before `fs::rename` leaves `dest` untouched. A parent-sync error
/// after a successful rename is returned as-is; the rename is not undone.
fn rename_synced(tmp: &Path, dest: &Path) -> io::Result<()> {
    #[cfg(test)]
    if tests::fail_installed_pre_rename(tmp) {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "injected failure before rename",
        ));
    }
    fs::rename(tmp, dest)?;
    sync_parent_after_rename(dest)
}

/// Persist the directory entry from a same-directory rename.
///
/// Unix, including macOS: `File::open` the parent and `sync_all`, and
/// propagate the error. This runs only after `fs::rename` succeeded. It does
/// not undo that rename, and it is not proof of power-loss durability.
///
/// Windows: a directory cannot be opened with `File::open` the way a Unix
/// directory can, so the parent sync is skipped. The skip is best-effort
/// only and is not a power-loss durability claim. Other non-Unix targets use
/// this same skip. Std `cfg` only; no Unix-only dependency.
fn sync_parent_after_rename(dest: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        let Some(parent) = dest.parent() else {
            return Ok(());
        };
        if parent.as_os_str().is_empty() {
            return Ok(());
        }
        let dir = fs::File::open(parent)?;
        dir.sync_all()?;
        Ok(())
    }
    #[cfg(not(unix))]
    {
        let _ = dest;
        Ok(())
    }
}

/// Pre-write live bytes when that file is valid JSON.
///
/// `None` (missing or not JSON) leaves an existing `.bak` in place. The
/// bytes come from the live path, never from the new `<live>.tmp`.
fn snapshot_valid_live(path: &Path) -> io::Result<Option<Vec<u8>>> {
    if !parses_as_json_file(path) {
        return Ok(None);
    }
    Ok(Some(fs::read(path)?))
}

/// Stage snapshotted live bytes at `<live>.bak.tmp`, then rename onto `.bak`.
fn publish_backup(path: &Path, live_bytes: &[u8]) -> io::Result<()> {
    let tmp = bak_stage_path(path);
    write_synced_temp(&tmp, live_bytes)?;
    rename_synced(&tmp, &bak_dest_path(path))
}

/// Sync the new bytes to `<live>.tmp`, then publish `.bak` when the
/// pre-write live file parses. Does not rename the live temp, so a stop
/// here leaves the live file as it was.
///
/// Live temp failure returns before `.bak` is replaced. Backup failure
/// before the `.bak` rename returns with the old `.bak` still in place.
/// Names stay fixed (`.tmp`, `.bak.tmp`, `.bak`); one writer, no lock.
fn stage_tmp(path: &Path, contents: &[u8]) -> std::io::Result<PathBuf> {
    ensure_parent(path)?;
    let prior_live = snapshot_valid_live(path)?;
    let tmp = live_stage_path(path);
    write_synced_temp(&tmp, contents)?;
    if let Some(live_bytes) = prior_live {
        publish_backup(path, &live_bytes)?;
    }
    Ok(tmp)
}

/// `<live>.tmp` then rename onto the live path. `.bak` is replaced only from
/// a valid pre-write live file, through `<live>.bak.tmp`.
fn write_last_good(path: &Path, contents: &[u8]) -> std::io::Result<()> {
    let tmp = stage_tmp(path, contents)?;
    rename_synced(&tmp, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};
    use std::io::Write;

    use super::*;
    use crate::hex_listen::PowrushNet;
    use crate::house_name::HouseName;
    use crate::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "powrush-u1-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0),
            tag
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("scratch");
        dir
    }

    #[test]
    fn override_wins_over_os_user_dir() {
        let lab = PathBuf::from("/tmp/powrush-lab-walk/data");
        let os = PathBuf::from("/home/someone/.local/share/powrush");
        let got = resolve_persist_dir(Some(lab.clone()), os);
        assert_eq!(got, lab);
        assert!(is_writable_user_dir_rule(&got));
        assert!(!is_program_files_path(&got));
    }

    #[test]
    fn empty_override_falls_through_to_os_dir() {
        let os = PathBuf::from("/home/someone/.local/share/powrush");
        assert_eq!(resolve_persist_dir(None, os.clone()), os);
        assert_eq!(resolve_persist_dir(Some(PathBuf::new()), os.clone()), os);
    }

    #[test]
    fn os_user_dir_is_not_program_files() {
        let dir = os_user_data_dir();
        assert!(
            is_writable_user_dir_rule(&dir),
            "OS user-data must not be Program Files, got {}",
            dir.display()
        );
        assert!(!is_program_files_path(&dir));
        let display = dir.to_string_lossy();
        assert!(
            display.contains("powrush")
                || display.contains("Powrush")
                || dir.ends_with(CWD_DATA_REL),
            "expected named Powrush user dir, got {}",
            dir.display()
        );
    }

    #[test]
    fn program_files_is_rejected_as_save_root() {
        assert!(is_program_files_path(Path::new(
            r"C:\Program Files\Steam\steamapps\common\Powrush"
        )));
        assert!(is_program_files_path(Path::new(
            r"C:\Program Files (x86)\Steam\steamapps\common\Powrush\data"
        )));
        assert!(!is_program_files_path(Path::new(
            r"C:\Users\ada\AppData\Local\Powrush"
        )));
        assert!(!is_writable_user_dir_rule(Path::new(
            r"C:\Program Files\Powrush"
        )));
    }

    #[test]
    fn persist_file_name_strips_data_prefix() {
        assert_eq!(
            persist_file_name("data/powrush_house.json"),
            "powrush_house.json"
        );
        assert_eq!(
            persist_file_name("powrush_settings.json"),
            "powrush_settings.json"
        );
        assert_eq!(
            persist_file_name("data/powrush_genshare.jsonl"),
            "powrush_genshare.jsonl"
        );
    }

    #[test]
    fn empty_user_dir_adopts_cwd_data() {
        let root = scratch("adopt");
        let cwd = root.join("cwd-data");
        let user = root.join("user");
        fs::create_dir_all(&cwd).unwrap();
        fs::create_dir_all(&user).unwrap();
        fs::write(
            cwd.join("powrush_house.json"),
            r#"{"schema":"powrush_house_v1"}"#,
        )
        .unwrap();
        fs::write(cwd.join("nevc_status.json"), "{}").unwrap();
        assert_eq!(maybe_adopt_cwd(&user, &cwd), AdoptKind::Copied);
        assert!(user.join("powrush_house.json").exists());
        assert!(
            !user.join("nevc_status.json").exists(),
            "adopt only powrush_* persist files"
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn nonempty_user_dir_ignores_cwd_data() {
        let root = scratch("keep");
        let cwd = root.join("cwd-data");
        let user = root.join("user");
        fs::create_dir_all(&cwd).unwrap();
        fs::create_dir_all(&user).unwrap();
        fs::write(cwd.join("powrush_house.json"), "cwd").unwrap();
        fs::write(user.join("powrush_house.json"), "user").unwrap();
        assert_eq!(maybe_adopt_cwd(&user, &cwd), AdoptKind::SkippedUserHasFiles);
        let raw = fs::read_to_string(user.join("powrush_house.json")).unwrap();
        assert_eq!(raw, "user");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn write_lands_in_named_dir_not_program_files() {
        let dir = scratch("write");
        let path = dir.join("powrush_house.json");
        let mut house = HouseName::default();
        house.skip();
        fs::write(&path, house.to_json().unwrap()).unwrap();
        assert!(path.exists());
        assert!(is_writable_user_dir_rule(&dir));
        assert!(!is_program_files_path(&dir));
        assert!(!is_f_book_fixture_dir(&dir));
        let back = HouseName::from_json(&fs::read_to_string(&path).unwrap()).unwrap();
        assert!(back.resolved);
        assert_eq!(back.display_name(), crate::house_name::UNNAMED);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn stranger_boot_does_not_load_f_book() {
        let fresh = HouseName::default();
        assert!(!fresh.resolved);
        assert_eq!(fresh.display_name(), crate::house_name::UNNAMED);
        let (pack, _c, standing, _w, booked) = crate::f_book_fixture::load_f_book_disk();
        assert!(booked.resolved);
        assert!(pack.complete && pack.hour_three_complete);
        assert_ne!(fresh.resolved, booked.resolved);
        assert!(!standing.declared_lethal);

        let empty = scratch("stranger");
        assert!(!persist_files_present(&empty));
        assert!(!is_f_book_fixture_dir(&empty));
        assert!(!crate::f_book_fixture::F_BOOK_DATA_REL.eq("data"));
        assert!(crate::f_book_fixture::fixture_is_not_default_door());
        let resolved = resolve_persist_dir(Some(empty.clone()), os_user_data_dir());
        assert_eq!(resolved, empty);
        assert!(!is_f_book_fixture_dir(&resolved));
        assert!(!named_exists_in(&empty, "powrush_house.json"));
        let _ = fs::remove_dir_all(&empty);
    }

    fn named_exists_in(dir: &Path, file: &str) -> bool {
        dir.join(file).exists()
    }

    #[test]
    fn online_stays_grey() {
        assert!(!PowrushNet::Off.title_online_enabled());
        assert!(!PowrushNet::Localhost.title_online_enabled());
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(!online_row_is_honest_disabled(ONLINE_STUB_LABEL, true));
        assert_eq!(ONLINE_STUB_LABEL, "Online — off (no listen)");
        assert!(!crate::hex_protocol::default_client_listens());
    }

    #[test]
    fn default_os_dir_is_not_f_book() {
        let dir = os_user_data_dir();
        assert!(!is_f_book_fixture_dir(&dir));
        assert!(!dir.to_string_lossy().contains("tests/fixtures/f-book"));
    }

    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn with_user_dir<R>(dir: &Path, body: impl FnOnce() -> R + std::panic::UnwindSafe) -> R {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
        let prev = std::env::var(USER_DIR_OVERRIDE_ENV).ok();
        std::env::set_var(USER_DIR_OVERRIDE_ENV, dir);
        let result = std::panic::catch_unwind(body);
        match prev {
            Some(value) => std::env::set_var(USER_DIR_OVERRIDE_ENV, value),
            None => std::env::remove_var(USER_DIR_OVERRIDE_ENV),
        }
        match result {
            Ok(value) => value,
            Err(payload) => std::panic::resume_unwind(payload),
        }
    }

    #[test]
    fn torn_tmp_does_not_replace_good_live() {
        let dir = scratch("torn");
        let live = dir.join("powrush_note.json");
        let old = br#"{"gen":1,"body":"old-good"}"#;
        let partial = br#"{"gen":2,"body":"PAR"#;
        let new = br#"{"gen":2,"body":"new-good"}"#;
        write_last_good(&live, old).unwrap();
        let tmp = stage_tmp(&live, partial).unwrap();
        assert_eq!(fs::read(&live).unwrap(), old);
        assert_eq!(fs::read(&tmp).unwrap(), partial);
        assert_ne!(fs::read(&live).unwrap(), partial);
        write_last_good(&live, new).unwrap();
        let live_now = fs::read(&live).unwrap();
        assert_eq!(live_now, new);
        assert_ne!(live_now, partial);
        assert!(!sibling_path(&live, ".tmp").exists());
        assert!(
            serde_json::from_str::<serde_json::Value>(std::str::from_utf8(&live_now).unwrap())
                .is_ok()
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn bak_restore_read_named_falls_back_to_previous_good() {
        let dir = scratch("bak-read");
        fs::write(dir.join("powrush_seal.json"), b"{}\n").unwrap();
        let name = "powrush_note.json";
        let first = r#"{"gen":1,"body":"first-good"}"#;
        let second = r#"{"gen":2,"body":"second-good"}"#;
        with_user_dir(&dir, || {
            write_named(name, first).unwrap();
            write_named(name, second).unwrap();
            let bak = dir.join("powrush_note.json.bak");
            assert_eq!(fs::read_to_string(&bak).unwrap(), first);
            assert_eq!(read_named(name).unwrap(), second);
            fs::write(dir.join(name), b"").unwrap();
            assert_eq!(read_named(name).unwrap(), first);
            fs::write(dir.join(name), b"not-json").unwrap();
            assert_eq!(read_named(name).unwrap(), first);
            fs::remove_file(dir.join(name)).unwrap();
            assert_eq!(read_named(name).unwrap(), first);
        });
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn named_exists_follows_good_bak_when_live_file_is_absent() {
        let dir = scratch("named-exists-bak");
        // Live sibling so an empty user dir does not adopt cwd `data/`.
        fs::write(dir.join("powrush_seal.json"), b"{}\n").unwrap();
        let name = "powrush_hour_two.json";
        let good = r#"{"complete":true}"#;
        with_user_dir(&dir, || {
            let live = dir.join(name);
            let bak = dir.join("powrush_hour_two.json.bak");
            let tmp = dir.join("powrush_hour_two.json.tmp");

            assert!(!named_exists(name));

            fs::write(&bak, good).unwrap();
            assert!(named_exists(name));
            assert_eq!(read_named(name).unwrap(), good);

            fs::write(&bak, "not-json").unwrap();
            assert!(!named_exists(name));
            assert!(read_named(name).is_err());

            fs::remove_file(&bak).unwrap();
            fs::write(&tmp, good).unwrap();
            assert!(!live.exists());
            assert!(!named_exists(name));
            assert!(read_named(name).is_err());

            fs::remove_file(&tmp).unwrap();
            fs::write(&live, good).unwrap();
            assert!(named_exists(name));
            assert_eq!(read_named(name).unwrap(), good);

            // Corrupt live and no good bak: exists() would be true, read_named Err.
            fs::write(&live, "not-json").unwrap();
            assert!(live.exists());
            assert!(!named_exists(name));
            assert!(read_named(name).is_err());

            fs::write(&bak, good).unwrap();
            assert!(named_exists(name));
            assert_eq!(read_named(name).unwrap(), good);
        });
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn empty_live_is_not_copied_over_good_bak() {
        let dir = scratch("empty-live");
        let live = dir.join("powrush_note.json");
        let first = br#"{"gen":1}"#;
        let second = br#"{"gen":2}"#;
        let third = br#"{"gen":3}"#;
        write_last_good(&live, first).unwrap();
        write_last_good(&live, second).unwrap();
        let bak = sibling_path(&live, ".bak");
        assert_eq!(fs::read(&bak).unwrap(), first);
        fs::write(&live, b"").unwrap();
        write_last_good(&live, third).unwrap();
        assert_eq!(fs::read(&bak).unwrap(), first);
        assert_ne!(fs::read(&bak).unwrap(), b"");
        assert!(parses_as_json_file(&bak));
        assert_eq!(fs::read(&live).unwrap(), third);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn bak_or_tmp_only_cwd_is_not_counted_or_adopted() {
        let root = scratch("skip-sib");
        let cwd = root.join("cwd-data");
        let user = root.join("user");
        fs::create_dir_all(&cwd).unwrap();
        fs::create_dir_all(&user).unwrap();
        fs::write(cwd.join("powrush_house.json.bak"), r#"{"ok":1}"#).unwrap();
        fs::write(cwd.join("powrush_house.json.tmp"), r#"{"partial":true}"#).unwrap();
        fs::write(user.join("powrush_house.json.bak"), r#"{"kept":true}"#).unwrap();
        assert!(!persist_files_present(&cwd));
        assert!(!persist_files_present(&user));
        assert_eq!(maybe_adopt_cwd(&user, &cwd), AdoptKind::SkippedCwdEmpty);
        assert!(!user.join("powrush_house.json.tmp").exists());
        assert_eq!(
            fs::read_to_string(user.join("powrush_house.json.bak")).unwrap(),
            r#"{"kept":true}"#
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn jsonl_persist_is_counted_and_adopted() {
        let root = scratch("jsonl");
        let cwd = root.join("cwd-data");
        let user = root.join("user");
        fs::create_dir_all(&cwd).unwrap();
        fs::create_dir_all(&user).unwrap();
        let genshare = "{\"hex\":\"a\"}\n";
        let events = "{\"kind\":\"lived\"}\n";
        fs::write(cwd.join("powrush_genshare.jsonl"), genshare).unwrap();
        fs::write(cwd.join("powrush_lived_events.jsonl"), events).unwrap();
        fs::write(cwd.join("powrush_genshare.jsonl.bak"), "stale-bak").unwrap();
        fs::write(cwd.join("powrush_genshare.jsonl.tmp"), "partial-tmp").unwrap();
        assert!(persist_files_present(&cwd));
        assert_eq!(maybe_adopt_cwd(&user, &cwd), AdoptKind::Copied);
        assert_eq!(
            fs::read_to_string(user.join("powrush_genshare.jsonl")).unwrap(),
            genshare
        );
        assert_eq!(
            fs::read_to_string(user.join("powrush_lived_events.jsonl")).unwrap(),
            events
        );
        assert!(!user.join("powrush_genshare.jsonl.bak").exists());
        assert!(!user.join("powrush_genshare.jsonl.tmp").exists());
        let _ = fs::remove_dir_all(&root);
    }

    /// Injected faults check ordering and the bytes left on disk.
    /// They do not emulate power loss, and they do not prove that `sync_all`
    /// reached physical media. A same-directory rename is not treated as a
    /// broad platform durability proof. An error after rename is not undone.
    const PARTIAL_KEEP: usize = 8;
    const STALE_LIVE_TMP: &[u8] = b"STALE-LIVE-TMP-NOT-A-SAVE";
    const STALE_BAK_TMP: &[u8] = b"STALE-BAK-TMP-NOT-A-SAVE";

    #[derive(Clone, Copy)]
    enum FaultKind {
        /// `write_all` stores `keep` bytes, then returns `Err`.
        Partial { keep: usize },
        /// `write_all` finishes; `sync_all` returns `Err` and rename is not reached.
        Sync,
        /// `write_all` and `sync_all` finish; the next rename of this temp returns `Err`.
        BeforeRename,
    }

    thread_local! {
        static LIVE_FAULT: Cell<Option<FaultKind>> = Cell::new(None);
        static BAK_FAULT: Cell<Option<FaultKind>> = Cell::new(None);
        static PRE_RENAME: RefCell<Option<PathBuf>> = RefCell::new(None);
    }

    #[must_use]
    struct FaultGuard;

    impl Drop for FaultGuard {
        fn drop(&mut self) {
            clear_faults();
        }
    }

    fn clear_faults() {
        LIVE_FAULT.with(|slot| slot.set(None));
        BAK_FAULT.with(|slot| slot.set(None));
        PRE_RENAME.with(|slot| *slot.borrow_mut() = None);
    }

    fn install_live(kind: FaultKind) -> FaultGuard {
        clear_faults();
        LIVE_FAULT.with(|slot| slot.set(Some(kind)));
        FaultGuard
    }

    fn install_bak(kind: FaultKind) -> FaultGuard {
        clear_faults();
        BAK_FAULT.with(|slot| slot.set(Some(kind)));
        FaultGuard
    }

    fn take_fault(tmp: &Path) -> Option<FaultKind> {
        let name = tmp.file_name()?.to_string_lossy();
        if name.ends_with(".bak.tmp") {
            BAK_FAULT.with(|slot| slot.take())
        } else if name.ends_with(".tmp") {
            LIVE_FAULT.with(|slot| slot.take())
        } else {
            None
        }
    }

    struct FaultWriter {
        file: fs::File,
        kind: FaultKind,
        path: PathBuf,
    }

    impl FaultWriter {
        fn open(tmp: &Path, kind: FaultKind) -> std::io::Result<Self> {
            Ok(Self {
                file: create_truncating(tmp)?,
                kind,
                path: tmp.to_path_buf(),
            })
        }
    }

    impl SyncedWrite for FaultWriter {
        fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
            match self.kind {
                FaultKind::Partial { keep } => {
                    let n = keep.min(buf.len());
                    if n > 0 {
                        Write::write_all(&mut self.file, &buf[..n])?;
                    }
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "injected partial persist write",
                    ))
                }
                FaultKind::Sync | FaultKind::BeforeRename => Write::write_all(&mut self.file, buf),
            }
        }

        fn sync_all(&mut self) -> std::io::Result<()> {
            match self.kind {
                FaultKind::Sync => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "injected persist sync failure",
                )),
                FaultKind::BeforeRename => {
                    fs::File::sync_all(&self.file)?;
                    PRE_RENAME.with(|slot| *slot.borrow_mut() = Some(self.path.clone()));
                    Ok(())
                }
                FaultKind::Partial { .. } => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "injected partial persist write",
                )),
            }
        }
    }

    /// Run the installed writer, if this temp has one. `None` means the
    /// production `File` path must run. The fault type is test-only.
    pub(super) fn apply_installed_fault(tmp: &Path, bytes: &[u8]) -> Option<std::io::Result<()>> {
        let kind = take_fault(tmp)?;
        Some(run_fault_writer(tmp, bytes, kind))
    }

    fn run_fault_writer(tmp: &Path, bytes: &[u8], kind: FaultKind) -> std::io::Result<()> {
        let mut writer = FaultWriter::open(tmp, kind)?;
        write_all_and_sync(&mut writer, bytes)
    }

    pub(super) fn fail_installed_pre_rename(tmp: &Path) -> bool {
        PRE_RENAME.with(|slot| {
            let mut armed = slot.borrow_mut();
            match armed.as_ref() {
                Some(path) if path == tmp => {
                    *armed = None;
                    true
                }
                _ => false,
            }
        })
    }

    struct Seed {
        dir: PathBuf,
        live: PathBuf,
        older: Vec<u8>,
        current: Vec<u8>,
        new_bytes: Vec<u8>,
    }

    fn seed_good_pair(tag: &str) -> Seed {
        let dir = scratch(tag);
        let live = dir.join("powrush_note.json");
        let older = br#"{"gen":0,"body":"bak-good"}"#.to_vec();
        let current = br#"{"gen":1,"body":"live-good"}"#.to_vec();
        let new_bytes = br#"{"gen":2,"body":"new-good"}"#.to_vec();
        assert!(older.len() > PARTIAL_KEEP && current.len() > PARTIAL_KEEP);
        assert!(new_bytes.len() > PARTIAL_KEEP);
        assert_ne!(&older[..PARTIAL_KEEP], &current[..PARTIAL_KEEP]);
        assert_ne!(&current[..PARTIAL_KEEP], &new_bytes[..PARTIAL_KEEP]);
        write_last_good(&live, &older).unwrap();
        write_last_good(&live, &current).unwrap();
        assert_eq!(fs::read(&live).unwrap(), current);
        assert_eq!(fs::read(bak_dest_path(&live)).unwrap(), older);
        assert!(!live_stage_path(&live).exists());
        assert!(!bak_stage_path(&live).exists());
        Seed {
            dir,
            live,
            older,
            current,
            new_bytes,
        }
    }

    fn assert_injected(result: std::io::Result<()>) {
        let err = result.expect_err("fault must propagate");
        assert!(
            err.to_string().contains("injected"),
            "fault must be the injected error, got {err}"
        );
    }

    fn assert_live_bak(seed: &Seed, live_bytes: &[u8], bak_bytes: &[u8]) {
        assert_eq!(fs::read(&seed.live).unwrap(), live_bytes);
        assert_eq!(fs::read(bak_dest_path(&seed.live)).unwrap(), bak_bytes);
    }

    fn finish(seed: &Seed) {
        let _ = fs::remove_dir_all(&seed.dir);
    }

    #[test]
    fn stage_temp_paths_are_distinct_siblings() {
        let live = std::env::temp_dir()
            .join("powrush-stage-paths")
            .join("powrush_note.json");
        let tmp = live_stage_path(&live);
        let bak = bak_dest_path(&live);
        let bak_tmp = bak_stage_path(&live);
        assert_eq!(
            tmp.file_name().and_then(|s| s.to_str()),
            Some("powrush_note.json.tmp")
        );
        assert_eq!(
            bak.file_name().and_then(|s| s.to_str()),
            Some("powrush_note.json.bak")
        );
        assert_eq!(
            bak_tmp.file_name().and_then(|s| s.to_str()),
            Some("powrush_note.json.bak.tmp")
        );
        assert_eq!(tmp.parent(), live.parent());
        assert_eq!(bak.parent(), live.parent());
        assert_eq!(bak_tmp.parent(), live.parent());
        assert_ne!(tmp, bak_tmp);
        assert_ne!(tmp, live);
        assert_ne!(tmp, bak);
        assert_ne!(bak_tmp, live);
        assert_ne!(bak_tmp, bak);
        assert_ne!(bak, live);
        assert!(!is_live_persist_name("powrush_note.json.tmp"));
        assert!(!is_live_persist_name("powrush_note.json.bak"));
        assert!(!is_live_persist_name("powrush_note.json.bak.tmp"));
        assert!(is_live_persist_name("powrush_note.json"));
    }

    #[test]
    fn partial_live_write_preserves_live_and_old_bak() {
        let seed = seed_good_pair("partial-live");
        let _guard = install_live(FaultKind::Partial { keep: PARTIAL_KEEP });
        assert_injected(write_last_good(&seed.live, &seed.new_bytes));
        assert_live_bak(&seed, &seed.current, &seed.older);
        let staged = fs::read(live_stage_path(&seed.live)).unwrap();
        assert_eq!(staged, &seed.new_bytes[..PARTIAL_KEEP]);
        assert_ne!(fs::read(&seed.live).unwrap(), seed.new_bytes);
        assert_ne!(fs::read(bak_dest_path(&seed.live)).unwrap(), seed.new_bytes);
        assert!(!bak_stage_path(&seed.live).exists());
        finish(&seed);
    }

    #[test]
    fn stale_live_tmp_partial_write_preserves_dest() {
        let seed = seed_good_pair("stale-live-tmp");
        let tmp = live_stage_path(&seed.live);
        fs::write(&tmp, STALE_LIVE_TMP).unwrap();
        let _guard = install_live(FaultKind::Partial { keep: PARTIAL_KEEP });
        assert_injected(write_last_good(&seed.live, &seed.new_bytes));
        assert_live_bak(&seed, &seed.current, &seed.older);
        assert_ne!(fs::read(&seed.live).unwrap(), STALE_LIVE_TMP);
        assert_ne!(fs::read(bak_dest_path(&seed.live)).unwrap(), STALE_LIVE_TMP);
        let staged = fs::read(&tmp).unwrap();
        assert_eq!(staged, &seed.new_bytes[..PARTIAL_KEEP]);
        assert_ne!(staged, STALE_LIVE_TMP);
        assert_ne!(tmp, seed.live);
        finish(&seed);
    }

    #[test]
    fn live_sync_failure_before_rename_does_not_promote() {
        let seed = seed_good_pair("live-sync");
        let _guard = install_live(FaultKind::Sync);
        assert_injected(write_last_good(&seed.live, &seed.new_bytes));
        assert_live_bak(&seed, &seed.current, &seed.older);
        let tmp = live_stage_path(&seed.live);
        assert!(tmp.exists());
        assert_ne!(tmp, seed.live);
        assert_eq!(fs::read(&tmp).unwrap(), seed.new_bytes);
        assert_ne!(fs::read(&seed.live).unwrap(), seed.new_bytes);
        finish(&seed);
    }

    #[test]
    fn live_pre_rename_after_sync_does_not_promote() {
        let seed = seed_good_pair("live-pre-rename");
        let _guard = install_live(FaultKind::BeforeRename);
        assert_injected(write_last_good(&seed.live, &seed.new_bytes));
        // Backup of the still-current live commits before the live rename.
        // The blocked rename does not roll that backup back and does not
        // publish the new temp.
        assert_eq!(fs::read(&seed.live).unwrap(), seed.current);
        assert_eq!(fs::read(bak_dest_path(&seed.live)).unwrap(), seed.current);
        assert_ne!(fs::read(bak_dest_path(&seed.live)).unwrap(), seed.new_bytes);
        let tmp = live_stage_path(&seed.live);
        assert!(tmp.exists());
        assert_ne!(tmp, seed.live);
        assert_eq!(fs::read(&tmp).unwrap(), seed.new_bytes);
        assert!(!bak_stage_path(&seed.live).exists());
        finish(&seed);
    }

    #[test]
    fn backup_pre_rename_failure_preserves_old_bak_and_live() {
        let seed = seed_good_pair("bak-pre-rename");
        let _guard = install_bak(FaultKind::BeforeRename);
        assert_injected(write_last_good(&seed.live, &seed.new_bytes));
        assert_live_bak(&seed, &seed.current, &seed.older);
        let bak_tmp = bak_stage_path(&seed.live);
        let live_tmp = live_stage_path(&seed.live);
        assert_ne!(bak_tmp, live_tmp);
        assert_ne!(bak_tmp, seed.live);
        assert_ne!(bak_tmp, bak_dest_path(&seed.live));
        assert!(bak_tmp.exists());
        assert_eq!(fs::read(&bak_tmp).unwrap(), seed.current);
        assert_ne!(fs::read(&bak_tmp).unwrap(), seed.new_bytes);
        assert_eq!(fs::read(&live_tmp).unwrap(), seed.new_bytes);
        assert_ne!(fs::read(&seed.live).unwrap(), seed.new_bytes);
        finish(&seed);
    }

    #[test]
    fn backup_sync_failure_preserves_old_bak_and_live() {
        let seed = seed_good_pair("bak-sync");
        let _guard = install_bak(FaultKind::Sync);
        assert_injected(write_last_good(&seed.live, &seed.new_bytes));
        assert_live_bak(&seed, &seed.current, &seed.older);
        let bak_tmp = bak_stage_path(&seed.live);
        assert!(bak_tmp.exists());
        assert_eq!(fs::read(&bak_tmp).unwrap(), seed.current);
        assert_ne!(fs::read(&bak_tmp).unwrap(), seed.new_bytes);
        assert_ne!(fs::read(&seed.live).unwrap(), seed.new_bytes);
        assert_eq!(
            fs::read(live_stage_path(&seed.live)).unwrap(),
            seed.new_bytes
        );
        finish(&seed);
    }

    #[test]
    fn stale_bak_tmp_partial_write_preserves_old_bak_and_live() {
        let seed = seed_good_pair("stale-bak-tmp");
        let bak_tmp = bak_stage_path(&seed.live);
        fs::write(&bak_tmp, STALE_BAK_TMP).unwrap();
        let _guard = install_bak(FaultKind::Partial { keep: PARTIAL_KEEP });
        assert_injected(write_last_good(&seed.live, &seed.new_bytes));
        assert_live_bak(&seed, &seed.current, &seed.older);
        assert_ne!(fs::read(&seed.live).unwrap(), STALE_BAK_TMP);
        assert_ne!(fs::read(bak_dest_path(&seed.live)).unwrap(), STALE_BAK_TMP);
        let staged = fs::read(&bak_tmp).unwrap();
        assert_eq!(staged, &seed.current[..PARTIAL_KEEP]);
        assert_ne!(staged, &seed.new_bytes[..PARTIAL_KEEP]);
        assert_ne!(staged, STALE_BAK_TMP);
        assert_ne!(fs::read(&seed.live).unwrap(), seed.new_bytes);
        finish(&seed);
    }

    #[test]
    fn stale_stage_files_are_replaced_not_adopted() {
        let seed = seed_good_pair("stale-replace");
        fs::write(live_stage_path(&seed.live), STALE_LIVE_TMP).unwrap();
        fs::write(bak_stage_path(&seed.live), STALE_BAK_TMP).unwrap();
        write_last_good(&seed.live, &seed.new_bytes).unwrap();
        assert_eq!(fs::read(&seed.live).unwrap(), seed.new_bytes);
        assert_eq!(fs::read(bak_dest_path(&seed.live)).unwrap(), seed.current);
        assert_ne!(fs::read(&seed.live).unwrap(), STALE_LIVE_TMP);
        assert_ne!(fs::read(bak_dest_path(&seed.live)).unwrap(), STALE_BAK_TMP);
        assert!(!live_stage_path(&seed.live).exists());
        assert!(!bak_stage_path(&seed.live).exists());
        finish(&seed);
    }

    #[test]
    fn raw_sibling_reads_split_not_found_from_errors() {
        let dir = scratch("raw-split");
        fs::write(dir.join("powrush_seal.json"), b"{}\n").unwrap();
        let name = "powrush_raw_probe.json";
        with_user_dir(&dir, || {
            assert_eq!(read_live_bytes(name).unwrap(), RawPersist::NotFound);
            assert_eq!(read_backup_bytes(name).unwrap(), RawPersist::NotFound);
            fs::write(dir.join(name), b"live-raw").unwrap();
            fs::write(dir.join(format!("{name}.bak")), b"bak-raw").unwrap();
            assert_eq!(
                read_live_bytes(name).unwrap(),
                RawPersist::Bytes(b"live-raw".to_vec())
            );
            assert_eq!(
                read_backup_bytes(name).unwrap(),
                RawPersist::Bytes(b"bak-raw".to_vec())
            );
            fs::remove_file(dir.join(name)).unwrap();
            fs::create_dir(dir.join(name)).unwrap();
            assert!(read_live_bytes(name).is_err());
            assert_eq!(
                read_backup_bytes(name).unwrap(),
                RawPersist::Bytes(b"bak-raw".to_vec())
            );
        });
        let _ = fs::remove_dir_all(&dir);
    }
}
