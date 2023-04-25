use azure_core::headers::APP;
use rbdc_mssql::MssqlConnection;
use refinery::config::{Config, ConfigDbType};
use refinery::Error;
use crate::configuration::{APPCONFIG, ApplicationConfig};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;


mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

pub async fn migrate() -> Result<(), Error> {
    let mut config = Config::from_file_location("application.toml")?;
    config.set_trust_cert();
    embedded::migrations::runner().run_async(&mut config).await.unwrap();
    Ok(())
}