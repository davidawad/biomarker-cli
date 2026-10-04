//! Platform-conventional locations for the config file and the database.
//!
//! | platform | config file | database |
//! |----------|-------------|----------|
//! | Linux, BSD | `$XDG_CONFIG_HOME/biomarker-cli/config.toml` (`~/.config/…`) | `$XDG_DATA_HOME/biomarker-cli/biomarker.db` (`~/.local/share/…`) |
//! | macOS | same XDG-style paths as Linux (shipped that way since 0.1) | same |
//! | Windows | `%APPDATA%\biomarker-cli\config.toml` | `%LOCALAPPDATA%\biomarker-cli\biomarker.db` |
//!
//! `XDG_CONFIG_HOME` / `XDG_DATA_HOME` are honoured on every platform when
//! set to an absolute path, and `BIOMARKER_CONFIG` / `BIOMARKER_DB` override
//! both (see [`crate::config`]).

use std::path::PathBuf;

const APP: &str = "biomarker-cli";

/// The user's home directory: `$HOME` when set, else what the OS reports
/// (`%USERPROFILE%` on Windows, the passwd entry on Unix).
pub fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .filter(|h| !h.is_empty())
        .map(PathBuf::from)
        .or_else(|| directories::BaseDirs::new().map(|b| b.home_dir().to_path_buf()))
}

fn home() -> PathBuf {
    home_dir().unwrap_or_else(|| PathBuf::from("."))
}

/// `$var` if it is an absolute path, else `platform()`.
fn base(var: &str, platform: impl FnOnce() -> PathBuf) -> PathBuf {
    std::env::var_os(var).map(PathBuf::from).filter(|p| p.is_absolute()).unwrap_or_else(platform)
}

#[cfg(not(windows))]
fn platform_config() -> PathBuf {
    home().join(".config")
}
#[cfg(not(windows))]
fn platform_data() -> PathBuf {
    home().join(".local").join("share")
}

/// `%APPDATA%` (roaming): the config file is small and follows the user.
#[cfg(windows)]
fn platform_config() -> PathBuf {
    directories::BaseDirs::new().map_or_else(|| home().join("AppData").join("Roaming"), |b| b.config_dir().into())
}
/// `%LOCALAPPDATA%`: the database stays on this machine (its key may live in
/// this machine's Credential Manager).
#[cfg(windows)]
fn platform_data() -> PathBuf {
    directories::BaseDirs::new().map_or_else(|| home().join("AppData").join("Local"), |b| b.data_local_dir().into())
}

pub fn default_config_path() -> PathBuf {
    base("XDG_CONFIG_HOME", platform_config).join(APP).join("config.toml")
}

pub fn default_db_path() -> PathBuf {
    base("XDG_DATA_HOME", platform_data).join(APP).join("biomarker.db")
}

/// Expand a leading `~/` (also `~\` on Windows) to the home directory.
pub fn expand_tilde(p: &str) -> String {
    let rest = p.strip_prefix("~/").or_else(|| if cfg!(windows) { p.strip_prefix("~\\") } else { None });
    match (rest, home_dir()) {
        (Some(rest), Some(home)) => home.join(rest).to_string_lossy().into_owned(),
        _ => p.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_end_in_app_dir() {
        assert!(default_db_path().ends_with(format!("{APP}/biomarker.db")));
        assert!(default_config_path().ends_with(format!("{APP}/config.toml")));
    }

    #[test]
    fn tilde_only_expands_a_prefix() {
        assert_eq!(expand_tilde("/abs/x.db"), "/abs/x.db");
        assert_eq!(expand_tilde("a~/x.db"), "a~/x.db");
        if let Some(home) = home_dir() {
            assert_eq!(PathBuf::from(expand_tilde("~/x.db")), home.join("x.db"));
        }
    }
}
