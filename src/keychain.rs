//! The OS keychain (macOS Keychain, freedesktop Secret Service, Windows
//! Credential Manager): only used for databases whose key is configured to
//! live there (`key_source = "keychain"`), databases made by 0.2/0.3 (until
//! `db rekey`), and `db unlock` sessions.

use crate::crypto::hex;
use crate::error::Result;

pub const SERVICE: &str = "biomarker-cli";

/// Keychain account names for a database.
pub fn kek_account(db_id: &[u8]) -> String {
    format!("kek-{}", hex(db_id))
}
pub fn kek_next_account(db_id: &[u8]) -> String {
    format!("kek-{}-next", hex(db_id))
}
fn session_account(db_id: &[u8]) -> String {
    format!("session-{}", hex(db_id))
}

/// Forget every key item belonging to a database id.
pub fn forget(db_id: &[u8]) {
    let _ = delete(&kek_account(db_id));
    let _ = delete(&kek_next_account(db_id));
}

/// Whether the database's primary KEK is stored in the keychain (touches it).
pub fn has(db_id: &[u8]) -> bool {
    matches!(load(&kek_account(db_id)), Ok(Some(_)))
}

/// Time-limited cached KEKs created by `db unlock`.
pub mod session {
    use super::{delete as kc_delete, load as kc_load, session_account, store as kc_store};
    use crate::crypto::{Key, KEY_LEN};
    use crate::error::Result;

    fn now() -> u64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
    }

    /// Cache `kek` for `ttl_secs`.
    pub fn put(db_id: &[u8], kek: &Key, ttl_secs: u64) -> Result<u64> {
        let expires = now().saturating_add(ttl_secs);
        let mut secret = zeroize::Zeroizing::new(Vec::with_capacity(8 + KEY_LEN));
        secret.extend_from_slice(&expires.to_le_bytes());
        secret.extend_from_slice(kek.as_bytes());
        kc_store(&session_account(db_id), &secret)?;
        Ok(expires)
    }

    /// The cached KEK and its expiry, if a live session exists. Expired
    /// sessions are deleted.
    pub fn get_with_expiry(db_id: &[u8]) -> Option<(Key, u64)> {
        let secret = zeroize::Zeroizing::new(kc_load(&session_account(db_id)).ok()??);
        let expires = u64::from_le_bytes(secret.get(..8)?.try_into().ok()?);
        if expires <= now() {
            let _ = kc_delete(&session_account(db_id));
            return None;
        }
        Key::from_bytes(secret.get(8..)?).map(|k| (k, expires))
    }

    pub fn get(db_id: &[u8]) -> Option<Key> {
        get_with_expiry(db_id).map(|(k, _)| k)
    }

    /// Remove a session; true if one existed.
    pub fn clear(db_id: &[u8]) -> Result<bool> {
        kc_delete(&session_account(db_id))
    }
}

#[cfg(target_os = "macos")]
mod imp {
    use super::SERVICE as KEYCHAIN_SERVICE;
    use crate::error::Result;
    use crate::keys::key_error;
    use security_framework::passwords;

    const NOT_FOUND: i32 = -25300; // errSecItemNotFound

    pub fn available() -> std::result::Result<(), String> {
        Ok(())
    }
    pub fn load(account: &str) -> Result<Option<Vec<u8>>> {
        match passwords::get_generic_password(KEYCHAIN_SERVICE, account) {
            Ok(v) => Ok(Some(v)),
            Err(e) if e.code() == NOT_FOUND => Ok(None),
            Err(e) => Err(key_error(format!("keychain: {e}"))),
        }
    }
    pub fn store(account: &str, secret: &[u8]) -> Result<()> {
        passwords::set_generic_password(KEYCHAIN_SERVICE, account, secret)
            .map_err(|e| key_error(format!("keychain: {e}")))
    }
    pub fn delete(account: &str) -> Result<bool> {
        match passwords::delete_generic_password(KEYCHAIN_SERVICE, account) {
            Ok(()) => Ok(true),
            Err(e) if e.code() == NOT_FOUND => Ok(false),
            Err(e) => Err(key_error(format!("keychain: {e}"))),
        }
    }
}

