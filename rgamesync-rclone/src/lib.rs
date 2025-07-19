use std::ffi::OsStr;
use std::process::{Command, ExitStatus};

use thiserror::Error;



#[derive(Error, Debug)]
pub enum RCloneError {
    #[error("Failed to spawn child process: {0}")]
    CouldNotSpawn(std::io::Error),
    #[error("Child process failed: {0}")]
    UnexpectedClose(std::io::Error),
    #[error("Child process failed: {0}")]
    UnexpectedStatusCode(ExitStatus),
}

pub fn run_rclone<
    ArgIter: IntoIterator<Item = Arg>,
    Arg: AsRef<OsStr>,
>(
    args: ArgIter
) -> Result<(), RCloneError> {
    let mut child = Command::new("rclone")
        .args(args)
        .spawn()
        .map_err(|err| RCloneError::CouldNotSpawn(err))?;

    let status = child.wait()
        .map_err(|err| RCloneError::UnexpectedClose(err))?;

    if !status.success() {
        Err(RCloneError::UnexpectedStatusCode(status))
    } else {
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_rclone_version() {
        run_rclone(["version"])
            .expect("runs `rclone version` successfully");
    }
    #[test]
    fn errors_on_invalid_command() {
        let res = run_rclone(["this-does-not-exist"]);
        assert!(res.is_err());
        assert!(matches!(res, Err(RCloneError::UnexpectedStatusCode(_))));
    }
}
