// use acme_micro::create_p384_key;
// use acme_micro::{Certificate, Directory, DirectoryUrl, Error};
use acme_micro::Certificate;
use std::fs::File;
use std::io::prelude::*;
// use std::time::Duration;
// use anyhow::Result;
// use serde::{Serialize, Deserialize};

pub fn test_domain_exists(domain: &str) -> std::io::Result<bool>  {
    let path = format!("./cert/{}.key", domain);
    println!("{}", path);

    let res = File::open(path);
    println!("{:?}", res);

    let exists = match res {
        Ok(_) => true,
        Err(_) => false,
    };
    Ok(exists)
}

pub fn create_cert_file(domain: &str, cert: &Certificate) -> std::io::Result<()> {
    let dir = format!("./cert");
    std::fs::create_dir_all(&dir)?;

    let path = format!("./cert/{}.key", domain);
    println!("{}", path);

    {
        let mut file = File::create(path)?;
        file.write_all(cert.private_key().as_bytes())?;
        file.sync_all()?;
    }

    let path = format!("./cert/{}.pem", domain);
    println!("{}", path);

    {
        let mut file = File::create(path)?;
        file.write_all(cert.certificate().as_bytes())?;
        file.sync_all()?;
    }

    Ok(())
}


mod tests {
    // use acme_micro::Certificate;

    #[test]
    fn test_test_domain_exists() {
        let res = super::test_domain_exists("clia.tech");
        println!("{:?}", res);
    }

    // #[test]
    // fn test_create_cert_file() {
    //     let res = super::create_cert_file("clia.tech", &Certificate::new("private_key: String".to_owned(), "certificate: String".to_owned()));
    // }
}