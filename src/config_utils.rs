use std::fs;
use aes::cipher::Key;
use openssl::rsa::Rsa;

pub enum KeyType {
    RSA,
    None,
}

pub fn get_config() -> Result<(String, String, String, KeyType), i32> {
    let conf = fs::read_to_string(".conf")
        .expect("Unable to read .conf");
    let mut server_url = String::new();
    let mut username = String::new();
    let mut priv_keyfile = String::new();
    let mut pub_keyfile = String::new();
    let mut keytype = KeyType::None;
    for line in conf.lines() {
        if line.starts_with("SERVER_URL") {
            server_url = line.split("=").nth(1).unwrap().to_string().trim().to_string();
        } else if line.starts_with("USERNAME") {
            username = line.split("=").nth(1).unwrap().to_string().trim().to_string();
        } else if line.starts_with("PRIV_KEYFILE") {
            priv_keyfile = line.split("=").nth(1).unwrap().to_string().trim().to_string();
        } else if line.starts_with("PUB_KEYFILE") {
            pub_keyfile = line.split("=").nth(1).unwrap().to_string().trim().to_string();
        } else if line.starts_with("KEYTYPE") {
            let str_keytype = line.split("=").nth(1).unwrap().to_string().trim().to_string();
            keytype = match str_keytype.as_str() {
                "RSA" => KeyType::RSA,
                _ => return Err(2),
            };
        }
    }
    // Verify that all fields are present
    if server_url.is_empty() ||
        username.is_empty() ||
        priv_keyfile.is_empty() ||
        pub_keyfile.is_empty() {
        return Err(1);
    }
    Ok((server_url, username, priv_keyfile, keytype))
}

pub fn create_valid_config() -> (String, String, String, KeyType) {
    // Ask user for server URL
    println!("Enter server URL:");
    let mut server_url = String::new();
    std::io::stdin().read_line(&mut server_url)
        .expect("Unable to read server URL");
    server_url = server_url.trim().to_string();

    // Ask user for username
    println!("Enter username:");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username)
        .expect("Unable to read username");
    username = username.trim().to_string();

    // Ask user for key type
    println!("Enter key type [RSA]:");
    let mut keytype = String::new();
    std::io::stdin().read_line(&mut keytype)
        .expect("Unable to read key type");
    keytype = keytype.trim().to_string();
    if keytype.is_empty() {
        keytype = "RSA".to_string();
    }
    let keytype = match keytype.as_str() {
        "RSA" => KeyType::RSA,
        _ => panic!("Invalid key type"),
    };

    // Ask user if they want to generate a new key pair or use an existing one
    println!("Generate new (pem) key pair? [y/N]:");
    let mut generate_key = String::new();
    std::io::stdin().read_line(&mut generate_key)
        .expect("Unable to read generate key");
    generate_key = generate_key.trim().to_string();
    let generate_key = match generate_key.as_str() {
        "y" => true,
        "Y" => true,
        _ => false,
    };

    // Generate new key pair or load existing key pair
    let (priv_keyfile, pub_keyfile) = match generate_key {
        true => {
            // Generate new key pair
            let (priv_keyfile, pub_keyfile) = match keytype {
                KeyType::RSA => {
                    // RSA
                    let rsa = Rsa::generate(2048)
                        .expect("Unable to generate RSA key pair");
                    let priv_keyfile = "priv_key.pem".to_string();
                    let pub_keyfile = "pub_key.pem".to_string();
                    fs::write(&priv_keyfile, rsa.private_key_to_pem().unwrap())
                        .expect("Unable to write private key to file");
                    fs::write(&pub_keyfile, rsa.public_key_to_pem().unwrap())
                        .expect("Unable to write public key to file");
                    (priv_keyfile, pub_keyfile)
                },
                _ => panic!("Invalid key type"),
            };
            (priv_keyfile, pub_keyfile)
        },
        false => {
            // Load existing key pair
            println!("Enter private key file:");
            let mut priv_keyfile = String::new();
            std::io::stdin().read_line(&mut priv_keyfile)
                .expect("Unable to read private key file");
            priv_keyfile = priv_keyfile.trim().to_string();
            println!("Enter public key file:");
            let mut pub_keyfile = String::new();
            std::io::stdin().read_line(&mut pub_keyfile)
                .expect("Unable to read public key file");
            pub_keyfile = pub_keyfile.trim().to_string();
            (priv_keyfile, pub_keyfile)
        },
    };

    // Write config file
    let mut config = String::new();
    for (key, value) in vec![
        ("server_url", server_url.clone()),
        ("username", username.clone()),
        ("priv_keyfile", priv_keyfile.clone()),
        ("pub_keyfile", pub_keyfile.clone()),
        ("keytype", match keytype {
            KeyType::RSA => "RSA".to_string(),
            _ => panic!("Invalid key type"),
        }),
    ] {
       config.push_str(&format!("{}={}", key, value));
    }
    fs::write(".conf1", config)
        .expect("Unable to write config file");
    return (server_url, username, "".to_string(), keytype);
}
