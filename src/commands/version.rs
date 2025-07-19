use rgamesync_rclone::{run_rclone, RCloneError};



pub fn run_version() -> Result<(), RCloneError> {
    run_rclone(["version"])?;
    // TODO: add macro to get crate version.
    println!("rgamesync v{}", "0.1.0");
    Ok(())
}