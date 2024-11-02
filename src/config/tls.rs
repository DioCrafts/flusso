// src/config/tls.rs

use std::fs::File;
use std::io::{self, BufReader};
use rustls::{Certificate, PrivateKey};
use rustls_pemfile::{certs, rsa_private_keys};

pub struct TlsConfig {
    pub certs: Vec<Certificate>,
    pub key: PrivateKey,
}

impl TlsConfig {
    /// Carga certificados y claves privadas desde los archivos especificados
    pub fn load(cert_path: &str, key_path: &str) -> io::Result<Self> {
        let certs = load_certs(cert_path)?;
        let key = load_private_key(key_path)?;
        Ok(Self { certs, key })
    }
}

/// Carga los certificados desde el archivo
fn load_certs(path: &str) -> io::Result<Vec<Certificate>> {
    let cert_file = File::open(path)?;
    let mut reader = BufReader::new(cert_file);
    let certs = certs(&mut reader)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid certificate"))?;
    Ok(certs.into_iter().map(Certificate).collect())
}

/// Carga la clave privada desde el archivo
fn load_private_key(path: &str) -> io::Result<PrivateKey> {
    let key_file = File::open(path)?;
    let mut reader = BufReader::new(key_file);
    let keys = rsa_private_keys(&mut reader)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid private key"))?;
    if let Some(key) = keys.into_iter().next() {
        Ok(PrivateKey(key))
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidData, "No private key found"))
    }
}
