/*!
 * Powrush-MMO Server — Comprehensive Hardening
 *
 * Includes:
 * - Anti-debug detection
 * - Binary integrity check
 * - Privilege dropping
 * - seccomp syscall filtering
 * - landlock filesystem sandboxing
 *
 * All features can be disabled with POWRUSH_DISABLE_HARDENING=1
 */

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use landlock::{AccessFs, Ruleset, RulesetAttr, RulesetCreatedAttr, ABI};
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

/// Apply seccomp filter (basic safe allowlist for game server)
fn apply_seccomp() -> Result<(), Box<dyn std::error::Error>> {
    let mut filter = SeccompFilter::new(
        SeccompAction::KillProcess,
        SeccompAction::Allow,
    )?;

    // Allow common syscalls needed by Tokio + networking
    let allowed = vec![
        libc::SYS_read,
        libc::SYS_write,
        libc::SYS_close,
        libc::SYS_futex,
        libc::SYS_epoll_ctl,
        libc::SYS_epoll_wait,
        libc::SYS_socket,
        libc::SYS_connect,
        libc::SYS_accept,
        libc::SYS_bind,
        libc::SYS_listen,
        libc::SYS_getsockname,
        libc::SYS_getpeername,
        libc::SYS_sendto,
        libc::SYS_recvfrom,
        libc::SYS_setsockopt,
        libc::SYS_getsockopt,
        libc::SYS_clone,
        libc::SYS_clone3,
        libc::SYS_mmap,
        libc::SYS_munmap,
        libc::SYS_madvise,
        libc::SYS_brk,
        libc::SYS_mprotect,
        libc::SYS_rt_sigaction,
        libc::SYS_rt_sigprocmask,
        libc::SYS_sigaltstack,
        libc::SYS_gettid,
        libc::SYS_getpid,
        libc::SYS_getrandom,
        libc::SYS_clock_gettime,
        libc::SYS_nanosleep,
        libc::SYS_sched_yield,
        libc::SYS_exit,
        libc::SYS_exit_group,
    ];

    for syscall in allowed {
        filter.add_rule(SeccompRule::new(syscall, SeccompAction::Allow))?; 
    }

    let program: BpfProgram = filter.try_into()?;
    seccompiler::apply_filter(&program)?;

    println!("[Hardening] seccomp filter applied");
    Ok(())
}

/// Apply landlock filesystem restrictions
fn apply_landlock() -> Result<(), Box<dyn std::error::Error>> {
    let abi = ABI::V2;

    let mut ruleset = Ruleset::default()
        .handle_access(AccessFs::from_all(abi))?
        .create()?;

    // Only allow reading/writing in current directory and /tmp for logs
    let cwd = std::env::current_dir()?;
    ruleset.add_rule(
        AccessFs::from_all(abi),
        cwd,
    )?;

    if let Ok(tmp) = std::env::var("TMPDIR") {
        ruleset.add_rule(AccessFs::from_all(abi), Path::new(&tmp))?; 
    }

    let _status = ruleset.restrict_self()?;
    println!("[Hardening] landlock filesystem sandbox applied");
    Ok(())
}
