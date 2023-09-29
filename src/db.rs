
use crate::read_path::File;
use sqlx::{self};
use std::fs;
use std::path::Path;

pub struct DbForStorageFile;

impl DbForStorageFile {
    pub async fn create_table(pool: &sqlx::SqlitePool) {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS files (
                id INTEGER PRIMARY KEY,
                path TEXT NOT NULL,
                size INTEGER NOT NULL,
                data TEXT NOT NULL
            )
            "#,
        )
        .execute(pool)
        .await
        .unwrap();
    }
    pub async fn embed_directory(
        pool: &sqlx::SqlitePool,
        dir_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for entry in walkdir::WalkDir::new(dir_path) {
            let entry = entry?;

            let path = entry.path();

            if path.is_file() {
                let data = fs::read(path)?;
                let size = data.len() as i64;

                let path_str = path.to_str().ok_or("Invalid")?;

                sqlx::query(
                    r#"
                                        INSERT INTO files (path, size, data)
                                        VALUES(?, ?, ?)
                
                                    
                                    "#,
                )
                .bind(path_str)
                .bind(size)
                .bind(&data)
                .execute(pool)
                .await?;
            }
        }

        Ok(())
    }

    pub async fn embed_file(pool: &sqlx::SqlitePool, file: &File) {
    
        let metadata = match fs::metadata(&file.path) {
            Ok(metadata) => metadata,
            Err(e) => {
                eprintln!("Error reading metadata of {:?}: {}", file.path, e);
                return;
            }
        };

       
        if !metadata.is_file() {
            eprintln!("{:?} is not a file", file.path);
            return;
        }


        let size = metadata.len() as i64;


        let data = match fs::read_to_string(&file.path) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error reading {:?}: {}", file.path, e);
                return;
            }
        };

     
        sqlx::query(
            r#"
            INSERT INTO files (path, size, data)
            VALUES (?, ?, ?)
            "#,
        )
    
        .bind(file.path.to_str().unwrap())
      
        .bind(size)
     
        .bind(data)
   
        .execute(pool)
        .await
        .unwrap();
    }
}
