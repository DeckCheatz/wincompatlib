use std::path::PathBuf;
use std::io::{Error, ErrorKind};

use super::*;

pub trait WineBootExt {
    fn wineboot_command(&self) -> Command;
    fn update_prefix<T: Into<PathBuf>>(&self, path: Option<T>) -> Result<Output>;
    fn stop_processes(&self, force: bool) -> Result<Output>;
    fn restart(&self) -> Result<Output>;
    fn shutdown(&self) -> Result<Output>;
    fn end_session(&self) -> Result<Output>;
}

impl WineBootExt for Wine {
    /// Get base `wineboot` command. Will return `wine wineboot` if `self.wineboot()` is `None`
    fn wineboot_command(&self) -> Command {
        match self.wineboot() {
            Some(WineBoot::Unix(wineboot)) => Command::new(wineboot),

            Some(WineBoot::Windows(wineboot)) => {
                let mut command = Command::new(&self.binary);

                command.arg(wineboot);

                command
            }

            None => {
                let mut command = Command::new(&self.binary);

                command.arg("wineboot");

                command
            }
        }
    }

    /// Create (or update existing) wine prefix. Runs `wineboot -u` command
    /// 
    /// ```no_run
    /// use wincompatlib::prelude::*;
    /// 
    /// Wine::default()
    ///     .update_prefix(Some("/path/to/prefix"))
    ///     .expect("Failed to update prefix");
    /// ```
    /// 
    /// Use prefix specified in current wine struct:
    /// 
    /// ```no_run
    /// use wincompatlib::prelude::*;
    /// 
    /// Wine::default()
    ///     .with_prefix("/path/to/prefix")
    ///     .update_prefix::<&str>(None) // We need to specify the type for T: Into<PathBuf>
    ///     .expect("Failed to update prefix");
    /// ```
    /// 
    /// If prefix is not specified in `Wine` struct and is not given to `update_prefix` method -
    /// then `Err` will be returned
    fn update_prefix<T: Into<PathBuf>>(&self, path: Option<T>) -> Result<Output> {
        let Some(path) = path.map(|path| path.into()).or_else(move || self.prefix.clone()) else {
            return Err(Error::new(ErrorKind::InvalidInput, "No prefix path given"));
        };

        // Create all parent directories
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }

        self.wineboot_command()
            .arg("-u")
            .envs(self.get_envs())
            .env("WINEPREFIX", path)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    }

    /// Stop running processes. Runs `wineboot -k` command, or `wineboot -f` if `force = true`
    /// 
    /// ```no_run
    /// use wincompatlib::prelude::*;
    /// 
    /// Wine::default()
    ///     .stop_processes(false)
    ///     .expect("Failed to update prefix");
    /// ```
    fn stop_processes(&self, force: bool) -> Result<Output> {
        self.wineboot_command()
            .arg(if force { "-f" } else { "-k" })
            .envs(self.get_envs())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    }

    /// Imitate windows restart. Runs `wineboot -r` command
    /// 
    /// ```no_run
    /// use wincompatlib::prelude::*;
    /// 
    /// Wine::default()
    ///     .with_prefix("/path/to/prefix")
    ///     .restart()
    ///     .expect("Failed to restart");
    /// ```
    fn restart(&self) -> Result<Output> {
        self.wineboot_command()
            .arg("-r")
            .envs(self.get_envs())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    }

    /// Imitate windows shutdown. Runs `wineboot -s` command
    /// 
    /// ```no_run
    /// use wincompatlib::prelude::*;
    /// 
    /// Wine::default()
    ///     .with_prefix("/path/to/prefix")
    ///     .shutdown()
    ///     .expect("Failed to shutdown");
    /// ```
    fn shutdown(&self) -> Result<Output> {
        self.wineboot_command()
            .arg("-s")
            .envs(self.get_envs())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    }

    /// End wineboot session. Runs `wineboot -e` command
    /// 
    /// ```no_run
    /// use wincompatlib::prelude::*;
    /// 
    /// Wine::default()
    ///     .with_prefix("/path/to/prefix")
    ///     .end_session()
    ///     .expect("Failed to shutdown");
    /// ```
    fn end_session(&self) -> Result<Output> {
        self.wineboot_command()
            .arg("-e")
            .envs(self.get_envs())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    }
}
