use sqlx;

pub struct DatabaseHandler {
    pub pool: sqlx::PgPool,
}

impl DatabaseHandler {
    pub async fn setup(self) -> Self {
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS pastes (
                paste_id VARCHAR PRIMARY KEY NOT NULL,
                paste_data VARCHAR  
            );
        ",
        )
        .execute(&self.pool)
        .await
        .expect("Unable to create database.");
        return self;
    }
}
