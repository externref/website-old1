use sqlx;
use crate::utils;
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

    pub async fn add_paste(&mut self, content: &str) -> String {
        let paste_id = utils::create_random_string();
        sqlx::query(&format!(
            "
                INSERT INTO pastes 
                (paste_id, paste_data)
                VALUES ('{}', '{}');
                ",
            paste_id, content
        ))
        .execute(&*self.pool)
        .await
        .expect("Unable to add new paste.");
        return paste_id;
    }
    pub async fn get_paste(self, id: &str) -> String {
        let row = sqlx::query(&format!(
            "
            SELECT paste_data FROM pastes
            WHERE paste_id = '{}'
            ",
            id.to_lowercase()
        ))
        .fetch_one(&*self.pool)
        .await
        .ok();
        match row {
            Some(row) => {
                let paste_data: Option<String> = sqlx::Row::try_get(&row, "paste_data").ok();
                return paste_data
                    .unwrap_or_else(|| format!("No paste with ID {} found.", id).to_string());
            }
            None => format!("No paste with ID {} found.", id).to_string(),
        }
    }
}
