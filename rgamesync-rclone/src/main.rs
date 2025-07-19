use rgamesync_rclone::run_rclone;



fn main() {
    run_rclone(["version"]).expect("runs");
}
