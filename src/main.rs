mod acme;
mod cert;

use std::env;
use std::process;

use ntex::web::{self, middleware, App};
use ntex_files as fs;

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

    ntex::rt::spawn(async move {
        let res = cert::test_domain_exists(&domain);
        let exists = match res {
            Ok(e) => e,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };
        println!("exists: {}", exists);
    
        if !exists {
            println!("Request cert...");
            let res = acme::request_cert(&acme::AcmeInfo{
                domain: domain.to_owned(),
                email: email.to_owned(),
                web_root: "./static".to_owned(),
            });
    
            match res {
                Ok(crt) => {
                    match cert::create_cert_file(&domain, &crt) {
                        Ok(_) => println!("Cert saved."),
                        Err(err) => println!("{}", err),
                    }
                    
                }
                Err(err) => {
                    println!("{}", err);
                    // process::exit(0x0100);
                }
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
}
