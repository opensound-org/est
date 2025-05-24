use std::{ffi::OsStr, process::Command as StdCommand};
use tokio::process::Command as TokioCommand;

/// An enum that can wrap [`std::process::Command`] or [`tokio::process::Command`] and can `Clone`.
///
/// Note that Cloning `Command` is a lossy clone, which will lose platform specific options such as:
/// [`pre_exec`](https://doc.rust-lang.org/stable/std/process/struct.Command.html#method.pre_exec),
/// [`creation_flags`](https://doc.rust-lang.org/stable/std/process/struct.Command.html#method.creation_flags),
/// etc.
///
/// See: <https://users.rust-lang.org/t/is-there-any-way-to-clone-a-std-command/121905>
#[derive(Debug)]
pub enum Command {
    Std(StdCommand),
    Tokio(TokioCommand),
}

impl Command {
    /// Create a new `Command` wrapped with [`std::process::Command`].
    ///
    /// See: [`std::process::Command::new`]
    pub fn std<S: AsRef<OsStr>>(program: S) -> Self {
        StdCommand::new(program).into()
    }

    /// Create a new `Command` wrapped with [`tokio::process::Command`].
    ///
    /// See: [`tokio::process::Command::new`]
    pub fn tokio_default<S: AsRef<OsStr>>(program: S) -> Self {
        TokioCommand::new(program).into()
    }

    /// Create a new `Command` wrapped with [`tokio::process::Command`] with [`kill_on_drop`] option.
    ///
    /// [`kill_on_drop`]: tokio::process::Command::kill_on_drop
    ///
    /// See:
    /// - [`tokio::process::Command::new`]
    /// - [`tokio::process::Command::kill_on_drop`]
    pub fn tokio_config<S: AsRef<OsStr>>(program: S, kill_on_drop: bool) -> Self {
        let mut cmd = TokioCommand::new(program);

        if kill_on_drop {
            cmd.kill_on_drop(true);
        }

        cmd.into()
    }

    /// Check whether `Self` is wrapped with [`std::process::Command`].
    pub fn wrapping_std(&self) -> bool {
        matches!(self, Self::Std(_))
    }

    /// Check whether `Self` is wrapped with [`tokio::process::Command`].
    pub fn wrapping_tokio(&self) -> bool {
        matches!(self, Self::Tokio(_))
    }

    /// Cheaply convert to a `&std::process::Command` for places where the type from the standard
    /// library is expected.
    ///
    /// This method will return `&std::process::Command` even if it is wrapping
    /// [`tokio::process::Command`].
    ///
    /// See: [`tokio::process::Command::as_std`]
    pub fn as_std(&self) -> &StdCommand {
        match self {
            Self::Std(v) => v,
            Self::Tokio(v) => v.as_std(),
        }
    }

    /// Cheaply convert to a `&mut std::process::Command` for places where the type from the
    /// standard library is expected.
    ///
    /// This method will return `&mut std::process::Command` even if it is wrapping
    /// [`tokio::process::Command`].
    ///
    /// See: [`tokio::process::Command::as_std_mut`]
    pub fn as_std_mut(&mut self) -> &mut StdCommand {
        match self {
            Self::Std(v) => v,
            Self::Tokio(v) => v.as_std_mut(),
        }
    }

    /// If the instance is wrapping [`tokio::process::Command`], it returns
    /// `Some(&tokio::process::Command)`, otherwise it returns `None`.
    pub fn as_tokio(&self) -> Option<&TokioCommand> {
        match self {
            Self::Std(_) => None,
            Self::Tokio(v) => Some(v),
        }
    }

    /// If the instance is wrapping [`tokio::process::Command`], it returns
    /// `Some(&mut tokio::process::Command)`, otherwise it returns `None`.
    pub fn as_tokio_mut(&mut self) -> Option<&mut TokioCommand> {
        match self {
            Self::Std(_) => None,
            Self::Tokio(v) => Some(v),
        }
    }

    /// Cheaply convert into a [`std::process::Command`].
    ///
    /// Note that if the instance is wrapping [`tokio::process::Command`], Tokio specific options
    /// will be lost. Currently, this only applies to [`kill_on_drop`].
    ///
    /// [`kill_on_drop`]: tokio::process::Command::kill_on_drop
    ///
    /// See: [`tokio::process::Command::into_std`]
    pub fn into_std(self) -> StdCommand {
        self.into()
    }

