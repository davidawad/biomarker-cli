//! Opening, creating, migrating and rekeying encrypted databases.

use std::path::Path;

use crate::crypto::{self, random_array, DataKeys, Header};
use crate::db::{Db, RowExt, Sealed};
use crate::error::{AppError, Result};
use crate::keys::{self, key_error, Source};

pub const INSECURE_WARNING: &str = "warning: --insecure-plaintext: health data is stored UNENCRYPTED on disk";

#[derive(Debug, Clone, Copy)]
pub struct OpenOpts {
    pub source: Source,
    /// Allow plaintext databases (`--insecure-plaintext`).
    pub insecure: bool,
}

/// What is at a database path.
pub enum State {
    Missing,
    Sealed(Header),
    Plain,
}

pub fn state(path: &Path) -> Result<State> {
    if !path.exists() {
        Ok(State::Missing)
    } else if crypto::is_sealed(path) {
        crypto::read_header(path).map(State::Sealed)
    } else {
        Ok(State::Plain)
    }
}

fn disp(path: &Path) -> String {
    path.display().to_string()
}

pub fn plaintext_error(path: &Path) -> AppError {
    key_error(format!(
        "{} is an unencrypted database; encrypt it in place with `biomarker db encrypt`, \
         or pass --insecure-plaintext to keep using it unencrypted",
        path.display()
    ))
}

/// Open (creating if missing) the database at `path`. New databases are
/// encrypted unless `insecure`; they are migrated and sealed before return.
pub fn open(path: &Path, o: OpenOpts, warn: &dyn Fn(&str)) -> Result<Db> {
    let lock = crate::db::lock(path)?;
    match state(path)? {
        State::Missing if o.insecure => {
            warn(INSECURE_WARNING);
            Ok(Db::open_plain(path)?.with_lock(lock))
        }
        State::Missing => {
            let id = random_array()?;
            let nk = keys::new_kek(o.source, keys::ENV_KEY, id, &disp(path))?;
            let keys = DataKeys::random()?;
            let header = Header::new(nk.kind, id, &nk.kek, &keys)?;
            let db = Db::create_sealed(path, Sealed { header, keys, key_source: nk.source.as_str() })?;
            crate::migrations::migrate(&db)?;
            db.persist()?;
            nk.commit()?;
            Ok(db.with_lock(lock))
        }
        State::Sealed(header) => {
            let u = keys::unlock(o.source, &header, &disp(path), true)?;
            Ok(Db::open_sealed(path, Sealed { header, keys: u.keys, key_source: u.source })?.with_lock(lock))
        }
        State::Plain if o.insecure => {
            warn(INSECURE_WARNING);
            Ok(Db::open_plain(path)?.with_lock(lock))
        }
        State::Plain => Err(plaintext_error(path)),
    }
}

/// Unlock an existing encrypted database's keys without loading it.
pub fn unlock(path: &Path, source: Source, allow_prompt: bool) -> Result<(Header, keys::Unlocked)> {
    match state(path)? {
        State::Sealed(h) => {
            let u = keys::unlock(source, &h, &disp(path), allow_prompt)?;
            Ok((h, u))
        }
        State::Missing => Err(AppError::not_found(format!("no database at {}", path.display()))),
        State::Plain => Err(AppError::invalid(format!("{} is not encrypted", path.display()))),
    }
}

/// Row count per user table, for round-trip verification.
fn table_counts(db: &Db) -> Result<Vec<(String, i64)>> {
    let names: Vec<String> = db
        .query("SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name", &[])?
        .iter()
        .filter_map(|r| r.s(0))
        .collect();
    names
        .into_iter()
        .map(|n| {
            let c = db.query_scalar_i64(&format!("SELECT count(*) FROM \"{}\"", n.replace('"', "\"\"")), &[])?;
            Ok((n, c))
        })
        .collect()
}

pub struct EncryptReport {
    pub tables: Vec<(String, i64)>,
    pub bytes: u64,
    pub key_source: &'static str,
    pub wiped: Vec<String>,
}

