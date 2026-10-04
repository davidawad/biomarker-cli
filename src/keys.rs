//! Key-encryption-key (KEK) sources and resolution.
//!
//! Each database has a random data-encryption key (DEK) that is wrapped by a
//! KEK (see [`crate::crypto`]). The KEK comes from, in order (configurable
//! with the `key_source` setting / `BIOMARKER_KEY_SOURCE`):
//!
//! 1. the OS keychain: macOS Keychain (security-framework), the freedesktop
//!    Secret Service on Linux/BSD (keyring-core + zbus store) or the Windows
//!    Credential Manager (keyring-core + windows-native store). A random
//!    256-bit KEK is stored per database under service `biomarker-cli`.
//!    Without a usable keychain (headless server, CI, no D-Bus session) the
//!    next sources are used;
//! 2. the `BIOMARKER_KEY` environment variable: `raw:<64 hex chars>` is used
//!    directly as the KEK, anything else is a passphrase run through Argon2id;
//! 3. an interactive passphrase (Argon2id), when stdin is a terminal.
//!
//! `db unlock` caches a KEK in the keychain for a limited time (a "session");
//! `db lock` removes it.

use std::io::IsTerminal;

use zeroize::Zeroizing;

use crate::crypto::{self, derive_kek, hex, unhex, DataKeys, Header, KdfParams, KekKind, Key, KEY_LEN};
use crate::error::{AppError, ErrorKind, Result};

pub const ENV_KEY: &str = "BIOMARKER_KEY";
pub const ENV_NEW_KEY: &str = "BIOMARKER_NEW_KEY";
pub const KEYCHAIN_SERVICE: &str = "biomarker-cli";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    Auto,
    Keychain,
    Env,
    Passphrase,
}

impl Source {
    pub fn parse(s: &str) -> Result<Self> {
        match s {
            "" | "auto" => Ok(Self::Auto),
            "keychain" => Ok(Self::Keychain),
            "env" => Ok(Self::Env),
            "passphrase" => Ok(Self::Passphrase),
            other => Err(AppError::config(format!("unknown key source '{other}'"))),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Keychain => "keychain",
            Self::Env => "env",
            Self::Passphrase => "passphrase",
        }
    }

    /// Concrete sources to try, in order.
    fn order(self) -> &'static [Self] {
        match self {
            Self::Auto => &[Self::Keychain, Self::Env, Self::Passphrase],
            Self::Keychain => &[Self::Keychain],
            Self::Env => &[Self::Env],
            Self::Passphrase => &[Self::Passphrase],
        }
    }
}

pub fn key_error(m: impl Into<String>) -> AppError {
    AppError::new(ErrorKind::Key, m)
}

/// A value of `BIOMARKER_KEY`-style variables.
pub enum EnvKey {
    Raw(Key),
    Passphrase(Zeroizing<String>),
}

pub fn parse_env_key(var: &str) -> Result<Option<EnvKey>> {
    let Some(v) = std::env::var_os(var) else { return Ok(None) };
    let v = Zeroizing::new(v.to_string_lossy().into_owned());
    if v.is_empty() {
        return Ok(None);
    }
    if let Some(h) = v.strip_prefix("raw:") {
        let bytes = Zeroizing::new(unhex(h).unwrap_or_default());
        return Key::from_bytes(&bytes)
            .map(|k| Some(EnvKey::Raw(k)))
            .ok_or_else(|| key_error(format!("{var}=raw:… must be {} hex characters", 2 * KEY_LEN)));
    }
    Ok(Some(EnvKey::Passphrase(v)))
}

fn interactive() -> bool {
    std::io::stdin().is_terminal()
}

fn prompt(msg: &str) -> Result<Zeroizing<String>> {
    rpassword::prompt_password(msg).map(Zeroizing::new).map_err(|e| key_error(format!("reading passphrase: {e}")))
}

fn prompt_new(what: &str) -> Result<Zeroizing<String>> {
    let a = prompt(&format!("New passphrase for {what}: "))?;
    if a.chars().count() < 8 {
        return Err(key_error("passphrase must be at least 8 characters"));
    }
    let b = prompt("Repeat passphrase: ")?;
    if *a != *b {
        return Err(key_error("passphrases do not match"));
    }
    Ok(a)
}

/// Keychain account names for a database.
fn kek_account(db_id: &[u8]) -> String {
    format!("kek-{}", hex(db_id))
}
fn kek_next_account(db_id: &[u8]) -> String {
    format!("kek-{}-next", hex(db_id))
}
fn session_account(db_id: &[u8]) -> String {
    format!("session-{}", hex(db_id))
}

