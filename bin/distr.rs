#[ntex::main]
async fn main() -> std::io::Result<()> {
    libdistr::start().await
}
