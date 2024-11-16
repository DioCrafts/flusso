// src/tls/acme.rs

use reqwest::Client;
use serde::{Deserialize, Serialize};
use rustls::pki_types::CertificateDer;
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use tokio::time::sleep;
use log::{info, error};
use anyhow::Result;

#[derive(Serialize, Deserialize)]
struct Account {
    contact: Vec<String>,
    terms_of_service_agreed: bool,
}

#[derive(Serialize, Deserialize)]
struct CertificateRequest {
    csr: String, // Debes generar el CSR y codificarlo en Base64
}

pub struct AcmeClient {
    client: Client,
}

impl AcmeClient {
    pub fn new() -> Self {
        AcmeClient {
            client: Client::new(),
        }
    }

    pub async fn register_account(&self, email: &str) -> Result<()> {
        let account_data = Account {
            contact: vec![format!("mailto:{}", email)],
            terms_of_service_agreed: true,
        };

        let response = self.client
            .post("https://acme-staging-v02.api.letsencrypt.org/acme/new-acct")
            .json(&account_data)
            .send()
            .await?;

        if response.status().is_success() {
            info!("Account successfully registered!");
        } else {
            error!("Error registering account: {:?}", response.text().await?);
        }
        Ok(())
    }

    pub async fn request_certificate(&self, csr: &str) -> Result<Vec<u8>> {
        let cert_req = CertificateRequest {
            csr: csr.to_string(),
        };

        let response = self.client
            .post("https://acme-staging-v02.api.letsencrypt.org/acme/new-cert")
            .json(&cert_req)
            .send()
            .await?;

        if response.status().is_success() {
            info!("Certificate issued successfully!");
            let cert_der = response.bytes().await?.to_vec(); // Convertimos a Vec<u8> directamente
            Ok(cert_der)
        } else {
            error!("Error issuing certificate: {:?}", response.text().await?);
            anyhow::bail!("Error issuing certificate");
        }
    }

    pub fn save_certificate(&self, cert: &[u8], domain: &str) -> std::io::Result<()> {
        let cert_path = format!("/etc/letsencrypt/live/{}/cert.pem", domain);
        let mut cert_file = File::create(&cert_path)?; // Clonamos cert_path
        cert_file.write_all(cert)?; // Escribimos directamente el array de bytes
        info!("Certificate saved to {}", cert_path);
        Ok(())
    }
}

pub async fn renew_certificate(acme_client: &AcmeClient, domain: &str, csr: &str) {
    loop {
        match acme_client.request_certificate(csr).await {
            Ok(cert) => {
                if let Err(e) = acme_client.save_certificate(&cert, domain) {
                    error!("Failed to save certificate: {}", e);
                } else {
                    info!("Certificate renewed for domain: {}", domain);
                }
            }
            Err(e) => {
                error!("Failed to renew certificate: {}", e);
            }
        }
        sleep(Duration::from_secs(60 * 60 * 24)).await; // Intenta renovar cada 24 horas
    }
}