    /// Cheaply convert into a [`tokio::process::Command`].
    ///
    /// Note that if the instance is wrapping the [`std::process::Command`], [`kill_on_drop`] will
    /// use the default value of `false`.
    ///
    /// [`kill_on_drop`]: tokio::process::Command::kill_on_drop
    pub fn into_tokio(self) -> TokioCommand {
        self.into()
    }

    /// Consume `Self`, convert it to [`std::process::Command`], and then return a new instance
    /// that wraps it.
    pub fn convert_to_std(self) -> Self {
        self.into_std().into()
    }

    /// Consume `Self`, convert it to [`tokio::process::Command`], and then return a new instance
    /// that wraps it.
    pub fn convert_to_tokio(self) -> Self {
        self.into_tokio().into()
    }
}

impl From<StdCommand> for Command {
    fn from(value: StdCommand) -> Self {
        Self::Std(value)
    }
}

impl From<Command> for StdCommand {
    fn from(value: Command) -> Self {
        match value {
            Command::Std(v) => v,
            Command::Tokio(v) => v.into_std(),
        }
    }
}

impl From<TokioCommand> for Command {
    fn from(value: TokioCommand) -> Self {
        Self::Tokio(value)
    }
}

impl From<Command> for TokioCommand {
    fn from(value: Command) -> Self {
        match value {
            Command::Std(v) => v.into(),
            Command::Tokio(v) => v,
        }
    }
}

