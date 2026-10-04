//! Encryption at rest: the sealed database container and the primitives it is
//! built from (XChaCha20-Poly1305, Argon2id, zeroized key material).
//!
//! fsqlite 0.4.7 ships a page cipher in `fsqlite-pager` but does not wire it
//! into `Connection` (`PRAGMA key` / `PRAGMA fsqlite.key` are silently
//! ignored; see docs/security.md and `tests/security.rs`). The database is
//! therefore kept as a *sealed container*: the whole SQLite image is decrypted
//! into an in-memory fsqlite connection on open and re-encrypted into a fresh
//! file (temp file, fsync, rename) after every committing statement. The
//! plaintext image never touches the filesystem.
//!
//! Container layout (all integers little-endian):
//!
//! ```text
//! off  len  field
//!   0    8  magic "BMSEAL01"
//!   8    1  KEK kind: 1 = raw 256-bit key, 2 = Argon2id passphrase
//!   9    3  reserved (zero)
//!  12    4  Argon2id m_cost (KiB)        } only meaningful for kind 2
//!  16    4  Argon2id t_cost              }
//!  20    4  Argon2id p_cost              }
//!  24   16  Argon2id salt                }
//!  40   16  database id (random, stable across rekey)
//!  56   24  key-wrap nonce
//!  80   80  wrapped DEK || audit key (64 B) + Poly1305 tag; AAD = bytes 0..56
//! 160   24  body nonce
//! 184    …  body: SQLite image encrypted with the DEK + tag; AAD = magic || db id
//! ```

use std::io::Write;
use std::path::{Path, PathBuf};

use chacha20poly1305::aead::AeadInOut;
use chacha20poly1305::{KeyInit, Tag, XChaCha20Poly1305, XNonce};
use zeroize::{Zeroize, Zeroizing};

use crate::error::{AppError, Result};

pub const MAGIC: &[u8; 8] = b"BMSEAL01";
pub const KEY_LEN: usize = 32;
pub const NONCE_LEN: usize = 24;
pub const TAG_LEN: usize = 16;
pub const ID_LEN: usize = 16;
const WRAP_AAD_END: usize = 56;
const WRAPPED_LEN: usize = 2 * KEY_LEN + TAG_LEN;
pub const HEADER_LEN: usize = 184;

/// Argon2id cost used for passphrase-derived KEKs (64 MiB, 3 passes).
pub const ARGON_M_COST: u32 = 64 * 1024;
pub const ARGON_T_COST: u32 = 3;
pub const ARGON_P_COST: u32 = 1;

/// A 256-bit secret key, wiped from memory on drop.
#[derive(Clone, Zeroize, zeroize::ZeroizeOnDrop)]
pub struct Key([u8; KEY_LEN]);

impl Key {
    pub fn random() -> Result<Self> {
        let mut k = [0u8; KEY_LEN];
        fill_random(&mut k)?;
        Ok(Self(k))
    }

    pub fn from_bytes(b: &[u8]) -> Option<Self> {
        <[u8; KEY_LEN]>::try_from(b).ok().map(Self)
    }

    pub fn as_bytes(&self) -> &[u8; KEY_LEN] {
        &self.0
    }

    fn cipher(&self) -> XChaCha20Poly1305 {
        XChaCha20Poly1305::new(&self.0.into())
    }
}

impl std::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Key(..)")
    }
}

pub fn fill_random(buf: &mut [u8]) -> Result<()> {
    getrandom::fill(buf).map_err(|e| AppError::new(crate::error::ErrorKind::General, format!("OS RNG: {e}")))
}

pub fn random_array<const N: usize>() -> Result<[u8; N]> {
    let mut a = [0u8; N];
    fill_random(&mut a)?;
    Ok(a)
}

/// Encrypt `buf` in place and return the detached tag.
pub fn seal_in_place(key: &Key, nonce: &[u8; NONCE_LEN], aad: &[u8], buf: &mut [u8]) -> Result<[u8; TAG_LEN]> {
    let tag = key
        .cipher()
        .encrypt_inout_detached(&XNonce::from(*nonce), aad, buf.into())
        .map_err(|_| AppError::invalid("encryption failed (message too long)"))?;
    Ok(tag.into())
}

