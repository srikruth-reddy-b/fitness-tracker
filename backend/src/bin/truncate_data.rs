use backend::db::database::DBOperations;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::sync::Arc;
use std::error::Error;
use anyhow::{Result, bail};

#[tokio::main]
async fn main() -> Result<(),> {
    dotenv::dotenv().ok();
    env_logger::init();
    
    println!("Starting Data Truncation...");

    let mut database = DBOperations::new();
    if let Err(err) = database.init().await{
        println!("Error initializing database: {}", err);
        return Err(err);
    }
    let pool = database.get_pool().await?;
    let mut conn = pool.get().await?;

    println!("⚠ WARNING: Truncating all data...");
    match diesel::sql_query("TRUNCATE TABLE fittrack.sets, fittrack.cardio_logs, fittrack.workout_sessions, fittrack.variations, fittrack.muscle_groups, fittrack.cardio_exercises RESTART IDENTITY CASCADE;")
        .execute(&mut conn)
        .await{
            Ok(_) => (),
            Err(err) => {
                println!("Error truncating tables: {}", err);
                bail!(err);
            }
        };
    
    println!("✅ Tables truncated successfully.");
    Ok(())
}
