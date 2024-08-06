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
        .build(ConnectionManager::<PgConnection>::new(get_database_url()?))
        .map_err(|err| {
            error!("Unable to connect to database, error: {}", err);
            err.into()
        })
}

fn get_database_url() -> Result<String, env::VarError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial(timeout)]
    fn no_timeout_on_env() {
        // Given 'there is no timeout in environment variable'
        env::remove_var("DATABASE_CONNECTION_TIMEOUT");

        // When 'the application get the connection timeout'
        let current_timeout: Duration = get_connection_timeout();

        // Then 'the timeout should be the default'
        assert_eq!(current_timeout, Duration::from_secs(30));
    }

    #[test]
    #[serial(timeout)]
    fn invalid_timeout_on_env() {
        // Given 'there is a invalid timeout in environment variable'
        env::remove_var("DATABASE_CONNECTION_TIMEOUT");
        env::set_var("DATABASE_CONNECTION_TIMEOUT", "F");

        // When 'the application get the connection timeout'
        let current_timeout: Duration = get_connection_timeout();

        // Then 'the timeout should be the default'
        assert_eq!(current_timeout, Duration::from_secs(30));
    }

    #[test]
    #[serial(timeout)]
    fn valid_timeout_on_env() {
        // Given 'there is a valid timeout in environment variable'
        env::remove_var("DATABASE_CONNECTION_TIMEOUT");
        env::set_var("DATABASE_CONNECTION_TIMEOUT", "10");

        // When 'the application get the connection timeout'
        let current_timeout: Duration = get_connection_timeout();

        // Then 'the timeout should be the value from the environment variable'
        assert_eq!(current_timeout, Duration::from_secs(10));
    }

    #[test]
    #[serial(db_url)]
    fn no_database_url_on_env() {
        // Given 'there is not a database url in environment variable'
        env::remove_var("DATABASE_URL");

        // When 'the application get the database url'
        let current_database_url = get_database_url();

        // Then 'the result should be an Error'
        assert_eq!(current_database_url, Err(env::VarError::NotPresent));
    }

    #[test]
    #[serial(db_url)]
    fn database_url_on_env() {
        // Given 'there is a database url in environment variable'
        env::remove_var("DATABASE_URL");
        env::set_var("DATABASE_URL", "current_value");

        // When 'the application get the database url'
        let current_database_url = get_database_url();

        // Then 'the database url should be the value from the environment variable'
        assert_eq!(current_database_url.unwrap(), "current_value");
    }
}
