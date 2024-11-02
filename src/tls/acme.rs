// src/tls/acme.rs

use acme_lib::{Account, Directory, DirectoryUrl, Order, persist::FilePersist};
use std::fs;
use std::path::Path;
use std::time::Duration;
use std::thread;
use log::{info, error};

pub struct AcmeClient {
    account: Account<FilePersist>,
}

impl AcmeClient {
    pub fn new(email: &str, persist_path: &str) -> Self {
        let persist = FilePersist::new(persist_path);
        let url = DirectoryUrl::LetsEncrypt;
        let dir = Directory::from_url(persist, url).unwrap();
        let account = dir.account(email).unwrap_or_else(|_| dir.new_account(email, true).unwrap());

        AcmeClient { account }
    }

    pub fn request_certificate(&self, domain: &str) -> Result<(String, String), String> {
        let order = self.account.new_order(domain).map_err(|e| e.to_string())?;
        
        let auths = order.authorizations().map_err(|e| e.to_string())?;
        for auth in auths {
            let chall = auth.http_challenge().map_err(|e| e.to_string())?;
            let token = chall.http_token();
            let proof = chall.http_proof();
            self.publish_challenge(token, proof);

            chall.validate(Duration::from_secs(60)).map_err(|e| e.to_string())?;
        }

        let order_csr = order.finalize(&format!("C=US, ST=Some-State, O=Internet Widgits Pty Ltd, CN={}", domain))
            .map_err(|e| e.to_string())?;

        let cert = order_csr.download_cert().map_err(|e| e.to_string())?;
        let cert_path = format!("/etc/letsencrypt/live/{}/cert.pem", domain);
        let key_path = format!("/etc/letsencrypt/live/{}/privkey.pem", domain);

        fs::write(&cert_path, &cert).map_err(|e| e.to_string())?;
        fs::write(&key_path, &self.account.key().to_pem()).map_err(|e| e.to_string())?;

        Ok((cert_path, key_path))
    }

    fn publish_challenge(&self, token: &str, proof: &str) {
        let challenge_path = format!("/var/www/html/.well-known/acme-challenge/{}", token);
        fs::write(challenge_path, proof).expect("Failed to publish ACME challenge");
    }
}

pub fn renew_certificate(acme_client: &AcmeClient, domain: &str) {
    loop {
        match acme_client.request_certificate(domain) {
            Ok((cert, key)) => {
                info!("Certificate renewed: {}, key: {}", cert, key);
            }
            Err(e) => {
                error!("Failed to renew certificate: {}", e);
            }
        }
        thread::sleep(Duration::from_secs(60 * 60 * 24)); // Intenta renovar cada 24 horas
    }
}
