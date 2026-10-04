# Security: encryption at rest, keys, audit trail

biomarker stores personal health data, so everything it writes to disk is
encrypted by default. This document records the facts the design rests on,
the threat model, and what is and is not protected.

> **HIPAA note.** HIPAA obligations apply to *covered entities* and their
> business associates, not to software. biomarker provides technical
> safeguards of the kind the Security Rule asks for (encryption at rest,
> access control through keys, an audit trail). It does not make anyone
> "HIPAA compliant": that also takes administrative and physical safeguards,
> risk analysis, BAAs, and so on.

## 1. Facts about fsqlite 0.4.7 (the pinned engine)

frankensqlite.com documents page-level encryption through `PRAGMA fsqlite.key`
(XChaCha20-Poly1305, Argon2id, DEK/KEK envelope). In the version this repo pins
(`fsqlite = 0.4.7`):

* `fsqlite-pager/src/encrypt.rs` does contain a `PageEncryptor` (XChaCha20-Poly1305,
  DEK/KEK, AAD with page number and database id).
* Nothing in `fsqlite-core` constructs or calls it. `Connection` has no `key` or
  `rekey` PRAGMA dispatch, and unknown PRAGMAs are **silently ignored**
  (upstream test `test_unrecognised_pragma_silently_ignored`).
* `tests/security.rs::fsqlite_pragma_key_does_not_encrypt` opens a file
  database, runs `PRAGMA fsqlite.key = '…'` (and, separately, `PRAGMA key = '…'`),
  inserts a marker string, closes, and scans the main file and every sidecar.
  Result on 0.4.7: both PRAGMAs succeed without error, and the marker is found
  **in plaintext in `pragma.db` and in `pragma.db-wal`**. The other sidecars
  fsqlite creates (`-shm`, `-wal-cert`, `-wal-cert-head`, `-fsqlite-ns-use`,
  `-fsqlite-ns-gate`, `.fsqlite-migration-state`) did not contain it.

**Conclusion: the PRAGMA does not encrypt.** biomarker therefore implements its
own encryption layer with the same primitives and keeps fsqlite as the SQL
engine. If that test ever starts failing, fsqlite has gained real page
encryption and this design should be revisited.

## 2. Design: a sealed container

fsqlite can load a database image into a private in-memory VFS
(`Connection::import_bytes`) and serialise one back out (`Connection::export_bytes`).
biomarker uses this:

1. **Open**: read the container file, unwrap the data key, decrypt the image
   in memory and `import_bytes` it into an in-memory connection. No plaintext
   file, journal, WAL or temp file is created.
2. **Commit**: after every committing statement or transaction,
   `export_bytes`, encrypt the image under the DEK, write it to
   `<db>.tmp-<pid>` (owner-only: mode 0600, or an owner-only ACL on Windows),
   `fsync`, `rename` over the database, then `fsync` the directory (not
   possible on Windows, where `rename` is `MoveFileExW`/POSIX-semantics
   replace and NTFS journals it). A crash leaves either the old or the new
   container, never a half-written one. Read-only commands never rewrite the
   file.
3. **Concurrency**: every process takes an exclusive lock on `<db>.lock` (an
   empty file; `flock` on Unix, `LockFileEx` on Windows, via `std::fs::File::lock`)
   for as long as it has the database open, so whole-image re-sealing cannot
   lose a concurrent update.

### Container format (`BMSEAL01`)

| offset | size | field |
|-------:|-----:|-------|
| 0   | 8  | magic `BMSEAL01` |
| 8   | 1  | KEK kind: 1 = raw 256-bit key, 2 = Argon2id passphrase |
| 12  | 12 | Argon2id m_cost (KiB), t_cost, p_cost (kind 2) |
| 24  | 16 | Argon2id salt (kind 2) |
| 40  | 16 | database id (random, stable across rekeys) |
| 56  | 24 | key-wrap nonce |
| 80  | 80 | DEK ‖ audit key, wrapped with XChaCha20-Poly1305 under the KEK (AAD = bytes 0..56) |
| 160 | 24 | body nonce (fresh on every write) |
| 184 | …  | SQLite image encrypted under the DEK, then a 16-byte Poly1305 tag (AAD = magic ‖ database id) |

* Cipher: XChaCha20-Poly1305 (`chacha20poly1305` crate). 192-bit random nonces
  make nonce reuse across rewrites a non-issue.
* KDF: Argon2id (`argon2` crate), 64 MiB, 3 passes, 1 lane, 16-byte random salt.
  The parameters are stored in the header so they can be raised later.
* Keys (`Key`, `DataKeys`) are zeroized on drop (`zeroize`). Decrypted images
  are held in `Zeroizing` buffers where biomarker owns them.
