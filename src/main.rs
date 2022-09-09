mod config_utils;

fn main() {
    let (server_url, username, priv_keyfile, keytype) =
        match config_utils::get_config() {
            Ok((server_url, username, priv_keyfile, keytype)) => (server_url, username, priv_keyfile, keytype),
            Err(_) => config_utils::create_valid_config(),
        };
}
