use std::env;

use bb8::Pool;
use bb8_postgres::{PostgresConnectionManager, tokio_postgres::NoTls};
use log::{info, error};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct File {
    pub id: i32,
    pub telegram_id: String,
    pub name: String,
}

impl File {
    pub fn new(telegram_id: &str, name: &str) -> Self {
        File { id: 0, telegram_id: telegram_id.to_string(), name: name.to_string() }
    }
}

pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;

#[derive(Clone)]
pub struct Database {
    pool: DbPool
}

impl Database {
    pub async fn new() -> Self {
        info!("Starting database connection pool");

        let db_url = env::var("DATABASE_URL").expect("The database url is necessary");

        let manager = bb8_postgres::PostgresConnectionManager::new_from_stringlike(
            &db_url,
            NoTls
        ).unwrap();

        let pool = bb8::Pool::builder()
            .build(manager)
            .await
            .unwrap();
        
        Database { pool }
    }

    pub async fn add(&self, user_id: &str, file: &File) -> Result<(), ()> {
        let conn = self.pool.get().await.unwrap();
        
        if let Err(err) = conn.execute(
            "insert into file(telegram_id, name, user_id) values ($1::TEXT, $2::TEXT, $3::TEXT)",
            &[&file.telegram_id, &file.name, &user_id]
        ).await {
            error!("Failed to insert file: {err}");
            return Err(());
        }

        Ok(())
    }

    pub async fn remove(&self, user_id: &str, file_id: i32) -> Result<(), ()> {
        let conn = self.pool.get().await.unwrap();

        if let Err(err) = conn.execute(
            "delete from file where user_id = $1::TEXT and id = $2::INT",
            &[&user_id, &file_id]
        ).await {
            error!("Failed to remove file from user: {err}");
            return Err(());
        }

        Ok(())
    }

    pub async fn get(&self, id: i32, user_id: &str) -> Result<File, ()> {
        let conn = self.pool.get().await.unwrap();

        match conn.query(
            "select * from file where id = $1::INT and user_id = $2::TEXT",
            &[&id, &user_id]
        ).await {
            Ok(rows) => {
                let mut file = File::new(rows[0].get("telegram_id"), rows[0].get("name"));
                file.id = rows[0].get("id");
                return Ok(file);
            },
            Err(err) => {
                error!("Failed to get file: {err}");
                return Err(());
            }
        }
    }

    pub async fn get_all(&self, user_id: &str, qtd: i32, page: i32) -> Result<Vec<File>, ()> {
        let conn = self.pool.get().await.unwrap();

        match conn.query(
            "select * from file where user_id = $1::TEXT order by id desc limit $2::INT offset $3::INT",
            &[&user_id, &qtd, &((page - 1) * qtd)]
        ).await {
            Ok(rows) => {
                let files: Vec<_> = rows.iter().map(|row| {
                    let mut file = File::new(row.get("telegram_id"), row.get("name"));
                    file.id = row.get("id");
                    return file;
                }).collect();

                return Ok(files);
            },
            Err(err) => {
                error!("Failed to get files: {err}");
                return Err(());
            }
        }
    }

    pub async fn search(&self, user_id: &str, search: &str) -> Result<Vec<File>, ()> {
        let conn = self.pool.get().await.unwrap();

        match conn.query(
            "select * from file where user_id = $1::TEXT and name like $2::TEXT order by id desc limit 50",
            &[&user_id, &format!("%{search}%")]
        ).await {
            Ok(rows) => {
                let files: Vec<_> = rows.iter().map(|row| {
                    let mut file = File::new(row.get("telegram_id"), row.get("name"));
                    file.id = row.get("id");
                    return file;
                }).collect();

                return Ok(files);
            },
            Err(err) => {
                error!("Failed to get files: {err}");
                return Err(());
            }
        }
    }
}