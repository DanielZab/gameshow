use tokio_rustls::rustls::ServerConfig;
use std::fs::File;
use std::io::BufReader;
use rustls_pemfile::{certs, pkcs8_private_keys};
use rustls_pki_types::{CertificateDer, PrivatePkcs8KeyDer, PrivateKeyDer};


pub fn load_tls_config() -> ServerConfig {
    let cert_file = &mut BufReader::new(File::open("cert.pem").expect("Cannot open cert.pem"));
    let key_file = &mut BufReader::new(File::open("key.pem").expect("Cannot open key.pem"));

    let cert_chain: Vec<CertificateDer<'static>> = certs(cert_file)
        .expect("Cannot read certs")
        .into_iter()
        .map(CertificateDer::from)
        .collect();

    let mut keys: Vec<PrivatePkcs8KeyDer<'static>> = pkcs8_private_keys(key_file)
        .expect("Cannot read private key")
        .into_iter()
        .map(PrivatePkcs8KeyDer::from)
        .collect();

    assert!(!keys.is_empty(), "No private keys found");
    let key: PrivateKeyDer = PrivateKeyDer::from(keys.remove(0));

    ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .expect("Invalid TLS config")
}