/// A freshly chosen KEK for a new container (or the target of a rekey).
pub struct NewKek {
    pub kind: KekKind,
    pub kek: Key,
    pub source: Source,
    /// Keychain account the KEK was staged under (promoted by [`NewKek::commit`]).
    staged: Option<String>,
    db_id: [u8; crypto::ID_LEN],
}

impl NewKek {
    /// Make a staged keychain KEK the database's primary key. Call after the
    /// container sealed with it has been durably written.
    pub fn commit(&self) -> Result<()> {
        if let Some(staged) = &self.staged {
            keychain::store(&kek_account(&self.db_id), self.kek.as_bytes())?;
            let _ = keychain::delete(staged);
        }
        Ok(())
    }
}

/// Pick a KEK for a new database (or a rekey target). `env_var` is
/// `BIOMARKER_KEY` for new databases and `BIOMARKER_NEW_KEY` for rekey.
pub fn new_kek(source: Source, env_var: &str, db_id: [u8; crypto::ID_LEN], what: &str) -> Result<NewKek> {
    let mut why = Vec::new();
    for s in source.order() {
        match s {
            Source::Keychain => match keychain::available() {
                Ok(()) => {
                    let kek = Key::random()?;
                    let staged = kek_next_account(&db_id);
                    // A reachable but unusable keychain (e.g. a D-Bus session
                    // without a Secret Service provider) falls through too.
                    match keychain::store(&staged, kek.as_bytes()) {
                        Ok(()) => {
                            return Ok(NewKek { kind: KekKind::Raw, kek, source: *s, staged: Some(staged), db_id })
                        }
                        Err(e) => why.push(format!("keychain unusable ({})", e.message)),
                    }
                }
                Err(e) => why.push(format!("keychain unavailable ({e})")),
            },
            Source::Env => match parse_env_key(env_var)? {
                Some(EnvKey::Raw(kek)) => {
                    return Ok(NewKek { kind: KekKind::Raw, kek, source: *s, staged: None, db_id })
                }
                Some(EnvKey::Passphrase(p)) => {
                    let params = KdfParams::fresh()?;
                    let kek = derive_kek(p.as_bytes(), &params)?;
                    return Ok(NewKek { kind: KekKind::Passphrase(params), kek, source: *s, staged: None, db_id });
                }
                None => why.push(format!("{env_var} not set")),
            },
            Source::Passphrase => {
                if interactive() {
                    let p = prompt_new(what)?;
                    let params = KdfParams::fresh()?;
                    let kek = derive_kek(p.as_bytes(), &params)?;
                    return Ok(NewKek { kind: KekKind::Passphrase(params), kek, source: *s, staged: None, db_id });
                }
                why.push("no terminal for a passphrase prompt".into());
            }
            Source::Auto => {}
        }
    }
    Err(key_error(format!(
        "no encryption key available for {what} ({}); set {env_var}, run interactively, or pass --insecure-plaintext",
        why.join("; ")
    )))
}

/// Result of unlocking a container.
pub struct Unlocked {
    pub kek: Key,
    pub keys: DataKeys,
    /// Where the KEK came from: "session", "keychain", "env" or "passphrase".
    pub source: &'static str,
}

