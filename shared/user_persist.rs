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

/// Soft-write a persist file into the resolved user dir (creates the dir).
///
/// A good save parses as JSON (`serde_json::from_str::<serde_json::Value>`).
/// This module does not parse a typed schema. The live file is copied to a
/// sibling `.bak` only when it is a good save. Bytes then go to a sibling
/// `.tmp`, and `fs::rename` moves that file into place.
pub fn write_named(named: &str, contents: impl AsRef<[u8]>) -> std::io::Result<()> {
    write_last_good(&persist_path(named), contents.as_ref())
}

/// Whether the named persist file exists in the resolved user dir.
pub fn named_exists(named: &str) -> bool {
    persist_path(named).exists()
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

/// Copy live → `.bak` when the live file parses, then write `contents` to
/// the sibling `.tmp`. Does not rename, so a stop here leaves the live file
/// as it was.
fn stage_tmp(path: &Path, contents: &[u8]) -> std::io::Result<PathBuf> {
    ensure_parent(path)?;
    if parses_as_json_file(path) {
        fs::copy(path, sibling_path(path, ".bak"))?;
    }
    let tmp = sibling_path(path, ".tmp");
    fs::write(&tmp, contents)?;
    Ok(tmp)
}

/// `.tmp` then rename. Copy live → `.bak` only when the live file parses as JSON.
fn write_last_good(path: &Path, contents: &[u8]) -> std::io::Result<()> {
    let tmp = stage_tmp(path, contents)?;
    fs::rename(&tmp, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
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
        assert_eq!(persist_file_name("powrush_settings.json"), "powrush_settings.json");
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
        fs::write(cwd.join("powrush_house.json"), r#"{"schema":"powrush_house_v1"}"#).unwrap();
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
        assert!(serde_json::from_str::<serde_json::Value>(
            std::str::from_utf8(&live_now).unwrap()
        )
        .is_ok());
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
}
