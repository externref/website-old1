use rand;
use sqlx;

pub struct DatabaseHandler {
    pub pool: std::sync::Arc<sqlx::PgPool>,
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
        .execute(&*self.pool)
        .await
        .expect("Unable to create database.");
        return self;
    }
    fn create_random_string(&mut self) -> String {
        let s: String =
            rand::Rng::sample_iter(rand::thread_rng(), &rand::distributions::Alphanumeric)
                .take(7)
                .map(char::from)
                .collect();
        return s;
    }
    pub async fn add_paste(&mut self, content: &str) -> String {
        let paste_id = self.create_random_string();
        sqlx::query(&format!(
            "
                INSERT INTO pastes 
                VALUES ({}, {})
                ",
            paste_id, content
        ))
        .execute(&*self.pool)
        .await
        .expect("Unable to add new paste.");
        return paste_id;
    }
}
