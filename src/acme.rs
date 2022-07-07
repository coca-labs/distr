use acme_micro::create_rsa_key;
use acme_micro::{Certificate, Directory, DirectoryUrl, Error};
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcmeInfo {
    pub domain: String,
    pub email: String,
    pub web_root: String,
}

pub fn request_cert(info: &AcmeInfo) -> Result<Certificate, Error> {
    // Use DirectoryUrl::LetsEncrypStaging for dev/testing.
    let url = DirectoryUrl::LetsEncrypt;

    // Create a directory entrypoint.
    let dir = Directory::from_url(url)?;

    // Your contact addresses, note the `mailto:`
    let contact = vec![format!("mailto:{}", info.email)];

    // Generate a private key and register an account with your ACME provider.
    // You should write it to disk any use `load_account` afterwards.
    let acc = dir.register_account(contact.clone())?;

    // Example of how to load an account from string:
    let privkey = acc.acme_private_key_pem()?;
    let acc = dir.load_account(&privkey, contact)?;

    // Order a new TLS certificate for a domain.
    let mut ord_new = acc.new_order(&info.domain, &[])?;

    // If the ownership of the domain(s) have already been
    // authorized in a previous order, you might be able to
    // skip validation. The ACME API provider decides.
    let ord_csr = loop {
        // are we done?
        if let Some(ord_csr) = ord_new.confirm_validations() {
            break ord_csr;
        }

        // Get the possible authorizations (for a single domain
        // this will only be one element).
        let auths = ord_new.authorizations()?;

        // For HTTP, the challenge is a text file that needs to
        // be placed in your web server's root:
        //
        // /var/www/.well-known/acme-challenge/<token>
        //
        // The important thing is that it's accessible over the
        // web for the domain(s) you are trying to get a
        // certificate for:
        //
        // http://mydomain.io/.well-known/acme-challenge/<token>
        let chall = auths[0].http_challenge().unwrap();

        let dir = format!("{}/.well-known/acme-challenge", info.web_root);
        std::fs::create_dir_all(&dir)?;

        // The token is the filename.
        let token = chall.http_token();
        let path = format!("{}/.well-known/acme-challenge/{}", info.web_root, token);

        // The proof is the contents of the file
        let proof = chall.http_proof()?;
    
        // Here you must do "something" to place
        // the file/contents in the correct place.
        // update_my_web_server(&path, &proof);
        {
            println!("{}", path);
            let mut file = File::create(&path)?;
            println!("{:?}", file);
            file.write_all(proof.as_bytes())?;
            file.sync_all()?;
        }

        // After the file is accessible from the web, the calls
        // this to tell the ACME API to start checking the
        // existence of the proof.
        //
        // The order at ACME will change status to either
        // confirm ownership of the domain, or fail due to the
        // not finding the proof. To see the change, we poll
        // the API with 5000 milliseconds wait between.
        chall.validate(Duration::from_millis(5000))?;

        // Update the state against the ACME API.
        ord_new.refresh()?;
    };

    // Ownership is proven. Create a private key for
    // the certificate. These are provided for convenience, you
    // can provide your own keypair instead if you want.
    let pkey_pri = create_rsa_key(2048)?;

    // Submit the CSR. This causes the ACME provider to enter a
    // state of "processing" that must be polled until the
    // certificate is either issued or rejected. Again we poll
    // for the status change.
    let ord_cert = ord_csr.finalize_pkey(pkey_pri, Duration::from_millis(5000))?;

    // Finally download the certificate.
    let cert = ord_cert.download_cert()?;
    println!("{:?}", cert);

    Ok(cert)
}
