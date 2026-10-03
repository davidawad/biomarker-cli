use serde_json::json;

use crate::cli::DbCmd;
use crate::context::Ctx;
use crate::db::{Db, RowExt};
use crate::error::{AppError, Result};
use crate::migrations;
use crate::output::{to_record, Report};
use crate::store::Catalog;

pub fn run(ctx: &Ctx, cmd: DbCmd) -> Result<()> {
    match cmd {
        DbCmd::Path => ctx.emit(&Report::object(
            "db_path",
            to_record(&json!({"path": ctx.db_path, "exists": ctx.db_path.exists(), "source": ctx.resolved.source("db_path").as_str()})),
        )),
        DbCmd::Init => {
            let existed = ctx.db_path.exists();
            let db = Db::open_raw(&ctx.db_path)?;
            let applied = migrations::migrate(&db)?;
            ctx.info(&format!("{} {}", if existed { "database ready:" } else { "created database" }, ctx.db_path.display()));
            ctx.emit(&Report::object(
                "db_init",
                to_record(&json!({"path": ctx.db_path, "created": !existed, "applied": applied, "version": migrations::current_version(&db)?})),
            ))
        }
        DbCmd::Migrate { status } => migrate(ctx, status),
        DbCmd::Backup { dest } => {
            if dest.exists() {
                return Err(AppError::invalid(format!("{} already exists", dest.display())));
            }
            let db = ctx.db()?;
            db.backup_to(&dest)?;
            let size = std::fs::metadata(&dest).map(|m| m.len()).unwrap_or(0);
            ctx.info(&format!("backed up to {}", dest.display()));
            ctx.emit(&Report::object("db_backup", to_record(&json!({"source": ctx.db_path, "dest": dest, "bytes": size}))))
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
    let db = Db::open_raw(&ctx.db_path)?;
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
