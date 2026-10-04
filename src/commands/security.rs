//! `audit log` and `doctor`.

use serde_json::json;

use crate::cli::AuditCmd;
use crate::context::Ctx;
use crate::crypto::{self, KekKind};
use crate::error::Result;
use crate::keys;
use crate::output::{to_record, Report};
use crate::vault::{self, State};

pub fn audit(ctx: &Ctx, cmd: AuditCmd) -> Result<()> {
    match cmd {
        AuditCmd::Log { limit } => {
            let (h, u) = vault::unlock(&ctx.db_path, ctx.key_source()?, true)?;
            ctx.set_audit(h.db_id, u.keys.audit);
            let sink = ctx.audit_sink().expect("just set");
            let recs = sink.read()?;
            let skip = limit.map_or(0, |n| recs.len().saturating_sub(n));
            let rows = recs.iter().skip(skip).map(to_record).collect();
            ctx.emit(
                &Report::list("audit_log", rows)
                    .table_columns(&["ts", "user", "command", "ok", "rows_changed", "rows_out", "output"])
                    .meta("verified", json!(true))
                    .meta("total", json!(recs.len())),
            )
        }
    }
}

fn check(name: &str, status: &str, detail: impl Into<String>) -> crate::output::Record {
    to_record(&json!({"check": name, "status": status, "detail": detail.into()}))
}

#[cfg(unix)]
fn mode(p: &std::path::Path) -> Option<u32> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(p).ok().map(|m| m.permissions().mode() & 0o777)
}
#[cfg(not(unix))]
fn mode(_: &std::path::Path) -> Option<u32> {
    None
}

pub fn doctor(ctx: &Ctx) -> Result<()> {
    let path = &ctx.db_path;
    let source = ctx.key_source()?;
    let mut rows = Vec::new();
    let state = vault::state(path)?;
    rows.push(match &state {
        State::Missing => {
            check("database", "warn", format!("{} does not exist yet (created encrypted on first use)", path.display()))
        }
        State::Sealed(_) => {
            check("database", "ok", format!("{} is encrypted (XChaCha20-Poly1305 sealed container)", path.display()))
        }
        State::Plain if ctx.insecure => {
            check("database", "warn", format!("{} is NOT encrypted (--insecure-plaintext)", path.display()))
        }
        State::Plain => {
            check("database", "fail", format!("{} is NOT encrypted; run `biomarker db encrypt`", path.display()))
        }
    });
    rows.push(check(
        "key_source",
        "ok",
        format!("{} (setting key_source, source: {})", source.as_str(), ctx.resolved.source("key_source").as_str()),
    ));
    rows.push(match keys::keychain_status() {
        Ok(()) => check("keychain", "ok", "OS keychain available"),
        Err(e) => check("keychain", "warn", format!("unavailable: {e}")),
    });
    rows.push(match keys::parse_env_key(keys::ENV_KEY) {
        Ok(Some(keys::EnvKey::Raw(_))) => check("env_key", "ok", format!("{} is set (raw 256-bit key)", keys::ENV_KEY)),
        Ok(Some(keys::EnvKey::Passphrase(_))) => {
            check("env_key", "ok", format!("{} is set (passphrase)", keys::ENV_KEY))
        }
        Ok(None) => check("env_key", "ok", format!("{} not set", keys::ENV_KEY)),
        Err(e) => check("env_key", "fail", e.message),
    });
    if let State::Sealed(h) = &state {
        rows.push(check(
            "container",
            "ok",
            format!(
                "id {}, key wrapping: {}{}",
                crypto::hex(&h.db_id),
                h.kek_kind.as_str(),
                match h.kek_kind {
                    KekKind::Passphrase(p) => format!(" (Argon2id m={}KiB t={} p={})", p.m_cost, p.t_cost, p.p_cost),
                    KekKind::Raw => String::new(),
                }
            ),
        ));
        rows.push(check(
            "keychain_entry",
            "ok",
            if keys::keychain_has(&h.db_id) { "key stored in OS keychain" } else { "no keychain entry" },
        ));
        rows.push(match keys::session::get_with_expiry(&h.db_id) {
            Some((_, exp)) => {
                check("session", "ok", format!("unlocked (db unlock) until unix time {exp}; `db lock` ends it"))
            }
            None => check("session", "ok", "locked (no db unlock session)"),
        });
        match vault::unlock(path, source, false) {
            Ok((h, u)) => {
                rows.push(check("unlock", "ok", format!("unlocks with key from {}", u.source)));
                ctx.set_audit(h.db_id, u.keys.audit);
                let sink = ctx.audit_sink().expect("just set");
                rows.push(match sink.read() {
                    Ok(r) => check(
                        "audit_log",
                        "ok",
                        format!("{} entries, chain verified ({})", r.len(), sink.path().display()),
                    ),
                    Err(e) => check("audit_log", "fail", e.message),
                });
            }
            Err(e) => rows.push(check("unlock", "fail", format!("{} (interactive passphrase not tried)", e.message))),
        }
    }
    if !matches!(state, State::Missing) {
        for p in [path.clone(), crate::audit::path_for(path)] {
            match mode(&p) {
                Some(m) if m & 0o077 != 0 => {
                    rows.push(check("permissions", "warn", format!("{} is mode {m:o}; expected 600", p.display())))
                }
                Some(m) => rows.push(check("permissions", "ok", format!("{} is mode {m:o}", p.display()))),
                None => {}
            }
        }
    }
    let stray: Vec<String> = ["-wal", "-shm", "-journal", ".plaintext-wipe", ".encrypting"]
        .iter()
        .map(|s| crypto::sidecar(path, s))
        .filter(|p| p.exists())
        .map(|p| p.display().to_string())
        .collect();
    if matches!(state, State::Sealed(_)) {
        rows.push(if stray.is_empty() {
            check("sidecars", "ok", "no plaintext journal/WAL sidecars")
        } else {
            check("sidecars", "fail", format!("possible plaintext left on disk: {}", stray.join(", ")))
        });
    }
    ctx.emit(&Report::list("doctor", rows).table_columns(&["check", "status", "detail"]))
}