#[cfg(any(windows, all(unix, not(target_os = "macos"))))]
mod imp {
    use std::sync::OnceLock;

    use super::SERVICE as KEYCHAIN_SERVICE;
    use crate::error::Result;
    use crate::keys::key_error;
    use keyring_core::{Entry, Error};

    #[cfg(windows)]
    use windows_native_keyring_store::Store;
    #[cfg(not(windows))]
    use zbus_secret_service_keyring_store::Store;

    #[cfg(windows)]
    const NAME: &str = "credential manager";
    #[cfg(not(windows))]
    const NAME: &str = "secret service";

    /// Open the platform store once per process. On Linux this connects
    /// to the D-Bus session bus, which fails on headless machines.
    fn connect() -> std::result::Result<(), String> {
        static INIT: OnceLock<std::result::Result<(), String>> = OnceLock::new();
        INIT.get_or_init(|| Store::new().map(|s| keyring_core::set_default_store(s)).map_err(|e| e.to_string())).clone()
    }
    fn entry(account: &str) -> Result<Entry> {
        connect().map_err(|e| key_error(format!("{NAME}: {e}")))?;
        Entry::new(KEYCHAIN_SERVICE, account).map_err(|e| key_error(format!("{NAME}: {e}")))
    }

    pub fn available() -> std::result::Result<(), String> {
        connect()
    }
    pub fn load(account: &str) -> Result<Option<Vec<u8>>> {
        match entry(account)?.get_secret() {
            Ok(v) => Ok(Some(v)),
            Err(Error::NoEntry) => Ok(None),
            Err(e) => Err(key_error(format!("{NAME}: {e}"))),
        }
    }
    pub fn store(account: &str, secret: &[u8]) -> Result<()> {
        entry(account)?.set_secret(secret).map_err(|e| key_error(format!("{NAME}: {e}")))
    }
    pub fn delete(account: &str) -> Result<bool> {
        match entry(account)?.delete_credential() {
            Ok(()) => Ok(true),
            Err(Error::NoEntry) => Ok(false),
            Err(e) => Err(key_error(format!("{NAME}: {e}"))),
        }
    }
}

#[cfg(not(any(unix, windows)))]
mod imp {
    use crate::error::Result;
    use crate::keys::key_error;

    pub fn available() -> std::result::Result<(), String> {
        Err("no keychain support on this platform".into())
    }
    pub fn load(_: &str) -> Result<Option<Vec<u8>>> {
        Ok(None)
    }
    pub fn store(_: &str, _: &[u8]) -> Result<()> {
        Err(key_error("no keychain support on this platform"))
    }
    pub fn delete(_: &str) -> Result<bool> {
        Ok(false)
    }
}

#[cfg(target_os = "macos")]
pub const BACKEND: &str = "macOS Keychain";
#[cfg(windows)]
pub const BACKEND: &str = "Windows Credential Manager";
#[cfg(all(unix, not(target_os = "macos")))]
pub const BACKEND: &str = "freedesktop Secret Service (D-Bus)";
#[cfg(not(any(unix, windows)))]
pub const BACKEND: &str = "none";

/// Tests and CI can force the keychain off (`BIOMARKER_NO_KEYCHAIN=1`) so
/// they never touch the developer's real keychain.
fn disabled() -> bool {
    std::env::var_os("BIOMARKER_NO_KEYCHAIN").is_some_and(|v| !v.is_empty() && v != "0")
}

pub fn available() -> std::result::Result<(), String> {
    if disabled() {
        return Err("disabled by BIOMARKER_NO_KEYCHAIN".into());
    }
    imp::available()
}
pub fn load(account: &str) -> Result<Option<Vec<u8>>> {
    if available().is_err() {
        return Ok(None);
    }
    imp::load(account)
}
pub fn store(account: &str, secret: &[u8]) -> Result<()> {
    if let Err(e) = available() {
        return Err(crate::keys::key_error(format!("keychain unavailable: {e}")));
    }
    imp::store(account, secret)
}
pub fn delete(account: &str) -> Result<bool> {
    if available().is_err() {
        return Ok(false);
    }
    imp::delete(account)
}
