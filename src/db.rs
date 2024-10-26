use deadpool_diesel::postgres::Pool;
use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::products_highlighted)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProductHighlighted {
    pub id: i32,
    pub product_id: i32,
    pub priority: i16,
    pub enabled: bool,
}
const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
pub async fn run_migrations(pool: &Pool) -> anyhow::Result<()> {
    let conn = pool.get().await?;
    conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()).unwrap())
        .await
        .unwrap();
    Ok(())
}