/// Decrypt `buf` in place, verifying `tag`. Returns false on authentication failure.
pub fn open_in_place(key: &Key, nonce: &[u8; NONCE_LEN], aad: &[u8], buf: &mut [u8], tag: &[u8; TAG_LEN]) -> bool {
    key.cipher().decrypt_inout_detached(&XNonce::from(*nonce), aad, buf.into(), &Tag::from(*tag)).is_ok()
}

/// Seal a small message as `nonce || ciphertext || tag`.
pub fn seal_message(key: &Key, aad: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    let nonce = random_array::<NONCE_LEN>()?;
    let mut out = Vec::with_capacity(NONCE_LEN + plaintext.len() + TAG_LEN);
    out.extend_from_slice(&nonce);
    out.extend_from_slice(plaintext);
    let tag = seal_in_place(key, &nonce, aad, &mut out[NONCE_LEN..])?;
    out.extend_from_slice(&tag);
    Ok(out)
}

/// Open a message produced by [`seal_message`]; `None` if it fails to authenticate.
pub fn open_message(key: &Key, aad: &[u8], msg: &[u8]) -> Option<Zeroizing<Vec<u8>>> {
    if msg.len() < NONCE_LEN + TAG_LEN {
        return None;
    }
    let nonce: [u8; NONCE_LEN] = msg[..NONCE_LEN].try_into().ok()?;
    let tag: [u8; TAG_LEN] = msg[msg.len() - TAG_LEN..].try_into().ok()?;
    let mut body = Zeroizing::new(msg[NONCE_LEN..msg.len() - TAG_LEN].to_vec());
    open_in_place(key, &nonce, aad, &mut body, &tag).then_some(body)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KdfParams {
    pub m_cost: u32,
    pub t_cost: u32,
    pub p_cost: u32,
    pub salt: [u8; 16],
}

impl KdfParams {
    pub fn fresh() -> Result<Self> {
        Ok(Self { m_cost: ARGON_M_COST, t_cost: ARGON_T_COST, p_cost: ARGON_P_COST, salt: random_array()? })
    }
}

/// Derive a KEK from a passphrase with Argon2id.
pub fn derive_kek(passphrase: &[u8], p: &KdfParams) -> Result<Key> {
    use argon2::{Algorithm, Argon2, Params, Version};
    let params = Params::new(p.m_cost, p.t_cost, p.p_cost, Some(KEY_LEN))
        .map_err(|e| AppError::invalid(format!("bad Argon2id parameters in header: {e}")))?;
    let mut out = [0u8; KEY_LEN];
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
        .hash_password_into(passphrase, &p.salt, &mut out)
        .map_err(|e| AppError::invalid(format!("Argon2id: {e}")))?;
    let k = Key(out);
    out.zeroize();
    Ok(k)
}

/// How the key-encryption key of a container is obtained.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KekKind {
    /// A random 256-bit key (OS keychain or `BIOMARKER_KEY=raw:<hex>`).
    Raw,
    /// Argon2id over a passphrase (env or interactive).
    Passphrase(KdfParams),
}

impl KekKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Raw => "raw-key",
            Self::Passphrase(_) => "passphrase-argon2id",
        }
    }
}

/// The data keys protected by the KEK.
#[derive(Clone)]
pub struct DataKeys {
    /// Encrypts the database image.
    pub dek: Key,
    /// Encrypts audit-log records (kept across DEK rotation so old records stay readable).
    pub audit: Key,
}

impl DataKeys {
    pub fn random() -> Result<Self> {
        Ok(Self { dek: Key::random()?, audit: Key::random()? })
    }
}

/// Parsed container header.
#[derive(Clone)]
pub struct Header {
    pub kek_kind: KekKind,
    pub db_id: [u8; ID_LEN],
    wrap_nonce: [u8; NONCE_LEN],
    wrapped: [u8; WRAPPED_LEN],
}

