use std::env;
use std::error::Error;
use std::time::Duration;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, NopErrorHandler, Pool};
use log::{error, warn};

pub fn get_connection_pool() -> Result<Pool<ConnectionManager<PgConnection>>, Box<dyn Error>> {
    Pool::builder()
        .connection_timeout(get_connection_timeout())
        .error_handler(Box::new(NopErrorHandler))
        .build(ConnectionManager::<PgConnection>::new(read_database_url()?))
        .map_err(|err| {
            error!("Unable to connect to database, error: {}", err);
            err.into()
        })
}

fn read_database_url() -> Result<String, env::VarError> {
    env::var("DATABASE_URL").map_err(|err| {
        error!("environment variable 'DATABASE_URL' must be provided, error: {}", err);
        err
    })
}

fn get_connection_timeout() -> Duration {
    let default_timeout: u64 = 30;
    let timeout_key = "DATABASE_CONNECTION_TIMEOUT";
    let timeout = env::var(timeout_key)
        .map_or_else(
            |err| {
                warn!("Unable to read '{}', error: {}, default to: {} seconds", timeout_key, err, default_timeout);
                default_timeout
            },
            |value| {
                value.parse::<u64>().unwrap_or_else(|err| {
                    warn!("Unable to parse '{}', error: {}, default to: {} seconds", timeout_key, err, default_timeout);
                    default_timeout
                })
            });
    Duration::from_secs(timeout)
}
