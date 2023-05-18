use refinery::config::{Config, ConfigDbType};
use refinery::{Error, Report};
use crate::configuration::APPCONFIG;


mod embedded {
    use refinery::embed_migrations;
    #[cfg(feature = "postgres")]
    embed_migrations!("migrations/postgres");
    #[cfg(feature = "mssql")]
    embed_migrations!("migrations/mssql");
}

pub async fn migrate() -> Result<Report, Error> {
    #[cfg(feature = "postgres")]
    let dbtype = ConfigDbType::Postgres;
    #[cfg(feature = "mssql")]
    let dbtype = ConfigDbType::Mssql;

    let mut config = Config::new(dbtype)
        .set_db_host(APPCONFIG.main.db_host.as_str())
        .set_db_port(APPCONFIG.main.db_port.as_str())
        .set_db_user(APPCONFIG.main.db_user.as_str())
        .set_db_pass(APPCONFIG.main.db_pass.as_str())
        .set_db_name(APPCONFIG.main.db_name.as_str());
    config.set_trust_cert();
    embedded::migrations::runner().run_async(&mut config).await
}