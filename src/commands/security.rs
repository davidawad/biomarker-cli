//! `audit log` and `doctor`.

use serde_json::json;

use crate::cli::AuditCmd;
use crate::context::Ctx;
use crate::crypto::{self, KekKind};
use crate::error::Result;
use crate::keys;
use crate::output::{to_record, Report};
use crate::perms::{self, Access};
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

type Row = crate::output::Record;

fn database_row(state: &State, path: &std::path::Path, insecure: bool) -> Row {
    match state {
        State::Missing => {
            check("database", "warn", format!("{} does not exist yet (created encrypted on first use)", path.display()))
        }
        State::Sealed(_) => {
            check("database", "ok", format!("{} is encrypted (XChaCha20-Poly1305 sealed container)", path.display()))
        }
        State::Plain if insecure => {
            check("database", "warn", format!("{} is NOT encrypted (--insecure-plaintext)", path.display()))
        }
        State::Plain => {
            check("database", "fail", format!("{} is NOT encrypted; run `biomarker db encrypt`", path.display()))
        }
    }
}

fn key_rows(ctx: &Ctx, source: keys::Source) -> Vec<Row> {
    let backend = keys::keychain_backend();
    let keychain = match keys::keychain_status() {
        Ok(()) => check("keychain", "ok", format!("{backend} available")),
        Err(e) => check(
            "keychain",
            "warn",
            format!("{backend} unavailable: {e}; falling back to {} / passphrase prompt", keys::ENV_KEY),
        ),
    };
    let env_key = match keys::parse_env_key(keys::ENV_KEY) {
        Ok(Some(keys::EnvKey::Raw(_))) => check("env_key", "ok", format!("{} is set (raw 256-bit key)", keys::ENV_KEY)),
        Ok(Some(keys::EnvKey::Passphrase(_))) => {
            check("env_key", "ok", format!("{} is set (passphrase)", keys::ENV_KEY))
        }
        Ok(None) => check("env_key", "ok", format!("{} not set", keys::ENV_KEY)),
        Err(e) => check("env_key", "fail", e.message),
    };
    vec![
        check(
            "key_source",
            "ok",
            format!("{} (setting key_source, source: {})", source.as_str(), ctx.resolved.source("key_source").as_str()),
        ),
        keychain,
        env_key,
    ]
}

fn kek_detail(kind: &KekKind) -> String {
    match kind {
        KekKind::Passphrase(p) => format!(" (Argon2id m={}KiB t={} p={})", p.m_cost, p.t_cost, p.p_cost),
        KekKind::Raw => String::new(),
    }
}

/// Container, keychain, session, unlock and audit-chain rows for a sealed database.
fn sealed_rows(ctx: &Ctx, path: &std::path::Path, source: keys::Source, h: &crypto::Header) -> Vec<Row> {
    let mut rows = vec![
        check(
            "container",
            "ok",
            format!("id {}, key wrapping: {}{}", crypto::hex(&h.db_id), h.kek_kind.as_str(), kek_detail(&h.kek_kind)),
        ),
        check(
            "keychain_entry",
            "ok",
            if keys::keychain_has(&h.db_id) { "key stored in OS keychain" } else { "no keychain entry" },
        ),
        match keys::session::get_with_expiry(&h.db_id) {
            Some((_, exp)) => {
                check("session", "ok", format!("unlocked (db unlock) until unix time {exp}; `db lock` ends it"))
            }
            None => check("session", "ok", "locked (no db unlock session)"),
        },
    ];
    match vault::unlock(path, source, false) {
        Ok((h, u)) => {
            rows.push(check("unlock", "ok", format!("unlocks with key from {}", u.source)));
            ctx.set_audit(h.db_id, u.keys.audit);
            let sink = ctx.audit_sink().expect("just set");
            rows.push(match sink.read() {
                Ok(r) => {
                    check("audit_log", "ok", format!("{} entries, chain verified ({})", r.len(), sink.path().display()))
                }
                Err(e) => check("audit_log", "fail", e.message),
            });
        }
        Err(e) => rows.push(check("unlock", "fail", format!("{} (interactive passphrase not tried)", e.message))),
    }
    rows
}

/// Permission rows (Unix mode or Windows ACL) for the database, its audit
/// log and its directory, and plaintext sidecar detection.
fn file_rows(state: &State, path: &std::path::Path) -> Vec<Row> {
    let mut rows = Vec::new();
    if !matches!(state, State::Missing) {
        let dir = path.parent().filter(|d| !d.as_os_str().is_empty()).map(std::path::Path::to_path_buf);
        for p in [Some(path.to_path_buf()), Some(crate::audit::path_for(path)), dir].into_iter().flatten() {
            match perms::inspect(&p) {
                Some(Access::Shared(d)) => rows.push(check("permissions", "warn", format!("{} is {d}", p.display()))),
                Some(Access::Private(d)) => rows.push(check("permissions", "ok", format!("{} is {d}", p.display()))),
                None => {}
            }
        }
    }
    if matches!(state, State::Sealed(_)) {
        let stray: Vec<String> = ["-wal", "-shm", "-journal", ".plaintext-wipe", ".encrypting"]
            .iter()
            .map(|s| crypto::sidecar(path, s))
            .filter(|p| p.exists())
            .map(|p| p.display().to_string())
            .collect();
        rows.push(if stray.is_empty() {
            check("sidecars", "ok", "no plaintext journal/WAL sidecars")
        } else {
            check("sidecars", "fail", format!("possible plaintext left on disk: {}", stray.join(", ")))
        });
    }
    rows
}

pub fn doctor(ctx: &Ctx) -> Result<()> {
    let path = &ctx.db_path;
    let source = ctx.key_source()?;
    let state = vault::state(path)?;
    let mut rows = vec![database_row(&state, path, ctx.insecure)];
    rows.extend(key_rows(ctx, source));
    if let State::Sealed(h) = &state {
        rows.extend(sealed_rows(ctx, path, source, h));
    }
    rows.extend(file_rows(&state, path));
    ctx.emit(&Report::list("doctor", rows).table_columns(&["check", "status", "detail"]))
}