impl std::fmt::Debug for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Header").field("kek_kind", &self.kek_kind).field("db_id", &hex(&self.db_id)).finish()
    }
}

fn u32_at(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes(b[off..off + 4].try_into().expect("4 bytes"))
}

impl Header {
    /// Wrap `keys` under `kek` into a new header.
    pub fn new(kek_kind: KekKind, db_id: [u8; ID_LEN], kek: &Key, keys: &DataKeys) -> Result<Self> {
        let mut h = Self { kek_kind, db_id, wrap_nonce: random_array()?, wrapped: [0; WRAPPED_LEN] };
        let aad = h.wrap_aad();
        let mut buf = Zeroizing::new([0u8; 2 * KEY_LEN]);
        buf[..KEY_LEN].copy_from_slice(keys.dek.as_bytes());
        buf[KEY_LEN..].copy_from_slice(keys.audit.as_bytes());
        let tag = seal_in_place(kek, &h.wrap_nonce, &aad, &mut buf[..])?;
        h.wrapped[..2 * KEY_LEN].copy_from_slice(&buf[..]);
        h.wrapped[2 * KEY_LEN..].copy_from_slice(&tag);
        Ok(h)
    }

    fn wrap_aad(&self) -> [u8; WRAP_AAD_END] {
        let mut b = [0u8; WRAP_AAD_END];
        b[..8].copy_from_slice(MAGIC);
        match self.kek_kind {
            KekKind::Raw => b[8] = 1,
            KekKind::Passphrase(p) => {
                b[8] = 2;
                b[12..16].copy_from_slice(&p.m_cost.to_le_bytes());
                b[16..20].copy_from_slice(&p.t_cost.to_le_bytes());
                b[20..24].copy_from_slice(&p.p_cost.to_le_bytes());
                b[24..40].copy_from_slice(&p.salt);
            }
        }
        b[40..56].copy_from_slice(&self.db_id);
        b
    }

    pub fn encode(&self) -> [u8; 160] {
        let mut b = [0u8; 160];
        b[..WRAP_AAD_END].copy_from_slice(&self.wrap_aad());
        b[56..80].copy_from_slice(&self.wrap_nonce);
        b[80..160].copy_from_slice(&self.wrapped);
        b
    }

    pub fn decode(b: &[u8]) -> Result<Self> {
        if b.len() < HEADER_LEN || &b[..8] != MAGIC {
            return Err(AppError::db("not a biomarker encrypted database (bad magic)"));
        }
        let kek_kind = match b[8] {
            1 => KekKind::Raw,
            2 => KekKind::Passphrase(KdfParams {
                m_cost: u32_at(b, 12),
                t_cost: u32_at(b, 16),
                p_cost: u32_at(b, 20),
                salt: b[24..40].try_into().expect("16 bytes"),
            }),
            k => return Err(AppError::db(format!("unsupported key kind {k} in encrypted database header"))),
        };
        Ok(Self {
            kek_kind,
            db_id: b[40..56].try_into().expect("16 bytes"),
            wrap_nonce: b[56..80].try_into().expect("24 bytes"),
            wrapped: b[80..160].try_into().expect("80 bytes"),
        })
    }

    /// Unwrap the data keys; `None` when `kek` is wrong or the header was tampered with.
    pub fn unwrap_keys(&self, kek: &Key) -> Option<DataKeys> {
        let mut buf = Zeroizing::new([0u8; 2 * KEY_LEN]);
        buf.copy_from_slice(&self.wrapped[..2 * KEY_LEN]);
        let tag: [u8; TAG_LEN] = self.wrapped[2 * KEY_LEN..].try_into().ok()?;
        if !open_in_place(kek, &self.wrap_nonce, &self.wrap_aad(), &mut buf[..], &tag) {
            return None;
        }
        Some(DataKeys { dek: Key::from_bytes(&buf[..KEY_LEN])?, audit: Key::from_bytes(&buf[KEY_LEN..])? })
    }

