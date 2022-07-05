
mod acme;

use ntex::web::{self, middleware, App};
use ntex_files as fs;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

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
