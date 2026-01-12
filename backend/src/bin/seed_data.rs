use backend::db::database::DBOperations;
use backend::schema::fittrack::{muscle_groups, variations, cardio_exercises, users};
use backend::db::model::{MuscleGroup, Variation, CardioExercise};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::collections::HashMap;
use std::sync::Arc;
use std::error::Error;
use csv::Reader;
use std::fs::File;
use anyhow::{Result, bail};

#[tokio::main]
async fn main() -> Result<(),> {
    dotenv::dotenv().ok();
    env_logger::init();
    
    println!("Starting Metadata Seeding...");

    let mut database = DBOperations::new();
    if let Err(err) = database.init().await{
        println!("Error initializing database: {}", err);
        bail!(err)
    }
    let pool = match database.get_pool().await{
        Ok(p) => p,
        Err(err) => {
            println!("Error getting pool: {}", err);
            bail!(err)
        }
    };
    let mut conn = match pool.get().await{
        Ok(c) => c,
        Err(err) => {
            println!("Error getting connection: {}", err);
            bail!(err)
        }
    };

    // 0. Seed System User
    println!("Seeding System User specific ID 0...");
    let system_user_exists: Option<i32> = match users::table
        .filter(users::id.eq(0))
        .select(users::id)
        .first(&mut conn)
        .await
        .optional(){
            Ok(opt) => opt,
            Err(err) => {
                println!("Error checking system user: {}", err);
                bail!(err)
            }
        };

    if system_user_exists.is_none() {
        match diesel::insert_into(users::table)
            .values((
                users::id.eq(0),
                users::username.eq("system"),
                users::email.eq("system@fittrack.com"),
                users::password.eq("system_password_placeholder"),
                users::fullname.eq("System Administrator"),
            ))
            .execute(&mut conn)
            .await{
                Ok(_) => println!("System User (ID 0) created."),
                Err(err) => {
                    println!("Error creating System User: {}", err);
                    bail!(err)
                }
            }
    } else {
        println!("System User (ID 0) already exists.");
    }

    // 1. Seed Muscle Groups
    println!("Seeding Muscle Groups...");
    let mut mg_map: HashMap<String, i32> = HashMap::new();
    let file = match File::open("muscle_groups.csv"){
        Ok(f) => f,
        Err(err) => {
            println!("Error opening muscle_groups.csv: {}", err);
            bail!(err)
        }
    };
    let mut rdr = Reader::from_reader(file);

    for result in rdr.records() {
        let record = match result{
            Ok(rec) => rec,
            Err(err) => {
                println!("Error reading muscle group record: {}", err);
                bail!(err)
            }
        };
        let id_str = &record[0];
        let name = &record[1];
        
        let id: i32 = match id_str.parse(){
            Ok(num) => num,
            Err(err) => {
                println!("Error parsing muscle group ID '{}': {}", id_str, err);
                bail!(err)
            }
        };

        // Check if exists
        let existing: Option<i32> = match muscle_groups::table
            .filter(muscle_groups::id.eq(id))
            .select(muscle_groups::id)
            .first(&mut conn)
            .await
            .optional(){
                Ok(opt) => opt,
                Err(err) => {
                    println!("Error checking existing muscle group ID {}: {}", id, err);
                    bail!(err)
                }
            };

        if existing.is_none() {
             match diesel::insert_into(muscle_groups::table)
                .values((
                    muscle_groups::id.eq(id),
                    muscle_groups::name.eq(name), 
                    muscle_groups::user_id.eq(Some(0))
                ))
                .execute(&mut conn)
                .await{
                    Ok(_) => (),
                    Err(err) => {
                        println!("Error inserting muscle group {}: {}", name, err);
                        bail!(err)
                    }
                }
        } else {
            println!("  Skipping {}: ID {} already exists", name, id);
        }
        
        mg_map.insert(name.to_string(), id);
    }
    println!("Muscle Groups complete.");

    // 2. Seed Variations
    println!("Seeding Variations...");
    let file = match File::open("variations.csv"){
        Ok(f) => f,
        Err(err) => {
            println!("Error opening variations.csv: {}", err);
            bail!(err)
        }
    };
    let mut rdr = Reader::from_reader(file);

    for result in rdr.records() {
        let record = match result{
            Ok(rec) => rec,
            Err(err) => {
                println!("Error reading variation record: {}", err);
                bail!(err)
            }
        };
        let id_str = &record[0];
        let name = &record[1];
        let mg_name = &record[2];
        
        let id: i32 = match id_str.parse(){
            Ok(num) => num,
            Err(err) => {
                println!("Error parsing variation ID '{}': {}", id_str, err);
                bail!(err)
            }
        };

        if let Some(&mg_id) = mg_map.get(mg_name) {
             let existing_var: Option<i32> = match variations::table
                .filter(variations::id.eq(id))
                .select(variations::id)
                .first(&mut conn)
                .await
                .optional(){
                    Ok(opt) => opt,
                    Err(err) => {
                        println!("Error checking existing variation ID {}: {}", id, err);
                        bail!(err)
                    }
                };

            if existing_var.is_none() {
                match diesel::insert_into(variations::table)
                    .values((
                        variations::id.eq(id),
                        variations::name.eq(name),
                        variations::muscle_group_id.eq(mg_id),
                        variations::user_id.eq(Some(0))
                    ))
                    .execute(&mut conn)
                    .await{
                        Ok(_) => (),
                        Err(err) => {
                            println!("Error inserting variation {}: {}", name, err);
                            bail!(err)
                        }
                    }
            } else {
                 println!("  Skipping {}: ID {} already exists", name, id);
            }
        } else {
            println!("  Warning: Muscle Group '{}' not found for Variation '{}'", mg_name, name);
        }
    }
    println!("Variations complete.");

    // 3. Seed Cardio Exercises
    println!("Seeding Cardio Exercises...");
    let file = match File::open("cardio_exercises.csv"){
        Ok(f) => f,
        Err(err) => {
            println!("Error opening cardio_exercises.csv: {}", err);
            bail!(err)
        }
    };
    let mut rdr = Reader::from_reader(file);

    for result in rdr.records() {
        let record = match result{
            Ok(rec) => rec,
            Err(err) => {
                println!("Error reading cardio exercise record: {}", err);
                bail!(err)
            }
        };
        let id_str = &record[0];
        let name = &record[1];
        
        let id: i32 = match id_str.parse(){
            Ok(num) => num,
            Err(err) => {
                println!("Error parsing cardio exercise ID '{}': {}", id_str, err);
                bail!(err)
            }
        };

        let existing: Option<i32> = match cardio_exercises::table
            .filter(cardio_exercises::id.eq(id))
            .select(cardio_exercises::id)
            .first(&mut conn)
            .await
            .optional(){
                Ok(opt) => opt,
                Err(err) => {
                    println!("Error checking existing cardio ID {}: {}", id, err);
                    bail!(err)
                }
            };

        if existing.is_none() {
             match diesel::insert_into(cardio_exercises::table)
                .values((
                    cardio_exercises::id.eq(id),
                    cardio_exercises::name.eq(name),
                    cardio_exercises::user_id.eq(Some(0))
                ))
                .execute(&mut conn)
                .await{
                    Ok(_) => (),
                    Err(err) => {
                        println!("Error inserting cardio exercise {}: {}", name, err);
                        bail!(err)
                    }
                };
        } else {
             println!("  Skipping {}: ID {} already exists", name, id);
        }
    }
    println!("Cardio Exercises complete.");

    // 4. Reset Sequences
    println!("Resetting Sequences...");
    match diesel::sql_query("SELECT setval('fittrack.muscle_groups_id_seq', (SELECT MAX(id) FROM fittrack.muscle_groups));")
        .execute(&mut conn)
        .await{
            Ok(_) => (),
            Err(err) => {
                println!("Error resetting muscle_groups sequence: {}", err);
                bail!(err)
            }
        }
    match diesel::sql_query("SELECT setval('fittrack.variations_id_seq', (SELECT MAX(id) FROM fittrack.variations));")
        .execute(&mut conn)
        .await{
            Ok(_) => (),
            Err(err) => {
                println!("Error resetting variations sequence: {}", err);
                bail!(err)
            }
        }
    match diesel::sql_query("SELECT setval('fittrack.cardio_exercises_id_seq', (SELECT MAX(id) FROM fittrack.cardio_exercises));")
        .execute(&mut conn)
        .await{
            Ok(_) => (),
            Err(err) => {
                println!("Error resetting cardio_exercises sequence: {}", err);
                bail!(err)
            }
        }
    println!("Sequences reset to MAX(id).");

    println!("Seeding Finished Successfully!");
    Ok(())
}
