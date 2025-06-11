use diesel::{Connection, PgConnection};
use diesel_async::{
    pooled_connection::{
        bb8::{Pool, PooledConnection},
        AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;
use tokio::sync::OnceCell;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type DbPool = Pool<AsyncPgConnection>;

pub type DbConnection<'a> = PooledConnection<'a, AsyncPgConnection>;

pub async fn create_pool(db_url: &String) -> Result<DbPool, Box<dyn std::error::Error>> {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    let pool = Pool::builder().max_size(15).build(config).await?;
    Ok(pool)
}

pub async fn get_connection_pool(db_url: &String) -> &'static Pool<AsyncPgConnection> {
    static POOL: OnceCell<Pool<AsyncPgConnection>> = OnceCell::const_new();
    POOL.get_or_init(async || create_pool(db_url).await.expect("error  creating pool"))
        .await
}

pub fn create_conection(db_url: &String) -> PgConnection {
    PgConnection::establish(db_url).unwrap_or_else(|_| panic!("Error connecting to {db_url}"))
}

pub async fn run_migrations(
    connection: &mut PgConnection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
