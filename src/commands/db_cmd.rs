use serde_json::json;

use crate::cli::DbCmd;
use crate::context::Ctx;
use crate::db::RowExt;
use crate::error::{AppError, Result};
use crate::keys::{self, Source};
use crate::migrations;
use crate::output::{to_record, Report};
use crate::store::Catalog;
use crate::vault;

pub fn run(ctx: &Ctx, cmd: DbCmd) -> Result<()> {
    match cmd {
        DbCmd::Path => ctx.emit(&Report::object(
            "db_path",
            to_record(&json!({"path": ctx.db_path, "exists": ctx.db_path.exists(), "source": ctx.resolved.source("db_path").as_str()})),
        )),
        DbCmd::Init { encrypt } => {
            if encrypt && ctx.insecure {
                return Err(AppError::usage("--encrypt and --insecure-plaintext are mutually exclusive"));
            }
            let existed = ctx.db_path.exists();
            let db = ctx.db_raw()?;
            let applied = migrations::migrate(&db)?;
            ctx.info(&format!("{} {}", if existed { "database ready:" } else { "created database" }, ctx.db_path.display()));
            ctx.emit(&Report::object(
                "db_init",
                to_record(&json!({
                    "path": ctx.db_path,
                    "created": !existed,
                    "applied": applied,
                    "version": migrations::current_version(&db)?,
                    "encrypted": db.sealed().is_some(),
                    "key_source": db.sealed().map(|s| s.key_source),
                })),
            ))
        }
        DbCmd::Encrypt => encrypt(ctx),
        DbCmd::Rekey { to, rotate_dek } => rekey(ctx, to.as_deref(), rotate_dek),
        DbCmd::Unlock { ttl } => unlock(ctx, &ttl),
        DbCmd::Lock => lock(ctx),
        DbCmd::Migrate { status } => migrate(ctx, status),
        DbCmd::Backup { dest } => {
            if dest.exists() {
                return Err(AppError::invalid(format!("{} already exists", dest.display())));
            }
            let db = ctx.db()?;
            db.backup_to(&dest)?;
            let size = std::fs::metadata(&dest).map(|m| m.len()).unwrap_or(0);
            let encrypted = db.sealed().is_some();
            ctx.info(&format!("backed up to {}{}", dest.display(), if encrypted { " (encrypted, same key)" } else { "" }));
            ctx.emit(&Report::object(
                "db_backup",
                to_record(&json!({"source": ctx.db_path, "dest": dest, "bytes": size, "encrypted": encrypted})),
            ))
        }
        DbCmd::Vacuum => {
            let db = ctx.db()?;
            db.execute("VACUUM", &[])?;
            ctx.info("vacuum complete");
            ctx.emit(&Report::object("db_vacuum", to_record(&json!({"path": ctx.db_path, "ok": true}))))
        }
        DbCmd::Check => check(ctx),
        DbCmd::Info => info(ctx),
    }
}

fn migrate(ctx: &Ctx, status_only: bool) -> Result<()> {
    let db = ctx.db_raw()?;
    let applied_now = if status_only { vec![] } else { migrations::migrate(&db)? };
    let applied = migrations::applied(&db)?;
    let rows = migrations::MIGRATIONS
        .iter()
        .map(|m| {
            let a = applied.iter().find(|a| a.0 == m.version);
            to_record(&json!({
                "version": m.version,
                "name": m.name,
                "applied": a.is_some(),
                "applied_at": a.map(|a| a.2.clone()),
                "applied_now": applied_now.contains(&m.version),
            }))
        })
        .collect();
    ctx.emit(&Report::list("migrations", rows).meta("current_version", json!(migrations::current_version(&db)?)))
}

fn check(ctx: &Ctx) -> Result<()> {
    let db = ctx.db()?;
    let integrity: Vec<String> = db.query("PRAGMA integrity_check", &[])?.iter().filter_map(|r| r.s(0)).collect();
    let orphans = db.query_scalar_i64(
        "SELECT count(*) FROM measurements m LEFT JOIN people p ON p.id = m.person_id LEFT JOIN markers k ON k.id = m.marker_id WHERE p.id IS NULL OR k.id IS NULL",
        &[],
    )?;
    let cat = Catalog::load(&db)?;
    let rows = db.query("SELECT id, marker_id, value_raw, unit_raw, value FROM measurements", &[])?;
    let inconsistent: Vec<i64> = rows
        .iter()
        .filter(|r| {
            let expected = cat
                .by_id(r.i(1).unwrap_or_default())
                .and_then(|m| cat.to_canonical(m, r.f(2).unwrap_or_default(), &r.s(3).unwrap_or_default()).ok());
            expected.is_none_or(|e| (e - r.f(4).unwrap_or_default()).abs() > 1e-6 * e.abs().max(1.0))
        })
        .filter_map(|r| r.i(0))
        .collect();
    let version = migrations::current_version(&db)?;
    let ok = integrity == ["ok"] && orphans == 0 && inconsistent.is_empty() && version == migrations::latest_version();
    let report = Report::object(
        "db_check",
        to_record(&json!({
            "ok": ok,
            "integrity": integrity,
            "orphan_measurements": orphans,
            "inconsistent_canonical_values": inconsistent,
            "schema_version": version,
            "expected_version": migrations::latest_version(),
        })),
    );
    ctx.emit(&report)?;
    if ok {
        Ok(())
    } else {
        Err(AppError::db("database check failed"))
    }
}

