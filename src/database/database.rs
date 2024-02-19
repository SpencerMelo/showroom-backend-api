use std::env;
use std::time::Duration;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, NopErrorHandler, Pool};

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .connection_timeout(Duration::from_secs(30))
        .error_handler(Box::new(NopErrorHandler))
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