* Every byte of the header is authenticated (by the key wrap), and so is the body.
  Flipping any byte gives either "wrong key" (header, exit 8) or "failed
  authentication: … tampered with" (body, exit 5).

### Cost

Measured on an Apple Silicon Mac (release build, stable Rust, encryption on,
key from the environment with Argon2id), synthetic data: 13 markers across 20
people. Every invocation is a fresh process, so each figure includes one
full open (decrypt) and, for writes, one full seal:

| rows | db size | import (all rows) | query (all) | latest | add one | trend |
|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.7 MB | 1.00 s | 0.22 s | 0.23 s | 0.30 s | 0.23 s |
| 10,000 | 8.5 MB | 2.99 s | 0.60 s | 0.47 s | 0.42 s | 0.35 s |
| 50,000 | 20 MB | 12.52 s | 0.94 s | 0.48 s | 0.46 s | 0.65 s |

A personal lab history is typically a few thousand rows, where every
command stays well under a second.

Each command costs one full decrypt (open) and, if it writes, one full
encrypt + fsync (commit). Both are linear in the database size. XChaCha20-Poly1305
runs at well over 1 GB/s, so the remaining cost is fsqlite's own image import
and export plus the write. A passphrase KEK adds one Argon2id derivation per
command (about 0.1–0.3 s with the default parameters). `db unlock` avoids that by
caching the derived KEK in the keychain for a while.

Biomarker databases are small (thousands to tens of thousands of lab values),
so this costs a few hundred milliseconds at worst. genome-cli's multi-million-row
genotype stores need a different layout (chunked AEAD with an authenticated
chunk index, see §7). Do not reuse this whole-image design there.

## 3. Key management

Each database has a random 256-bit **DEK**, which encrypts the image, and a random
**audit key**, which encrypts audit records. Both are wrapped by a **KEK**. The
KEK sources are tried in this order. Restrict them with the `key_source` setting
or `BIOMARKER_KEY_SOURCE` (`auto`, `keychain`, `env`, `passphrase`):

1. **OS keychain.** On macOS this is the Keychain, via `security-framework`. On
   Linux it is the freedesktop Secret Service (GNOME Keyring, KWallet, …) via
   `keyring-core` and its zbus store. On Windows it is the Credential Manager
   (generic credentials) via `keyring-core` and its windows-native store.
   biomarker generates a random KEK per database and stores it under service
   `biomarker-cli`, account `kek-<database id>`. Anyone who can unlock your
   login keychain (or, on Windows, log in as you) can open the database. When
   the keychain is unusable, for example on a headless Linux server or in CI
   with no D-Bus session bus, `auto` moves on to the next sources, and
   `doctor`'s `keychain` row names the backend and why it is unavailable.
2. **`BIOMARKER_KEY` environment variable** (CI, scripts, servers).
   `raw:<64 hex chars>` is used directly as the KEK. Any other value is a
   passphrase and goes through Argon2id.
3. **Interactive passphrase** (Argon2id), prompted on the terminal when stdin
   is a TTY. New passphrases must be at least 8 characters and are confirmed.

`BIOMARKER_NO_KEYCHAIN=1` disables the keychain entirely. The test suite and the
README script set it so they never touch a developer's real keychain.

### Commands

| command | effect |
|---------|--------|
| `db init` | creates an **encrypted** database (`--encrypt` is accepted and is the default) |
| `--insecure-plaintext` (global) | the only way to create or open an unencrypted database; prints a warning every time |
| `db encrypt` | migrates a plaintext database in place: seals it to `<db>.encrypting`, verifies the round trip from disk (integrity check and identical row counts per table), swaps it in, then overwrites the plaintext file and any `-wal`/`-shm`/`-journal` with random bytes, fsyncs and deletes them |
| `db rekey [--to SOURCE] [--rotate-dek]` | re-wraps the data keys under a new KEK. For `env` the new key is read from `BIOMARKER_NEW_KEY`. With `--rotate-dek` the image is also re-encrypted under a fresh DEK. The audit key is kept so old audit records stay readable. Keychain KEKs are staged (`kek-<id>-next`) and promoted only after the new container is on disk. |
| `db unlock [--ttl 15m]` | caches the KEK in the OS keychain (`session-<id>`) until the TTL expires, so passphrase databases stop prompting |
| `db lock` | ends a `db unlock` session (keychain-stored KEKs stay; the note says so) |
| `db backup FILE` | writes an encrypted copy under the same keys |
| `doctor` | reports encryption state, KEK kind, the keychain backend and available key sources, session state, whether unlocking works without a prompt, file permissions (Unix mode or Windows ACL), stray plaintext sidecars, and audit-log verification |