    fn body_aad(&self) -> [u8; 8 + ID_LEN] {
        let mut a = [0u8; 8 + ID_LEN];
        a[..8].copy_from_slice(MAGIC);
        a[8..].copy_from_slice(&self.db_id);
        a
    }
}

/// True when the file at `path` starts with the container magic.
pub fn is_sealed(path: &Path) -> bool {
    use std::io::Read;
    let mut m = [0u8; 8];
    std::fs::File::open(path).and_then(|mut f| f.read_exact(&mut m)).is_ok() && &m == MAGIC
}

/// Read only the header of a sealed file.
pub fn read_header(path: &Path) -> Result<Header> {
    use std::io::Read;
    let mut b = [0u8; HEADER_LEN];
    std::fs::File::open(path)
        .and_then(|mut f| f.read_exact(&mut b))
        .map_err(|e| AppError::io(format!("reading {}: {e}", path.display())))?;
    Header::decode(&b)
}

/// Encrypt `image` (consumed; encrypted in place to avoid a plaintext copy)
/// into a complete container byte vector.
pub fn seal_image(header: &Header, dek: &Key, image: Vec<u8>) -> Result<Vec<u8>> {
    let mut image = Zeroizing::new(image);
    let nonce = random_array::<NONCE_LEN>()?;
    let tag = seal_in_place(dek, &nonce, &header.body_aad(), &mut image)?;
    let mut out = Vec::with_capacity(HEADER_LEN + image.len() + TAG_LEN);
    out.extend_from_slice(&header.encode());
    out.extend_from_slice(&nonce);
    out.extend_from_slice(&image);
    out.extend_from_slice(&tag);
    Ok(out)
}

/// Decrypt a container's body (the header must already be parsed and the keys unwrapped).
pub fn open_image(header: &Header, dek: &Key, mut sealed: Vec<u8>) -> Result<Zeroizing<Vec<u8>>> {
    if sealed.len() < HEADER_LEN + TAG_LEN {
        return Err(AppError::db("encrypted database is truncated"));
    }
    let nonce: [u8; NONCE_LEN] = sealed[160..HEADER_LEN].try_into().expect("24 bytes");
    let tag: [u8; TAG_LEN] = sealed[sealed.len() - TAG_LEN..].try_into().expect("16 bytes");
    sealed.truncate(sealed.len() - TAG_LEN);
    sealed.drain(..HEADER_LEN);
    let mut body = Zeroizing::new(sealed);
    if !open_in_place(dek, &nonce, &header.body_aad(), &mut body, &tag) {
        return Err(AppError::db(
            "encrypted database failed authentication: the file is corrupted or has been tampered with",
        ));
    }
    Ok(body)
}

/// Replace the header of an existing sealed byte vector (rekey without touching the body).
pub fn replace_header(sealed: &mut [u8], header: &Header) {
    sealed[..160].copy_from_slice(&header.encode());
}

// ---------------------------------------------------------------------------
// Files
// ---------------------------------------------------------------------------

fn sibling(path: &Path, suffix: &str) -> PathBuf {
    let mut s = path.as_os_str().to_owned();
    s.push(suffix);
    PathBuf::from(s)
}

/// `<path><suffix>`, e.g. `labs.db.audit`.
pub fn sidecar(path: &Path, suffix: &str) -> PathBuf {
    sibling(path, suffix)
}

/// Create a file readable and writable only by the owner (0600 on Unix).
pub fn create_private(path: &Path) -> std::io::Result<std::fs::File> {
    let mut o = std::fs::OpenOptions::new();
    o.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        o.mode(0o600);
    }
    o.open(path)
}

/// Open (creating 0600 if needed) a file for appending.
pub fn open_private_append(path: &Path) -> std::io::Result<std::fs::File> {
    let mut o = std::fs::OpenOptions::new();
    o.append(true).create(true).read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        o.mode(0o600);
    }
    o.open(path)
}

