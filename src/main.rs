use std::process::Command;

mod config_utils;


fn main() {
    let mut output = Command::new("xcrun")
        .arg("xcodebuild")
        .arg("clean")
        .arg("install")
        .current_dir("notification-helper/")
        .output()
        .expect("failed to execute process");
    if !output.status.success() {
        println!("Error: {}", String::from_utf8_lossy(&output.stdout));
        panic!("Command failed. Do you have xcode installed?");
    }
    let (server_url, username, priv_keyfile, keytype) =
        match config_utils::get_config() {
            Ok((server_url, username, priv_keyfile, keytype)) =>
                (server_url, username, priv_keyfile, keytype),
            Err(_) => config_utils::create_valid_config(),
        };
}