Exit code 8 (`key`) means no key was available, the key was wrong, or the database
is plaintext and `--insecure-plaintext` was not given.

**Back up your key.** A database whose KEK exists only in one machine's
keychain cannot be opened anywhere else. Before you move it, rekey it to a
passphrase (`db rekey --to passphrase`) or to an env key. Lose the key and the
data is gone; there is no recovery.

## 4. Audit trail

`<db>.audit` is append-only. Every command that opens an encrypted database
(reads included) appends one record:

```json
{"ts": "...", "user": "jb", "command": "export", "ok": true, "exit_code": 0,
 "rows_changed": 0, "rows_out": 42, "output": "encrypted-file"}
```

Records contain **no values**: no arguments, slugs, names, notes or results.
Each record is sealed with the audit key, and its AAD includes the database id
and the previous record's tag. Together the records form a MAC chain, so editing,
reordering or removing a record in the middle fails verification. `audit log [-n N]`
decrypts and verifies the whole chain, and `doctor` verifies it as well.

Limitations: truncating the *newest* records cannot be detected from the file
alone. Failed unlock attempts cannot be logged, because there is no key to log
them with. Plaintext (`--insecure-plaintext`) databases are not audited.

## 5. Exports and output

* `--output FILE` writes owner-only files (0600; an owner-only ACL on
  Windows). When the command read the database and the output is not
  encrypted, biomarker warns on stderr:
  `warning: writing plaintext health data to FILE; use --encrypt-output`.
* `--encrypt-output` writes an ASCII-armored **age** file, which you can decrypt
  with `age -d` or `rage -d`. The recipients come from `--recipient age1…`
  (repeatable). Without recipients biomarker uses a passphrase from
  `BIOMARKER_EXPORT_PASSPHRASE` or an interactive prompt (age scrypt).
* Output on stdout goes wherever you send it. biomarker cannot tell whether a
  shell redirection lands on an encrypted volume.

## 6. Threat model

**Protected:**

* Theft or copying of the database file, its backups or the audit log: lost
  laptop, synced folders, cloud backups, a disk image. Without the KEK the
  contents are indistinguishable from random data.
* Offline tampering with any of those files. It is detected and refused with a
  clear error.
* Plaintext leaking through SQLite side files. Encrypted databases never create
  `-wal`, `-journal`, `-shm` or temp files: the engine runs on an in-memory VFS.
* Accidental plaintext exports. You get a warning, `--encrypt-output` exists, and
  output files are owner-only (0600, or an owner-only ACL on Windows).

**Not protected:**

* **Memory.** While a command runs, the decrypted database is in process memory.
  An attacker with root, a debugger, or `/proc/<pid>/mem` access can read it.
  fsqlite's internal buffers are not zeroized by biomarker.
* **Swap, hibernation files and core dumps** can capture that memory. Use
  encrypted swap (default on macOS) and disable core dumps if that matters to you.
* **A compromised account.** Malware running as you can read the keychain item
  or `BIOMARKER_KEY`, or wait for you to unlock. Keychain mode trades this for
  convenience. Use passphrase mode without `db unlock` for the strongest
  at-rest posture.
* **Environment variables** can leak into shell history, `ps e` and CI logs.
  Prefer the keychain or a passphrase on interactive machines.
* **Secure deletion.** `db encrypt` overwrites the plaintext before deleting
  it. On SSDs, copy-on-write filesystems (APFS, btrfs, ZFS) and snapshotting
  backups (Time Machine) old blocks may survive. Full-disk encryption
  (FileVault, LUKS) is still recommended underneath.
* **Plaintext you produce yourself:** stdout redirections, terminal
  scrollback, `--insecure-plaintext`, and input CSV files you import (they are
  read, never copied).
* **Metadata:** file sizes, modification times, and the existence of the
  `.audit`/`.lock` files. The config file (which may contain
  `default_person`) is not encrypted.
* **External tools.** The genome pipeline's BAM/VCF intermediates are written by
  third-party programs (see §7).

## 7. genome-cli

This repository contains only biomarker-cli. The genome-cli parts of the
requirement apply to that codebase and are **not implemented here**: per-kit
genotype stores with chunked AEAD (per-chunk nonces, an authenticated chunk index
for fast random access), encrypted reference/rsid caches derived from personal
data, and `pipeline run --seal`, which encrypts final outputs and deletes
intermediates. Intermediates written by external tools (aligners, variant
callers) are plaintext by nature and must live on an encrypted volume. The
`crypto` module here (container format, key sources, audit chain) is meant to be
shared with genome-cli.
