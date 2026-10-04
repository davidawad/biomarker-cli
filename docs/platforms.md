# Platforms

biomarker-cli supports five targets. CI (`.github/workflows/ci.yml`) runs
`cargo fmt --check`, `cargo clippy --all-targets -D warnings` and `cargo test`
on each of them, and `.github/workflows/release.yml` builds the release
archives on the same runners.

| target | CI runner | toolchain | release archive |
|--------|-----------|-----------|-----------------|
| `x86_64-unknown-linux-gnu` | `ubuntu-latest` (release: `ubuntu-22.04`, glibc 2.35+) | nightly | `.tar.gz` |
| `aarch64-unknown-linux-gnu` | `ubuntu-24.04-arm` (release: `ubuntu-22.04-arm`) | stable | `.tar.gz` |
| `aarch64-apple-darwin` | `macos-latest` | stable | `.tar.gz` |
| `x86_64-apple-darwin` | `macos-15-intel` | nightly | `.tar.gz` |
| `x86_64-pc-windows-msvc` | `windows-latest` | nightly | `.zip` |

GitHub retired the `macos-13` image in December 2025, so Intel macOS runs on
`macos-15-intel`.

## Why nightly on some targets

fsqlite 0.4 turns on unstable compiler features for some targets. This is
true of every 0.4.x release up to and including 0.4.9, the latest:

* **x86_64 (Linux, macOS, Windows).** `fsqlite-pager` and `fsqlite-btree`
  start with `#![cfg_attr(target_arch = "x86_64", feature(core_intrinsics))]`
  and call `core::intrinsics::prefetch_read_data` for L1 prefetch hints in the
  page cache and B-tree cursors.
* **Windows.** `fsqlite-vfs` starts with
  `#![cfg_attr(windows, feature(windows_by_handle))]`, which it uses to
  identify files (volume serial number + file index) for locking.

No fsqlite cargo feature disables either one. (`nightly-simd` in
`fsqlite-types` is a separate, opt-in feature that we leave off.) On aarch64
neither cfg matches, so Linux arm64 and Apple Silicon build on stable.

`rust-toolchain.toml` cannot choose a channel per target, so it pins `nightly`
for everyone, and rustup honours it automatically. To build aarch64 on stable,
run `cargo +stable build` or set `RUSTUP_TOOLCHAIN=stable`. CI does the latter
for its aarch64 jobs. Setting `RUSTC_BOOTSTRAP=1` on a stable compiler would
also get past the feature gates, but the unstable APIs can change without
notice, so we use a real nightly instead. The release archives are prebuilt,
so people installing biomarker (including through Homebrew) need no Rust
toolchain.

## Key storage

`src/keys.rs` puts one facade (`keychain::{available, load, store, delete}`)
over a backend per OS. All backends store binary secrets under service
`biomarker-cli`.

| OS | backend | crate |
|----|---------|-------|
| macOS | Keychain (generic passwords) | `security-framework` |
| Linux, BSD | freedesktop Secret Service over D-Bus (GNOME Keyring, KWallet, KeePassXC) | `keyring-core` + `zbus-secret-service-keyring-store` |
| Windows | Credential Manager (generic credentials) | `keyring-core` + `windows-native-keyring-store` |

When the backend cannot be reached (a headless server or container with no
D-Bus session bus, a CI runner, or `BIOMARKER_NO_KEYCHAIN=1`), `key_source =
auto` falls back to `BIOMARKER_KEY` and then to an interactive passphrase.
`biomarker doctor` reports the backend and whether it is usable, for example:

```
keychain  warn  freedesktop Secret Service (D-Bus) unavailable: … Failed to connect to address `unix:path=/run/user/1000/bus` …; falling back to BIOMARKER_KEY / passphrase prompt
```

The test suite always uses `BIOMARKER_KEY` with `BIOMARKER_NO_KEYCHAIN=1` and
never touches a real keychain.

## File locations

| OS | config file | database |
|----|-------------|----------|
| Linux, BSD | `$XDG_CONFIG_HOME/biomarker-cli/config.toml`, default `~/.config/biomarker-cli/config.toml` | `$XDG_DATA_HOME/biomarker-cli/biomarker.db`, default `~/.local/share/biomarker-cli/biomarker.db` |
| macOS | same as Linux | same as Linux |
| Windows | `%APPDATA%\biomarker-cli\config.toml` | `%LOCALAPPDATA%\biomarker-cli\biomarker.db` |

