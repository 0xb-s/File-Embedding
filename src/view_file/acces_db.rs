use sqlx::{self, SqlitePool};

#[derive(Debug)]
pub struct FileData {
    pub id: i32,
    pub path: String,
    pub size: i64,
    pub data: Vec<u8>,
}

// pub async fn fetch_data(pool: &SqlitePool) -> Vec<FileData> {
//     let rows = sqlx::query(
//         r#"
//         SELECT * from files;
//         "#,
//     )
//     .fetch_all(pool)
//     .await;

//     let mut files = Vec::new();

//     match rows {
//         Ok(rows) => {
//             for row in rows {
//                 let file = FileData {
//                     id: row.get("id"),
//                     path: row.get("path"),
//                     size: row.get("size"),
//                     data: row.get("data"),
//                 };
//                 files.push(file);
//             }
//         }
//         Err(e) => {
//             eprintln!("Error fetching data: {}", e);
//             return files; // return early in case of error
//         }
//     }

//     files
// }
// pub async fn fetch_data(pool: &SqlitePool) -> Result<Vec<FileData>, sqlx::Error> {
//     let rows = sqlx::query(
//         r#"
//         SELECT * from files;
//         "#,
//     )
//     .fetch_all(pool)
//     .await;

//     let mut files = Vec::new();

//     match rows {
//         Ok(rows) => {
//             for row in rows {
//                 let file = FileData {
//                     id: row.get("id"),
//                     path: row.get("path"),
//                     size: row.get("size"),
//                     data: row.get("data"),
//                 };
//                 files.push(file);
//             }
//             Ok(files)
//         }
//         Err(e) => Err(e),
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use tokio::runtime::Runtime;

//     #[test]
//     fn it_works() {
//         let rt = Runtime::new().unwrap();
//         rt.block_on(async {
//             let pool = SqlitePool::connect("sqlite:me.db").await.unwrap();
//             let files = fetch_data(&pool).await;
//             for file in files {
//                 let data_str = String::from_utf8_lossy(&file.data);
//                 println!(
//                     "ID: {}, Path: {}, Size: {}, Data: {}",
//                     file.id, file.path, file.size, data_str
//                 );
//             }
//         });
//     }
// }
// Fetch a specific attribute by ID from the database
pub async fn fetch_path_by_id(pool: &SqlitePool, id: i32) -> Result<String, sqlx::Error> {
    let result: (String,) = sqlx::query_as(
        r#"
        SELECT path FROM files WHERE id = ?;
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(result.0)
}

pub async fn fetch_size_by_id(pool: &SqlitePool, id: i32) -> Result<i64, sqlx::Error> {
    let result: (i64,) = sqlx::query_as(
        r#"
        SELECT size FROM files WHERE id = ?;
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(result.0)
}

// pub async fn fetch_data_by_id(pool: &SqlitePool, id: i32) -> Result<Vec<u8>, sqlx::Error> {
//     let result: (Vec<u8>,) = sqlx::query_as(
//         r#"
//         SELECT data FROM files WHERE id = ?;
//         "#,
//     )
//     .bind(id)
//     .fetch_one(pool)
//     .await?;
//     Ok(result.0)
// }

pub async fn fetch_data_by_id(
    pool: &SqlitePool,
    id: i32,
) -> Result<String, Box<dyn std::error::Error>> {
    let result: (Vec<u8>,) = sqlx::query_as(
        r#"
        SELECT data FROM files WHERE id = ?;
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    // Attempt to convert the byte vector to a String
    let string_data = String::from_utf8(result.0)?;

    Ok(string_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    use sqlx::SqlitePool;

    // You can use a setup function to initialize the database connection for each test.
    async fn setup() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite:me.db").await.unwrap();
        pool
    }

    #[tokio::test]
    async fn test_fetch_path_by_id() {
        let pool = setup().await;
        let id = 1; // use a known ID from your test database
        match fetch_path_by_id(&pool, id).await {
            Ok(path) => println!("Path: {}", path),
            Err(e) => eprintln!("Error fetching path: {}", e),
        }
    }

    #[tokio::test]
    async fn test_fetch_size_by_id() {
        let pool = setup().await;
        let id = 1; // use a known ID from your test database
        match fetch_size_by_id(&pool, id).await {
            Ok(size) => println!("Size: {}", size),
            Err(e) => eprintln!("Error fetching size: {}", e),
        }
    }

    // #[tokio::test]
    // async fn test_fetch_data_by_id() {
    //     let pool = setup().await;
    //     let id = 1; // use a known ID from your test database
    //     match fetch_data_by_id(&pool, id).await {
    //         Ok(data) => {
    //             let data_str = String::from_utf8_lossy(&data);
    //             println!("Data: {}", data_str);
    //         }
    //         Err(e) => eprintln!("Error fetching data: {}", e),
    //     }
    // }
}
