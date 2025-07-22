use rgamesync_rclone::{RCloneError, run_rclone};



pub fn run_version() -> Result<(), RCloneError> {
    run_rclone(["version"])?;
    // TODO: add macro to get crate version.
    println!("rgamesync v0.1.0");
    Ok(())
}
