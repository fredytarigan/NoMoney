pub mod family;

use diesel::r2d2;
use diesel_async::AsyncPgConnection;
use uuid::Uuid;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<AsyncPgConnection>>;

