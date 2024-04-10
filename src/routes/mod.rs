pub mod family;

use diesel_async::pooled_connection::{
    bb8::Pool, bb8::PooledConnection, AsyncDieselConnectionManager,
};
use diesel_async::AsyncPgConnection;

pub type DbPool = Pool<AsyncPgConnection>;
pub type PgPooledConn = PooledConnection<'static, AsyncPgConnection>;

pub async fn initialize_db_pool() -> DbPool {
    let conn_spec =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable should be set");
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(conn_spec);
    Pool::builder()
        .build(config)
        .await
        .expect("Unable to connect into Database")

    // let mut conn = pool
    //     .get()
    //     .await
    //     .expect("Unable to get pool connection available from Database");

    // return conn;
}