/// Atomically replace `path` with `bytes`: write a private temp file in the
/// same directory, fsync it, rename it over `path`, fsync the directory.
pub fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    let tmp = sibling(path, &format!(".tmp-{}", std::process::id()));
    let res = (|| {
        let mut f = create_private(&tmp)?;
        f.write_all(bytes)?;
        f.sync_all()?;
        drop(f);
        std::fs::rename(&tmp, path)?;
        if let Some(dir) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            // Persist the rename; not supported on every platform, so best effort.
            if let Ok(d) = std::fs::File::open(dir) {
                let _ = d.sync_all();
            }
        }
        Ok::<_, std::io::Error>(())
    })();
    res.map_err(|e| {
        let _ = std::fs::remove_file(&tmp);
        AppError::io(format!("writing {}: {e}", path.display()))
    })
}

/// Best-effort secure delete: overwrite the file with random bytes, fsync,
/// then unlink. Not a guarantee on copy-on-write or wear-levelled storage
/// (APFS, btrfs, SSDs); see docs/security.md.
pub fn wipe_file(path: &Path) -> Result<()> {
    let len = match std::fs::metadata(path) {
        Ok(m) => m.len(),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(AppError::io(format!("{}: {e}", path.display()))),
    };
    let res = (|| {
        let mut f = std::fs::OpenOptions::new().write(true).open(path)?;
        let mut chunk = vec![0u8; 64 * 1024];
        let mut left = len;
        while left > 0 {
            let n = usize::try_from(left.min(chunk.len() as u64)).unwrap_or(chunk.len());
            fill_random(&mut chunk[..n]).map_err(|e| std::io::Error::other(e.message))?;
            f.write_all(&chunk[..n])?;
            left -= n as u64;
        }
        f.sync_all()?;
        drop(f);
        std::fs::remove_file(path)
    })();
    res.map_err(|e| AppError::io(format!("wiping {}: {e}", path.display())))
}

pub fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

pub fn unhex(s: &str) -> Option<Vec<u8>> {
    let s = s.trim();
    if !s.len().is_multiple_of(2) {
        return None;
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(s.get(i..i + 2)?, 16).ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_round_trip_and_tamper() {
        let kek = Key::random().unwrap();
        let keys = DataKeys::random().unwrap();
        let h = Header::new(KekKind::Raw, random_array().unwrap(), &kek, &keys).unwrap();
        let sealed = seal_image(&h, &keys.dek, b"SQLite format 3\0 secret".to_vec()).unwrap();
        assert!(!sealed.windows(6).any(|w| w == b"secret"));
        let h2 = Header::decode(&sealed).unwrap();
        let k2 = h2.unwrap_keys(&kek).unwrap();
        assert_eq!(&open_image(&h2, &k2.dek, sealed.clone()).unwrap()[..], b"SQLite format 3\0 secret");
        assert!(h2.unwrap_keys(&Key::random().unwrap()).is_none());
        let mut bad = sealed.clone();
        let n = bad.len();
        bad[n - 20] ^= 1;
        assert!(open_image(&h2, &k2.dek, bad).is_err());
        let mut bad_hdr = sealed;
        bad_hdr[45] ^= 1; // db id is authenticated by the key wrap
        assert!(Header::decode(&bad_hdr).unwrap().unwrap_keys(&kek).is_none());
    }

    #[test]
    fn passphrase_header_round_trip() {
        let p = KdfParams { m_cost: 64, t_cost: 1, p_cost: 1, salt: [7; 16] };
        let kek = derive_kek(b"pw", &p).unwrap();
        let keys = DataKeys::random().unwrap();
        let h = Header::new(KekKind::Passphrase(p), [1; 16], &kek, &keys).unwrap();
        let d = Header::decode(&[&h.encode()[..], &[0u8; 24]].concat()).unwrap();
        assert_eq!(d.kek_kind, KekKind::Passphrase(p));
        assert!(d.unwrap_keys(&derive_kek(b"pw", &p).unwrap()).is_some());
        assert!(d.unwrap_keys(&derive_kek(b"px", &p).unwrap()).is_none());
    }

    #[test]
    fn hex_round_trip() {
        assert_eq!(unhex(&hex(&[0, 255, 16])).unwrap(), vec![0, 255, 16]);
        assert!(unhex("zz").is_none());
    }
}
