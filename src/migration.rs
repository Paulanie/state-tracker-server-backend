use refinery::config::{Config};
use refinery::{Error, Report};


mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

pub async fn migrate() -> Result<Report, Error> {
    let mut config = Config::from_file_location("application.toml")?;
    config.set_trust_cert();
    embedded::migrations::runner().run_async(&mut config).await
}