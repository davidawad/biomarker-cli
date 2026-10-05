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

### Container format (`BMSEAL02`)

| offset | size | field |
|-------:|-----:|-------|
| 0    | 8  | magic `BMSEAL02` |
| 8    | 16 | database id (random, stable across rekeys) |
| 24   | 4  | slot table length L |
| 28   | L  | key slots (JSON array) |
| 28+L | 24 | body nonce (fresh on every write) |
| 52+L | …  | SQLite image encrypted under the DEK, then a 16-byte Poly1305 tag (AAD = `BMSEAL01` ‖ database id) |

Each **key slot** wraps the same `DEK ‖ audit key` (64 bytes), and any one slot
opens the database:

| slot | wrap |
|------|------|
| `ssh` | an [age](https://age-encryption.org) message to an SSH public key (ssh-ed25519 or ssh-rsa); the slot also records the key's SHA256 fingerprint and the private key path it was made from |
| `file` | XChaCha20-Poly1305 under a random 256-bit KEK kept in an owner-only key file |
| `env` | XChaCha20-Poly1305 under `BIOMARKER_KEY=raw:<hex>` |
| `keychain` | XChaCha20-Poly1305 under a random KEK in the OS keychain |
| `passphrase` | XChaCha20-Poly1305 under Argon2id(passphrase), parameters and salt in the slot |
| `legacy` | a 0.2/0.3 raw key (OS keychain or `BIOMARKER_KEY=raw:`), read-only |

Raw and passphrase wraps use AAD = magic ‖ database id ‖ slot kind (‖ Argon2id
parameters), so a slot cannot be moved to another database or relabelled.

**Version 1** (`BMSEAL01`, written by 0.2 and 0.3) has a fixed 160-byte header
with exactly one raw or passphrase wrap. It is still read; the first slot
change (`key add-*`, `key remove`, `db rekey`) rewrites it as version 2. The
body AAD is the same in both versions, so slot changes never re-encrypt the body.

* Cipher: XChaCha20-Poly1305 (`chacha20poly1305` crate). 192-bit random nonces
  make nonce reuse across rewrites a non-issue.
* KDF: Argon2id (`argon2` crate), 64 MiB, 3 passes, 1 lane, 16-byte random salt.
  The parameters are stored in the header so they can be raised later.
* Keys (`Key`, `DataKeys`) are zeroized on drop (`zeroize`). Decrypted images
  are held in `Zeroizing` buffers where biomarker owns them.
* The database id and every wrap are authenticated, and so is the body. Flipping
  a byte gives either "wrong key" (database id or a wrap, exit 8), an unreadable
  slot table (exit 5) or "failed authentication: … tampered with" (body, exit 5).
  Slot metadata that is only a hint (an `ssh` slot's private key path) is not
  trusted: the key still has to decrypt the slot.

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
caching the derived KEK in the keychain for a while. An `ssh` slot costs one
X25519 (or RSA) operation.

Biomarker databases are small (thousands to tens of thousands of lab values),
so this costs a few hundred milliseconds at worst. genome-cli's multi-million-row
genotype stores need a different layout (chunked AEAD with an authenticated
chunk index, see §7). Do not reuse this whole-image design there.

## 3. Key management

Each database has a random 256-bit **DEK**, which encrypts the image, and a random
**audit key**, which encrypts audit records. Both are wrapped in one or more key
slots (above). The `key_source` setting / `BIOMARKER_KEY_SOURCE` (`auto`, `ssh`,
`file`, `env`, `keychain`, `passphrase`) decides what a new database gets and
which slots are tried when opening.

### First run (`auto`, the default)

Encryption is never optional and needs no setup. The first command that creates
a database:

1. looks for your SSH key: the `ssh_key` setting / `BIOMARKER_SSH_KEY`, else
   `~/.ssh/id_ed25519`, then `~/.ssh/id_rsa` (on Windows `%USERPROFILE%\.ssh`,
   where OpenSSH keeps them). ed25519 and RSA keys work, with or without a
   passphrase; FIDO (`-sk`) keys and ECDSA keys are skipped;
2. prints which key it will use (path and SHA256 fingerprint, as
   `ssh-keygen -l` shows it), where that is recorded and how to recover, and on a
   terminal asks `Encrypt with this SSH key? [Y/n]`. Without a terminal it
   proceeds and the notice goes to stderr;
3. if the SSH key has a passphrase, also adds a **key file** slot so daily use
   never prompts; the SSH key stays the recovery key;
4. with no usable SSH key (or if you answer no), uses a key file slot alone and
   says so;
5. writes an `[encryption]` section at the end of the config file
   (`biomarker config path`): for every database, the keys that open it and
   plain-English recovery steps. biomarker rewrites that section when keys
   change; your settings above it are left alone.

`BIOMARKER_KEY`, when set, makes an `env` slot instead (CI, scripts).

### Opening

`auto` tries, in order: the key file (if one exists for the database), the SSH
key (silently if it has no passphrase; otherwise its passphrase from
`BIOMARKER_SSH_PASSPHRASE` or a prompt), `BIOMARKER_KEY`, the OS keychain
(**only** for a `keychain` or `legacy` slot), then a passphrase prompt. Prompts
happen only on a terminal. An explicit `key_source` restricts this to that one
kind.

### Key files

`<config dir>/biomarker-cli/keys/<database id>.key` (the `key_file` setting /
`BIOMARKER_KEY_FILE` points elsewhere): one line of hex, created owner-only
(0600 in a 0700 directory on Unix, an owner-only ACL on Windows). Like ssh with
a private key, biomarker refuses a key file other users can read. It lives
under the config directory, not next to the database, so syncing or backing up
the data directory does not carry its key along. New key files are written as
`<name>.next` and renamed once the database sealed with them is on disk.

### The OS keychain

Only used when you ask for it (`key_source = "keychain"`), for `legacy`
databases from 0.2/0.3, and for `db unlock` sessions. Moving a 0.3 database off
the keychain is one command, after which the keychain item is deleted:

```sh
biomarker db rekey --to ssh      # or --to file / --to passphrase
```

It may ask for keychain access one last time. `BIOMARKER_NO_KEYCHAIN=1`
disables the keychain entirely; the test suite and README script set it.

### Commands

| command | effect |
|---------|--------|
| `db init` | creates an **encrypted** database (`--encrypt` is accepted and is the default) |
| `--insecure-plaintext` (global) | the only way to create or open an unencrypted database; prints a warning every time |
| `db encrypt` | migrates a plaintext database in place: seals it to `<db>.encrypting`, verifies the round trip from disk (integrity check and identical row counts per table), swaps it in, then overwrites the plaintext file and any `-wal`/`-shm`/`-journal` with random bytes, fsyncs and deletes them |
| `key status` | lists the slots, whether each key is present on this machine, the recovery advice and the config file |
| `key add-ssh KEY.pub` | adds an SSH public key (a line or a `.pub` path), e.g. a second machine's or an offline recovery key |
| `key add-passphrase` | adds a passphrase (from `BIOMARKER_NEW_KEY`, or asked twice) |
| `key add-file` | adds a key file |
| `key remove N` | removes slot N; the last slot cannot be removed |
| `db rekey [--to SOURCE] [--rotate-dek]` | replaces every slot with a new key from SOURCE (`ssh`, `file`, `env` via `BIOMARKER_NEW_KEY`, `keychain`, `passphrase`). With `--rotate-dek` the image is also re-encrypted under a fresh DEK. The audit key is kept so old audit records stay readable. Key files and keychain KEKs are staged and promoted only after the new container is on disk |
| `db unlock [--ttl 15m]` | caches a passphrase (or raw) KEK in the OS keychain (`session-<id>`) until the TTL expires |
| `db lock` | ends a `db unlock` session |
| `db backup FILE` | writes an encrypted copy under the same keys |
| `doctor` | reports encryption state, every key slot and whether its key is here, recovery advice, session state, whether unlocking works without a prompt, file permissions (Unix mode or Windows ACL), stray plaintext sidecars, and audit-log verification |

Exit code 8 (`key`) means no key was available, the key was wrong, or the database
is plaintext and `--insecure-plaintext` was not given.

**Keep a key off this machine.** Lose every key in a database's slots and the
data is gone; there is no recovery. Your SSH key is usually already backed up or
on another machine; `key add-ssh` adds another machine's key, and `key
add-passphrase` adds a passphrase you can write down.

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
* **Anyone running as you.** A key file, an unprotected SSH key, `BIOMARKER_KEY`
  and the OS keychain all open the database for any program running under your
  account; that is what makes them prompt-free. The same is true of your SSH key
  itself, which already opens your servers. A key file protects against copying
  the database (or a backup, or a synced data folder) without the key; it does
  not protect against malware, or another admin, on the same account. For that,
  use only a passphrase slot (or a passphrase-protected SSH key with no key file:
  `key remove` the file slot) and skip `db unlock`.
* **Environment variables** can leak into shell history, `ps e` and CI logs.
  Prefer an SSH key, a key file or a passphrase on interactive machines.
* **Secure deletion.** `db encrypt` overwrites the plaintext before deleting
  it. On SSDs, copy-on-write filesystems (APFS, btrfs, ZFS) and snapshotting
  backups (Time Machine) old blocks may survive. Full-disk encryption
  (FileVault, LUKS) is still recommended underneath.
* **Plaintext you produce yourself:** stdout redirections, terminal
  scrollback, `--insecure-plaintext`, and input CSV files you import (they are
  read, never copied).
* **Metadata:** file sizes, modification times, and the existence of the
  `.audit`/`.lock` files. The config file (which may contain
  `default_person`, and lists each database's path and key fingerprints in its
  `[encryption]` section) is not encrypted; it holds no key material.
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