fn info(ctx: &Ctx) -> Result<()> {
    let db = ctx.db()?;
    let count = |t: &str| db.query_scalar_i64(&format!("SELECT count(*) FROM {t}"), &[]);
    let size = std::fs::metadata(&ctx.db_path).map(|m| m.len()).unwrap_or(0);
    ctx.emit(&Report::object(
        "db_info",
        to_record(&json!({
            "path": ctx.db_path,
            "bytes": size,
            "encrypted": db.sealed().is_some(),
            "key_source": db.sealed().map(|s| s.key_source),
            "schema_version": migrations::current_version(&db)?,
            "people": count("people")?,
            "markers": count("markers")?,
            "measurements": count("measurements")?,
            "ranges": count("ranges")?,
            "conversions": count("unit_conversions")?,
            "import_batches": count("import_batches")?,
        })),
    ))
}

fn encrypt(ctx: &Ctx) -> Result<()> {
    let r = vault::encrypt_in_place(&ctx.db_path, ctx.key_source()?)?;
    ctx.info(&format!(
        "encrypted {} (key from {}); plaintext overwritten and removed",
        ctx.db_path.display(),
        r.key_source
    ));
    if let Ok((h, u)) = vault::unlock(&ctx.db_path, ctx.key_source()?, false) {
        ctx.set_audit(h.db_id, u.keys.audit);
    }
    let tables: serde_json::Map<String, serde_json::Value> =
        r.tables.iter().map(|(t, n)| (t.clone(), json!(n))).collect();
    ctx.emit_mutation(&Report::object(
        "db_encrypt",
        to_record(&json!({
            "path": ctx.db_path,
            "bytes": r.bytes,
            "key_source": r.key_source,
            "verified_rows": tables,
            "wiped": r.wiped,
        })),
    ))
}

fn rekey(ctx: &Ctx, to: Option<&str>, rotate_dek: bool) -> Result<()> {
    let current = ctx.key_source()?;
    let to = to.map_or(Ok(current), Source::parse)?;
    let r = vault::rekey(&ctx.db_path, current, to, rotate_dek)?;
    ctx.set_audit(r.db_id, r.audit_key.clone());
    ctx.info(&format!("rekeyed {} ({} -> {})", ctx.db_path.display(), r.from, r.to));
    if r.to == "env" {
        ctx.info(&format!("set {} to the value of {} from now on", keys::ENV_KEY, keys::ENV_NEW_KEY));
    }
    ctx.emit_mutation(&Report::object(
        "db_rekey",
        to_record(&json!({"path": ctx.db_path, "from": r.from, "to": r.to, "rotated_dek": r.rotated_dek})),
    ))
}

/// `15m`, `2h`, `1d`, `90s` (bare numbers are minutes).
fn parse_ttl(s: &str) -> Result<u64> {
    let t = s.trim();
    let (num, mult) = match t.char_indices().last() {
        Some((i, 's')) => (&t[..i], 1),
        Some((i, 'm')) => (&t[..i], 60),
        Some((i, 'h')) => (&t[..i], 3600),
        Some((i, 'd')) => (&t[..i], 86400),
        _ => (t, 60),
    };
    num.trim()
        .parse::<u64>()
        .ok()
        .filter(|n| *n > 0)
        .map(|n| n.saturating_mul(mult))
        .ok_or_else(|| AppError::usage(format!("invalid --ttl '{s}' (e.g. 15m, 2h)")))
}

fn unlock(ctx: &Ctx, ttl: &str) -> Result<()> {
    let secs = parse_ttl(ttl)?;
    let (h, u) = vault::unlock(&ctx.db_path, ctx.key_source()?, true)?;
    keys::keychain_status().map_err(|e| keys::key_error(format!("db unlock needs the OS keychain: {e}")))?;
    let expires = keys::session::put(&h.db_id, &u.kek, secs)?;
    ctx.set_audit(h.db_id, u.keys.audit);
    ctx.info(&format!("unlocked {} for {}", ctx.db_path.display(), ttl));
    ctx.emit_mutation(&Report::object(
        "db_unlock",
        to_record(&json!({"path": ctx.db_path, "key_source": u.source, "expires_at_unix": expires})),
    ))
}

fn lock(ctx: &Ctx) -> Result<()> {
    let h = match vault::state(&ctx.db_path)? {
        vault::State::Sealed(h) => h,
        _ => return Err(AppError::invalid(format!("{} is not an encrypted database", ctx.db_path.display()))),
    };
    let had = keys::session::clear(&h.db_id)?;
    ctx.info(&if had { format!("locked {}", ctx.db_path.display()) } else { "no unlock session was active".into() });
    if keys::keychain_has(&h.db_id) {
        ctx.info("note: this database's key itself is stored in the OS keychain, which stays available while you are logged in");
    }
    ctx.emit_mutation(&Report::object(
        "db_lock",
        to_record(
            &json!({"path": ctx.db_path, "session_cleared": had, "key_in_keychain": keys::keychain_has(&h.db_id)}),
        ),
    ))
}
