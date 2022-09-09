mod config_utils;

use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use openssl::sign::Signer;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::hash::MessageDigest;
use crate::config_utils::KeyType;

fn main() {
    // Read server address from .conf file
    let (server_url, username, priv_keyfile, keytype) =
        config_utils::get_config()
            .expect("Unable to read .conf");
    if server_url.is_empty() {
        panic!("SERVER_URL not found in .conf");
    }

    // Connect to server
    let mut stream = TcpStream::connect(server_url)
        .expect("Unable to connect to server");

    // Do handshake step 1: Send username
    stream.write(username.as_bytes())
        .expect("Unable to send username to server");

    // Do handshake step 2: Receive challenge which is a 32-byte random string
    let mut challenge = [0; 32];
    stream.read(&mut challenge)
        .expect("Unable to receive challenge from server");

    // Do handshake step 3: Send signed challenge
    let signed_challenge = sign_challenge(&challenge, &priv_keyfile, &keytype);
    stream.write(signed_challenge.as_slice())
        .expect("Unable to send signed challenge to server");

    // Do handshake step 4: Receive response which is "OK" if the challenge is verified
    let mut response = [0; 2];
    stream.read(&mut response)
        .expect("Unable to receive response from server");

    if response == [79, 75] {
        println!("Handshake successful");
    } else {
        panic!("Handshake failed");
    }
}

fn sign_challenge(challenge: &[u8; 32], keyfile: &String, keytype: &KeyType) -> Vec<u8> {
    // Load private key from key file
    let key = fs::read_to_string(keyfile)
        .expect("Unable to read key file");

    // Sign challenge
    match  keytype {
        KeyType::RSA => {
            // RSA
            let rsa = Rsa::private_key_from_pem(key.as_bytes())
                .expect("Unable to load RSA private key");
            let pkey = PKey::from_rsa(rsa).unwrap();
            let mut signer = Signer::new(MessageDigest::sha256(), &pkey).unwrap();
            signer.update(challenge).unwrap();
            let signature = signer.sign_to_vec().unwrap();
            return signature;
        },
        _ => panic!("Invalid key type"),
    }

}