/// Unlock a container header with the configured sources. Tries a `db unlock`
/// session first, then each source in order; prompts only if `allow_prompt`.
pub fn unlock(source: Source, header: &Header, what: &str, allow_prompt: bool) -> Result<Unlocked> {
    let id = header.db_id;
    let try_key =
        |kek: Key, src: &'static str| header.unwrap_keys(&kek).map(|keys| Unlocked { kek, keys, source: src });
    let mut tried = Vec::new();
    if matches!(source, Source::Auto | Source::Keychain) {
        if let Some(kek) = session::get(&id) {
            if let Some(u) = try_key(kek, "session") {
                return Ok(u);
            }
        }
    }
    for s in source.order() {
        match s {
            Source::Keychain => {
                for account in [kek_account(&id), kek_next_account(&id)] {
                    if let Ok(Some(secret)) = keychain::load(&account) {
                        tried.push("keychain");
                        if let Some(u) = Key::from_bytes(&secret).and_then(|k| try_key(k, "keychain")) {
                            return Ok(u);
                        }
                    }
                }
            }
            Source::Env => match (parse_env_key(ENV_KEY)?, header.kek_kind) {
                (Some(EnvKey::Raw(k)), _) => {
                    tried.push(ENV_KEY);
                    if let Some(u) = try_key(k, "env") {
                        return Ok(u);
                    }
                }
                (Some(EnvKey::Passphrase(p)), KekKind::Passphrase(params)) => {
                    tried.push(ENV_KEY);
                    if let Some(u) = try_key(derive_kek(p.as_bytes(), &params)?, "env") {
                        return Ok(u);
                    }
                }
                (Some(EnvKey::Passphrase(_)), KekKind::Raw) => {
                    tried.push(ENV_KEY);
                }
                (None, _) => {}
            },
            Source::Passphrase => {
                if let (KekKind::Passphrase(params), true) = (header.kek_kind, allow_prompt && interactive()) {
                    let p = prompt(&format!("Passphrase for {what}: "))?;
                    if let Some(u) = try_key(derive_kek(p.as_bytes(), &params)?, "passphrase") {
                        return Ok(u);
                    }
                    return Err(key_error(format!("wrong passphrase for {what}")));
                }
            }
            Source::Auto => {}
        }
    }
    if tried.is_empty() {
        let hint = match header.kek_kind {
            KekKind::Raw => format!("its key lives in the OS keychain or in {ENV_KEY}=raw:<hex>"),
            KekKind::Passphrase(_) => format!("set {ENV_KEY} to its passphrase or run interactively"),
        };
        Err(key_error(format!("{what} is encrypted and no key is available: {hint}")))
    } else {
        Err(key_error(format!("wrong key for {what} (tried: {})", tried.join(", "))))
    }
}

/// Forget every keychain item belonging to a database id (after rekeying away
/// from the keychain or when the database is gone).
pub fn forget_keychain(db_id: &[u8]) {
    let _ = keychain::delete(&kek_account(db_id));
    let _ = keychain::delete(&kek_next_account(db_id));
}

/// Whether the database's primary KEK is stored in the keychain.
pub fn keychain_has(db_id: &[u8]) -> bool {
    matches!(keychain::load(&kek_account(db_id)), Ok(Some(_)))
}

pub fn keychain_status() -> std::result::Result<(), String> {
    keychain::available()
}

/// Name of this platform's keychain backend, e.g. "Windows Credential Manager".
pub fn keychain_backend() -> &'static str {
    keychain::BACKEND
}

/// Time-limited cached KEKs created by `db unlock`.
pub mod session {
    use super::{keychain, session_account, Key, KEY_LEN};
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
        keychain::store(&session_account(db_id), &secret)?;
        Ok(expires)
    }

    /// The cached KEK and its expiry, if a live session exists. Expired
    /// sessions are deleted.
    pub fn get_with_expiry(db_id: &[u8]) -> Option<(Key, u64)> {
        let secret = zeroize::Zeroizing::new(keychain::load(&session_account(db_id)).ok()??);
        let expires = u64::from_le_bytes(secret.get(..8)?.try_into().ok()?);
        if expires <= now() {
            let _ = keychain::delete(&session_account(db_id));
            return None;
        }
        Key::from_bytes(secret.get(8..)?).map(|k| (k, expires))
    }

    pub fn get(db_id: &[u8]) -> Option<Key> {
        get_with_expiry(db_id).map(|(k, _)| k)
    }

    /// Remove a session; true if one existed.
    pub fn clear(db_id: &[u8]) -> Result<bool> {
        keychain::delete(&session_account(db_id))
    }
}

/// Minimal platform keychain facade: binary secrets keyed by account name
/// under the `biomarker-cli` service.
mod keychain {
    use crate::error::Result;

    #[cfg(target_os = "macos")]
    mod imp {
        use super::super::{key_error, KEYCHAIN_SERVICE};
        use crate::error::Result;
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

        use super::super::{key_error, KEYCHAIN_SERVICE};
        use crate::error::Result;
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
            INIT.get_or_init(|| Store::new().map(|s| keyring_core::set_default_store(s)).map_err(|e| e.to_string()))
                .clone()
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
        use super::super::key_error;
        use crate::error::Result;

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
            return Err(super::key_error(format!("keychain unavailable: {e}")));
        }
        imp::store(account, secret)
    }
    pub fn delete(account: &str) -> Result<bool> {
        if available().is_err() {
            return Ok(false);
        }
        imp::delete(account)
    }
}
