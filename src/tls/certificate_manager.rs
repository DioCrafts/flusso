// src/tls/certificate_manager.rs

use rustls::{Certificate, PrivateKey};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

pub fn load_certificates(cert_path: &str, key_path: &str) -> io::Result<(Certificate, PrivateKey)> {
    // Carga el certificado
    let cert_file = File::open(Path::new(cert_path))?;
    let mut cert_reader = BufReader::new(cert_file);
    let certs = rustls_pemfile::certs(&mut cert_reader)?;
    let cert = Certificate(certs[0].clone());

    // Carga la clave privada
    let key_file = File::open(Path::new(key_path))?;
    let mut key_reader = BufReader::new(key_file);
    let keys = rustls_pemfile::pkcs8_private_keys(&mut key_reader)
        .or_else(|_| rustls_pemfile::rsa_private_keys(&mut key_reader))?;

    let key = PrivateKey(keys[0].clone());

    Ok((cert, key))
}

// Función para renovar certificados
pub fn renew_certificate(cert_path: &str, key_path: &str) -> Result<(Certificate, PrivateKey), String> {
    load_certificates(cert_path, key_path).map_err(|e| format!("Failed to renew certificate: {}", e))
}