* **macOS keeps the XDG-style paths.** Every release so far (0.1, 0.2, through
  Homebrew) has stored the database in `~/.local/share/biomarker-cli`.
  Switching to `~/Library/Application Support` would make existing databases
  look missing, so the shipped location stays the default.
* **Windows** uses the known folders (via the `directories` crate). The
  database goes in the *local* AppData because its key may live in this
  machine's Credential Manager; the small config file roams. No earlier
  release supported Windows, so there is no older location to migrate from.
* **Overrides** work everywhere. `XDG_CONFIG_HOME` / `XDG_DATA_HOME` are
  honoured on every OS when set to an absolute path. `BIOMARKER_CONFIG` /
  `--config` and `BIOMARKER_DB` / `--db` override everything. A leading `~/`
  (or `~\` on Windows) expands to `$HOME`, or to `%USERPROFILE%` when `HOME`
  is unset.

biomarker uses no cache directory.

## Permissions

| | Unix (Linux, macOS) | Windows |
|-|---------------------|---------|
| new database directory | `chmod 0700` | protected DACL: full control for the current user and `SYSTEM`, inherited by children, nothing inherited from the parent |
| files biomarker creates (database, temp file, audit log, lock, `--output`) | created `0600` | the same DACL (non-inheritable) |
| `doctor` check | mode has no group/other bits | every *allow* ACE is for the current user, `SYSTEM`, `Administrators` or the owner placeholders |

The code is in `src/perms.rs`. A directory you created yourself is left alone,
on every OS. `doctor` still reports it and warns when other users can access
it. `SYSTEM` is granted on Windows because backup and indexing services run as
it; it is the counterpart of root, which can read 0600 files anyway. On
volumes without ACL support (FAT/exFAT USB drives) Windows cannot restrict
access at all: biomarker still works there, but `doctor` warns that the files
have no ACL. Keep the database on NTFS.

## Atomic replace, fsync and locking

* **Atomic replace.** The database is written to `<db>.tmp-<pid>`, fsynced and
  renamed over the old file. On Windows `std::fs::rename` replaces an existing
  file, using POSIX semantics where NTFS supports them, so readers that still
  hold the old file open do not block the swap. Directories cannot be opened
  for fsync on Windows, so that step (best effort everywhere) is skipped there.
* **Locking.** `<db>.lock` is locked exclusively with `std::fs::File::lock`,
  which is `flock` on Unix and `LockFileEx` on Windows. No extra crate is
  needed.
* `tests/fs_portability.rs` covers both on every CI platform.

## External tools

biomarker-cli does not start any other program. It does not use minimap2,
samtools, bcftools, gzip, docker or podman. The only shell dependency is in
development: `scripts/readme-session.sh` (the README drift check) needs a
POSIX `sh`. On Windows those tests are reported as *ignored* with that reason;
the Linux and macOS jobs run them.

The genome pipeline lives in **genome-cli**, a separate repository (see
[security.md §7](security.md#7-genome-cli)). This repository cannot change its
process spawning. These are the requirements it should follow, and how its
tools run on each OS:

* **Resolving executables.** Look tools up with the `which` crate
  (`which::which("samtools")`), which applies `PATHEXT` and finds
  `samtools.exe` on Windows. Spawn the resolved path with `std::process::Command`,
  never through `sh -c`, and fail with a message that names the missing tool.
  Tests that need a tool should skip with a message when it is not found.

| tool | Linux / macOS | Windows |
|------|---------------|---------|
| minimap2 | native (bioconda, Homebrew, distro packages) | **WSL2**. There is no maintained native build, and bioconda has no `win-64` packages. |
| samtools, bcftools (htslib) | native | **WSL2**. MSYS2/Cygwin builds exist but are untested and unsupported. |
| gzip | native | native via `gzip.exe` from Git for Windows. In-process decompression (e.g. the `flate2` crate) avoids the dependency. |
| docker | native | native via Docker Desktop (WSL2 backend). Bind-mount Windows paths with forward slashes. |
| podman | native | native via Podman Desktop / `podman machine` (WSL2 VM) |

In practice, run the alignment and variant-calling pipeline either inside WSL2
(install genome-cli's Linux build there) or inside a container from native
Windows. biomarker itself runs natively on Windows.
