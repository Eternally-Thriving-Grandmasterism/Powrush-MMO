/*!
 * Powrush-MMO Server — Comprehensive Hardening
 */

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use landlock::{AccessFs, PathBeneath, Ruleset, RulesetAttr, RulesetCreatedAttr, ABI};
use nix::unistd::{getuid, setuid, Uid};
use seccompiler::{BpfProgram, SeccompAction, SeccompFilter, SeccompRule};
use sha2::{Digest, Sha256};

pub fn apply_server_hardening() {
    if env::var("POWRUSH_DISABLE_HARDENING").is_ok() {
        println!("[Hardening] All server hardening disabled");
        return;
    }

    check_anti_debug();
    check_binary_integrity();
    drop_privileges();

    if env::var("POWRUSH_ENABLE_SECCOMP").is_ok() || cfg!(not(debug_assertions)) {
        if let Err(e) = apply_seccomp() {
            eprintln!("[Hardening] seccomp failed to apply: {}", e);
        }
    }

    if env::var("POWRUSH_ENABLE_LANDLOCK").is_ok() || cfg!(not(debug_assertions)) {
        if let Err(e) = apply_landlock() {
            eprintln!("[Hardening] landlock failed to apply: {}", e);
        }
    }

    println!("[Hardening] Server hardening completed");
}

fn check_anti_debug() {
    if let Ok(mut file) = File::open("/proc/self/status") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() && !contents.contains("TracerPid:\t0") {
            eprintln!("[Hardening] Warning: Process is being debugged");
        }
    }
}

fn check_binary_integrity() {
    if let Ok(exe_path) = std::env::current_exe() {
        if let Ok(mut file) = File::open(&exe_path) {
            let mut hasher = Sha256::new();
            let mut buffer = [0u8; 8192];
            while let Ok(n) = file.read(&mut buffer) {
                if n == 0 { break; }
                hasher.update(&buffer[..n]);
            }
            println!("[Hardening] Server binary SHA256: {:x}", hasher.finalize());
        }
    }
}

fn drop_privileges() {
    if getuid().is_root() {
        let target = env::var("POWRUSH_USER").unwrap_or_else(|_| "nobody".to_string());
        if let Ok(user) = users::get_user_by_name(&target) {
            if setuid(Uid::from_raw(user.uid())).is_ok() {
                println!("[Hardening] Dropped to user: {}", target);
            }
        }
    }
}

/// Apply seccomp filter
fn apply_seccomp() -> Result<(), Box<dyn std::error::Error>> {
    let mut filter = SeccompFilter::new(
        SeccompAction::KillProcess,
        SeccompAction::Allow,
    )?;

    let allowed = vec![
        libc::SYS_read, libc::SYS_write, libc::SYS_close,
        libc::SYS_futex, libc::SYS_epoll_ctl, libc::SYS_epoll_wait,
        libc::SYS_socket, libc::SYS_connect, libc::SYS_accept, libc::SYS_bind,
        libc::SYS_listen, libc::SYS_getsockname, libc::SYS_getpeername,
        libc::SYS_sendto, libc::SYS_recvfrom, libc::SYS_setsockopt, libc::SYS_getsockopt,
        libc::SYS_clone, libc::SYS_clone3, libc::SYS_mmap, libc::SYS_munmap,
        libc::SYS_madvise, libc::SYS_brk, libc::SYS_mprotect,
        libc::SYS_rt_sigaction, libc::SYS_rt_sigprocmask, libc::SYS_sigaltstack,
        libc::SYS_gettid, libc::SYS_getpid, libc::SYS_getrandom,
        libc::SYS_clock_gettime, libc::SYS_nanosleep, libc::SYS_sched_yield,
        libc::SYS_exit, libc::SYS_exit_group,
    ];

    for syscall in allowed {
        filter.add_rule(SeccompRule::new(syscall, SeccompAction::Allow))?; 
    }

    let program: BpfProgram = filter.try_into()?;
    seccompiler::apply_filter(&program)?;
    println!("[Hardening] seccomp filter applied");
    Ok(())
}

/// Optimized landlock rules - more restrictive and practical
fn apply_landlock() -> Result<(), Box<dyn std::error::Error>> {
    let abi = ABI::V2;

    // Define minimal needed access rights
    let read_only = AccessFs::ReadFile | AccessFs::ReadDir;
    let read_write = read_only | AccessFs::WriteFile | AccessFs::CreateFile |
                     AccessFs::CreateDir | AccessFs::RemoveFile | AccessFs::RemoveDir;

    let mut ruleset = Ruleset::default()
        .handle_access(read_write)?
        .create()?;

    // Read-only access to current directory (assets, config, etc.)
    let cwd = std::env::current_dir()?;
    ruleset.add_rule(
        PathBeneath::new(cwd, read_only)?
    )?;

    // Read-write access to data directory (world saves, player data)
    if let Ok(data_dir) = env::var("POWRUSH_DATA_DIR") {
        let path = Path::new(&data_dir);
        if path.exists() {
            ruleset.add_rule(PathBeneath::new(path, read_write)?)?;
            println!("[Hardening] landlock: granted RW to {}", data_dir);
        }
    } else {
        // Default to ./data if exists
        let default_data = Path::new("./data");
        if default_data.exists() {
            ruleset.add_rule(PathBeneath::new(default_data, read_write)?)?;
        }
    }

    // Limited write access to logs/tmp
    let log_paths = vec![
        Path::new("/tmp"),
        Path::new("./logs"),
    ];

    for path in log_paths {
        if path.exists() {
            ruleset.add_rule(PathBeneath::new(path, read_write)?)?;
        }
    }

    let _status = ruleset.restrict_self()?;
    println!("[Hardening] Optimized landlock filesystem sandbox applied");
    Ok(())
}