/// Encrypt a plaintext database in place: seal into a temp file, verify the
/// round trip from disk (integrity check + identical row counts), swap it in,
/// then overwrite and delete the plaintext file and its sidecars.
pub fn encrypt_in_place(path: &Path, source: Source) -> Result<EncryptReport> {
    let lock = crate::db::lock(path)?;
    match state(path)? {
        State::Missing => return Err(AppError::not_found(format!("no database at {}", path.display()))),
        State::Sealed(_) => return Err(AppError::invalid(format!("{} is already encrypted", path.display()))),
        State::Plain => {}
    }
    let plain = Db::open_plain(path)?;
    crate::migrations::migrate(&plain)?;
    let tables = table_counts(&plain)?;
    let image = plain.export_image()?;
    drop(plain);

    let id = random_array()?;
    let nk = keys::new_kek(source, keys::ENV_KEY, id, &disp(path))?;
    let keys = DataKeys::random()?;
    let header = Header::new(nk.kind, id, &nk.kek, &keys)?;
    let sealed = crypto::seal_image(&header, &keys.dek, image)?;
    let tmp = crypto::sidecar(path, ".encrypting");
    crypto::atomic_write(&tmp, &sealed)?;

    let verify = || -> Result<()> {
        let h = crypto::read_header(&tmp)?;
        let k = h.unwrap_keys(&nk.kek).ok_or_else(|| AppError::db("verification: key unwrap failed"))?;
        let db = Db::open_sealed(&tmp, Sealed { header: h, keys: k, key_source: nk.source.as_str() })?;
        let integrity: Vec<String> = db.query("PRAGMA integrity_check", &[])?.iter().filter_map(|r| r.s(0)).collect();
        if integrity != ["ok"] {
            return Err(AppError::db(format!("verification: integrity check failed: {integrity:?}")));
        }
        if table_counts(&db)? != tables {
            return Err(AppError::db("verification: row counts differ after encryption"));
        }
        Ok(())
    };
    if let Err(e) = verify() {
        let _ = std::fs::remove_file(&tmp);
        return Err(e.context("db encrypt aborted; the plaintext database is unchanged"));
    }

    let aside = crypto::sidecar(path, ".plaintext-wipe");
    std::fs::rename(path, &aside).map_err(|e| AppError::io(format!("renaming {}: {e}", path.display())))?;
    std::fs::rename(&tmp, path).map_err(|e| AppError::io(format!("renaming {}: {e}", tmp.display())))?;
    nk.commit()?;
    let mut wiped = Vec::new();
    for p in [aside, crypto::sidecar(path, "-wal"), crypto::sidecar(path, "-shm"), crypto::sidecar(path, "-journal")] {
        if p.exists() {
            crypto::wipe_file(&p)?;
            wiped.push(p.display().to_string());
        }
    }
    drop(lock);
    Ok(EncryptReport { tables, bytes: sealed.len() as u64, key_source: nk.source.as_str(), wiped })
}

pub struct RekeyReport {
    pub from: &'static str,
    pub to: &'static str,
    pub rotated_dek: bool,
    pub db_id: [u8; crypto::ID_LEN],
    pub audit_key: crypto::Key,
}

/// Re-wrap the data keys under a new KEK (and with `rotate_dek` re-encrypt
/// the image under a fresh DEK). The new KEK comes from `to`; for the env
/// source it is read from `BIOMARKER_NEW_KEY`.
pub fn rekey(path: &Path, current: Source, to: Source, rotate_dek: bool) -> Result<RekeyReport> {
    let _lock = crate::db::lock(path)?;
    let (header, u) = unlock(path, current, true)?;
    let bytes = std::fs::read(path).map_err(|e| AppError::io(format!("reading {}: {e}", path.display())))?;
    let nk = keys::new_kek(to, keys::ENV_NEW_KEY, header.db_id, &format!("{} (new key)", disp(path)))?;
    let new_keys =
        if rotate_dek { DataKeys { dek: crypto::Key::random()?, audit: u.keys.audit.clone() } } else { u.keys.clone() };
    let new_header = Header::new(nk.kind, header.db_id, &nk.kek, &new_keys)?;
    let sealed = if rotate_dek {
        let mut image = crypto::open_image(&header, &u.keys.dek, bytes)?;
        crypto::seal_image(&new_header, &new_keys.dek, std::mem::take(&mut *image))?
    } else {
        // Same DEK: authenticate the body, then swap the header only.
        let mut b = bytes;
        drop(crypto::open_image(&header, &u.keys.dek, b.clone())?);
        crypto::replace_header(&mut b, &new_header);
        b
    };
    crypto::atomic_write(path, &sealed)?;
    nk.commit()?;
    let _ = keys::session::clear(&header.db_id);
    if nk.source != Source::Keychain {
        keys::forget_keychain(&header.db_id);
    }
    Ok(RekeyReport {
        from: u.source,
        to: nk.source.as_str(),
        rotated_dek: rotate_dek,
        db_id: header.db_id,
        audit_key: new_keys.audit,
    })
}
