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
pub fn read_named(named: &str) -> std::io::Result<String> {
    fs::read_to_string(persist_path(named))
}

/// Soft-write a persist file into the resolved user dir (creates the dir).
pub fn write_named(named: &str, contents: impl AsRef<[u8]>) -> std::io::Result<()> {
    let path = persist_path(named);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)
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

/// True when `dir` already holds at least one `powrush_*` persist file.
pub fn persist_files_present(dir: &Path) -> bool {
    let Ok(rd) = fs::read_dir(dir) else {
        return false;
    };
    rd.flatten().any(|e| {
        let name = e.file_name();
        let s = name.to_string_lossy();
        s.starts_with("powrush_") && e.path().is_file()
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdoptKind {
    SkippedUserHasFiles,
    SkippedCwdEmpty,
    Copied,
}

/// Copy `powrush_*` from `cwd_data` into `user_dir` only when user dir is empty.
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
            if !s.starts_with("powrush_") || !entry.path().is_file() {
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
}
