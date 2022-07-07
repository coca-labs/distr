mod acme;
mod cert;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process;

use ntex::web::{self, middleware, App};
use ntex_files as fs;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, rsa_private_keys};

#[ntex::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "ntex=info");
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: distr {{domain}} {{email}}");
        process::exit(0x0100);
    }
    println!("{:?}", args);

    let domain = args[1].clone();
    let email = args[2].clone();

    let res = cert::test_domain_exists(&domain);
    let exists = match res {
        Ok(e) => e,
        Err(err) => {
            println!("{}", err);
        }
    };
    println!("exists: {}", exists);

    if !exists {
        ntex::rt::spawn(async move {
            println!("Request cert...");
            let res = acme::request_cert(&acme::AcmeInfo {
                domain: domain.to_owned(),
                email: email.to_owned(),
                web_root: "./static".to_owned(),
            });
            match res {
                Ok(crt) => match cert::create_cert_file(&domain, &crt) {
                    Ok(_) => println!("Cert saved."),
                    Err(err) => println!("{}", err),
                },
                Err(err) => {
                    println!("{}", err);
                    // process::exit(0x0100);
                }
            }
        });

        web::server(|| {
            App::new()
                // enable logger
                .wrap(middleware::Logger::default())
                .service(
                    // static files
                    fs::Files::new("/", "./static/").index_file("index.html"),
                )
        })
        .bind("0.0.0.0:80")?
        .run()
        .await
    } else {
        // load ssl keys
        let key_path = format!("./cert/{}.key", domain);
        let cert_path = format!("./cert/{}.pem", domain);
        let key_file = &mut BufReader::new(File::open(&key_path).unwrap());
        let key = PrivateKey(rsa_private_keys(key_file).unwrap().remove(0));
        let cert_file = &mut BufReader::new(File::open(&cert_path).unwrap());
        let cert_chain = certs(cert_file)
            .unwrap()
            .iter()
            .map(|c| Certificate(c.to_vec()))
            .collect();
        let config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(cert_chain, key)
            .unwrap();

        web::server(|| {
            App::new()
                // enable logger
                .wrap(middleware::Logger::default())
                .service(
                    // static files
                    fs::Files::new("/", "./static/").index_file("index.html"),
                )
        })
        .bind("0.0.0.0:80")?
        .bind_rustls("0.0.0.0:443", config)?
        .run()
        .await
    }
}
