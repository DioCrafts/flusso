use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

pub fn load_certificates(cert_path: &str, key_path: &str) -> io::Result<(CertificateDer<'static>, PrivateKeyDer<'static>)> {
    // Carga el certificado
    let cert_file = File::open(Path::new(cert_path))?;
    let mut cert_reader = BufReader::new(cert_file);
    let certs = rustls_pemfile::certs(&mut cert_reader)?;
    let cert_bytes = certs[0].clone();
    let cert = CertificateDer::from(cert_bytes);

    // Carga la clave privada
    let key_file = File::open(Path::new(key_path))?;
    let mut key_reader = BufReader::new(key_file);
    let keys = rustls_pemfile::pkcs8_private_keys(&mut key_reader)
        .or_else(|_| rustls_pemfile::rsa_private_keys(&mut key_reader))?;

    let key_bytes = keys[0].clone();
    let private_key_der = PrivatePkcs8KeyDer::from(key_bytes);
    let key = PrivateKeyDer::Pkcs8(private_key_der);

    Ok((cert, key))
}

// Función para renovar certificados
pub fn renew_certificate(cert_path: &str, key_path: &str) -> Result<(CertificateDer<'static>, PrivateKeyDer<'static>), String> {
    load_certificates(cert_path, key_path).map_err(|e| format!("Failed to renew certificate: {}", e))
}

