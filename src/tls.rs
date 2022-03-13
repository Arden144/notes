use crate::config::*;

use std::{error::Error, fs::File, io::BufReader};

use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

pub fn get_tls_config() -> Result<ServerConfig, Box<dyn Error>> {
    let mut cert_file = BufReader::new(File::open(CERT)?);
    let mut key_file = BufReader::new(File::open(KEY)?);

    let cert_chain: Vec<Certificate> = certs(&mut cert_file)?
        .into_iter()
        .map(Certificate)
        .collect();
    assert!(!cert_chain.is_empty(), "Could not get certificate.");

    let key_der = pkcs8_private_keys(&mut key_file)?
        .into_iter()
        .map(PrivateKey)
        .next()
        .expect("Could not get private key.");

    Ok(ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key_der)?)
}