impl Clone for Command {
    fn clone(&self) -> Self {
        match self {
            Self::Std(std_cmd) => {
                // Direct cloning for std::process::Command
                let mut cloned = StdCommand::new(std_cmd.get_program());
                cloned.args(std_cmd.get_args());
                
                // Batch process environment variables
                cloned.envs(std_cmd.get_envs().filter_map(|(k, v)| v.map(|v| (k, v))));
                for (k, _) in std_cmd.get_envs().filter(|(_, v)| v.is_none()) {
                    cloned.env_remove(k);
                }

                if let Some(current_dir) = std_cmd.get_current_dir() {
                    cloned.current_dir(current_dir);
                }

                Self::Std(cloned)
            }
            Self::Tokio(tokio_cmd) => {
                // For tokio commands, preserve kill_on_drop setting
                let kill_on_drop = tokio_cmd.get_kill_on_drop();
                let std_cmd = tokio_cmd.as_std();
                
                let mut cloned = StdCommand::new(std_cmd.get_program());
                cloned.args(std_cmd.get_args());
                
                // Batch process environment variables
                cloned.envs(std_cmd.get_envs().filter_map(|(k, v)| v.map(|v| (k, v))));
                for (k, _) in std_cmd.get_envs().filter(|(_, v)| v.is_none()) {
                    cloned.env_remove(k);
                }

                if let Some(current_dir) = std_cmd.get_current_dir() {
                    cloned.current_dir(current_dir);
                }

                let mut tokio_cloned: TokioCommand = cloned.into();
                if kill_on_drop {
                    tokio_cloned.kill_on_drop(true);
                }

                Self::Tokio(tokio_cloned)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapping() {
        let cmd = Command::Std(StdCommand::new("echo"));
        assert!(cmd.wrapping_std());
        assert!(!cmd.wrapping_tokio());

        let cmd = Command::Tokio(TokioCommand::new("echo"));
        assert!(!cmd.wrapping_std());
        assert!(cmd.wrapping_tokio());
    }

    #[test]
    fn construct() {
        let cmd = Command::std("echo");
        assert!(cmd.wrapping_std());

        let cmd = Command::tokio_default("echo");
        assert!(cmd.wrapping_tokio());
        assert!(!cmd.as_tokio().unwrap().get_kill_on_drop());

        let cmd = Command::tokio_config("echo", false);
        assert!(cmd.wrapping_tokio());
        assert!(!cmd.as_tokio().unwrap().get_kill_on_drop());

        let cmd = Command::tokio_config("echo", true);
        assert!(cmd.wrapping_tokio());
        assert!(cmd.as_tokio().unwrap().get_kill_on_drop());
    }

    #[test]
    fn as_std() {
        let mut cmd = Command::std("echo");
        assert_eq!(cmd.as_std().get_program(), "echo");
        assert_eq!(cmd.as_std_mut().get_program(), "echo");

        let mut cmd = Command::tokio_default("echo");
        assert_eq!(cmd.as_std().get_program(), "echo");
        assert_eq!(cmd.as_std_mut().get_program(), "echo");
    }

    #[test]
    fn as_tokio() {
        let mut cmd = Command::std("echo");
        assert!(cmd.as_tokio().is_none());
        assert!(cmd.as_tokio_mut().is_none());

        let mut cmd = Command::tokio_default("echo");
        assert!(cmd.as_tokio().is_some());
        assert!(cmd.as_tokio_mut().is_some());
    }

    #[test]
    fn into_std() {
        let cmd = Command::std("echo").into_std();
        assert_eq!(cmd.get_program(), "echo");
        let cmd: TokioCommand = cmd.into();
        assert!(!cmd.get_kill_on_drop());

        let cmd = Command::tokio_default("echo").into_std();
        assert_eq!(cmd.get_program(), "echo");
        let cmd: TokioCommand = cmd.into();
        assert!(!cmd.get_kill_on_drop());

        let cmd = Command::tokio_config("echo", true).into_std();
        assert_eq!(cmd.get_program(), "echo");
        let cmd: TokioCommand = cmd.into();
        assert!(!cmd.get_kill_on_drop());
    }

    #[test]
    fn into_tokio() {
        let cmd = Command::std("echo").into_tokio();
        assert_eq!(cmd.as_std().get_program(), "echo");
        assert!(!cmd.get_kill_on_drop());

        let cmd = Command::tokio_default("echo").into_tokio();
        assert_eq!(cmd.as_std().get_program(), "echo");
        assert!(!cmd.get_kill_on_drop());

        let cmd = Command::tokio_config("echo", true).into_tokio();
        assert_eq!(cmd.as_std().get_program(), "echo");
        assert!(cmd.get_kill_on_drop());
    }

    #[test]
    fn convert() {
        let cmd = Command::std("echo");
        assert!(cmd.wrapping_std());
        let cmd = cmd.convert_to_std();
        assert!(cmd.wrapping_std());

        let cmd = Command::tokio_default("echo");
        assert!(cmd.wrapping_tokio());
        let cmd = cmd.convert_to_std();
        assert!(cmd.wrapping_std());

        let cmd = Command::std("echo");
        assert!(cmd.wrapping_std());
        let cmd = cmd.convert_to_tokio();
        assert!(cmd.wrapping_tokio());

        let cmd = Command::tokio_default("echo");
        assert!(cmd.wrapping_tokio());
        let cmd = cmd.convert_to_tokio();
        assert!(cmd.wrapping_tokio());
    }

    fn std_command() -> StdCommand {
        let mut cmd = StdCommand::new("echo");
        cmd.args(["a1", "a2"]);
        cmd.envs([("k1", "v1"), ("k2", "v2")]);
        cmd.current_dir("/tmp");
        cmd
    }

    fn eq_std(a: &StdCommand, b: &StdCommand) -> bool {
        a.get_program() == b.get_program()
            && a.get_args().collect::<Vec<_>>() == b.get_args().collect::<Vec<_>>()
            && a.get_envs().collect::<Vec<_>>() == b.get_envs().collect::<Vec<_>>()
            && a.get_current_dir() == b.get_current_dir()
    }

    fn eq_tokio(a: &TokioCommand, b: &TokioCommand) -> bool {
        eq_std(&a.as_std(), &b.as_std()) && a.get_kill_on_drop() == b.get_kill_on_drop()
    }

    fn eq_command(a: &Command, b: &Command) -> bool {
        match (a, b) {
            (Command::Std(a), Command::Std(b)) => eq_std(a, b),
            (Command::Tokio(a), Command::Tokio(b)) => eq_tokio(a, b),
            _ => false,
        }
    }

    #[test]
    fn clone() {
        let cmd: Command = std_command().into();
        let cloned = cmd.clone();
        assert!(eq_command(&cmd, &cloned));

        let cmd: TokioCommand = std_command().into();
        let cmd: Command = cmd.into();
        let cloned = cmd.clone();
        assert!(eq_command(&cmd, &cloned));

        let mut cmd: TokioCommand = std_command().into();
        cmd.kill_on_drop(false);
        let cmd: Command = cmd.into();
        let cloned = cmd.clone();
        assert!(eq_command(&cmd, &cloned));

        let mut cmd: TokioCommand = std_command().into();
        cmd.kill_on_drop(true);
        let cmd: Command = cmd.into();
        let cloned = cmd.clone();
        assert!(eq_command(&cmd, &cloned));
    }
}
