use std::sync::Arc;
use crate::db::database::DBOperations;
use anyhow::{Result,bail};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use crate::{db::{model::{CardioLog, NewCardioLog, NewWorkoutSession, NewWorkoutSet, UpdateCardioLog, UpdateWorkoutSession, UpdateWorkoutSet, WorkoutSession, WorkoutSet, MuscleGroup, Variation, CardioExercise, NewMuscleGroup, NewVariation, NewCardioExercise}}, schema::fittrack::{cardio_logs, sets, workout_sessions, variations, muscle_groups, cardio_exercises}};
use diesel_async::RunQueryDsl;
use diesel::ExpressionMethods;

pub struct LoggerDB{
    database: Arc<DBOperations>,
    pool: Option<Pool<AsyncPgConnection>>
}

impl LoggerDB{
    pub fn new(database: Arc<DBOperations>) -> Self{
        LoggerDB { database, pool: None }
    }

    pub async fn init(&mut self) -> Result<(),>{
        let pool = match self.database.get_pool().await{
            Ok(p) => p,
            Err(err) => bail!("{}",err)
        };

        self.pool = Some(pool);
        Ok(())
    }

    pub async fn add_workout_session(&self, mut session: NewWorkoutSession) -> Result<WorkoutSession>{
        let pool = match &self.pool{
            Some(pok) => pok,
            None => bail!("Pool is not intialiased"),
        };
        let mut conn = match pool.get().await{
            Ok(cok) => cok,
            Err(err) => {
                anyhow::bail!("{}",err);
            }
        };
        if session.title.is_none(){
            session.title = Some(format!("Session-{}", session.date));
        }
        let inserted_session: WorkoutSession = match diesel::insert_into(workout_sessions::table)
            .values(&session)
            // .returning(WorkoutSession::as_select()) 
            .get_result::<WorkoutSession>(&mut conn)
            .await
            {
                Ok(iok) => iok,
                Err(err) => bail!("{}",err)
            };
        Ok(inserted_session)
    }

    pub async fn add_workout_set(&self, set:NewWorkoutSet) -> Result<(),>{
        println!("Adding workout set: {:?}", set);
        let pool = match &self.pool{
            Some(pok ) => pok,
            None => bail!("Pool is not intialised"),
        };

        let mut conn = match pool.get().await{
            Ok(c) => c,
            Err(err) => bail!(err),
        };

        let inserted_set:WorkoutSet = match diesel::insert_into(sets::table)
            .values(&set)
            // .returning(WorkoutSet::as_select()) 
            .get_result(&mut conn)
            .await
            {
                Ok(sok) => sok,
                Err(err) => bail!("{}",err)
            };

        println!("Inserted set: {:?}", inserted_set);
        Ok(())
    }

    pub async fn add_workout_cardio(&self, logs:NewCardioLog) -> Result<(),>{
        let pool = match &self.pool{
            Some(pok ) => pok,
            None => bail!("Pool is not intialised"),
        };

        let mut conn = match pool.get().await{
            Ok(c) => c,
            Err(err) => bail!(err),
        };

        let _inserted_log:CardioLog = match diesel::insert_into(cardio_logs::table)
            .values(&logs)
            .get_result(&mut conn)
            .await
            {
                Ok(sok) => sok,
                Err(err) => bail!("{}",err)
            };
        Ok(())
    }

    pub async fn add_muscle_group(&self, data: NewMuscleGroup<'_>) -> Result<MuscleGroup> {
        let pool = match &self.pool { Some(p) => p, None => bail!("Pool not initialized") };
        let mut conn = match pool.get().await {
            Ok(c) => c,
            Err(err) => bail!(err),
        };
        let res = diesel::insert_into(muscle_groups::table).values(&data).get_result(&mut conn).await?;
        Ok(res)
    }

    pub async fn add_variation(&self, data: NewVariation<'_>) -> Result<Variation> {
        let pool = match &self.pool { Some(p) => p, None => bail!("Pool not initialized") };
        let mut conn = match pool.get().await {
            Ok(c) => c,
            Err(err) => bail!(err),
        };
        let res = diesel::insert_into(variations::table).values(&data).get_result(&mut conn).await?;
        Ok(res)
    }

    pub async fn add_cardio_exercise(&self, data: NewCardioExercise<'_>) -> Result<CardioExercise> {
        let pool = match &self.pool { Some(p) => p, None => bail!("Pool not initialized") };
        let mut conn = match pool.get().await {
            Ok(c) => c,
            Err(err) => bail!(err),
        };
        let res = diesel::insert_into(cardio_exercises::table).values(&data).get_result(&mut conn).await?;
        Ok(res)
    }

    pub async fn update_workout_session(&self, user_id: i32, session_id: i32, data: UpdateWorkoutSession) -> Result<WorkoutSession> {
        let pool = match &self.pool {
            Some(p) => p,
            None => bail!("Pool not initialized"),
        };
        let mut conn = match pool.get().await {
            Ok(c) => c,
            Err(err) => bail!(err),
        };

        let updated_session = diesel::update(workout_sessions::table)
            .filter(workout_sessions::id.eq(session_id))
            .filter(workout_sessions::user_id.eq(user_id))
            .set(data)
            .get_result(&mut conn)
            .await?;
        Ok(updated_session)
    }

    pub async fn delete_workout_session(&self, user_id: i32, session_id: i32) -> Result<()> {
        let pool = match &self.pool {
            Some(p) => p,
            None => bail!("Pool not initialized"),
        };
        let mut conn = match pool.get().await {
            Ok(c) => c,
            Err(err) => bail!(err),
        };

        let count = diesel::delete(workout_sessions::table)
            .filter(workout_sessions::id.eq(session_id))
            .filter(workout_sessions::user_id.eq(user_id))
            .execute(&mut conn)
            .await?;
        
        if count == 0 {
            bail!("Session not found or access denied");
        }
        Ok(())
    }

    pub async fn update_workout_set(&self, user_id: i32, set_id: i32, data: UpdateWorkoutSet) -> Result<WorkoutSet> {
        let pool = match &self.pool {
            Some(p) => p,
            None => bail!("Pool not initialized"),
        };
        let mut conn = match pool.get().await {
            Ok(c) => c,
            Err(err) => bail!(err),
        };

        let updated_set = diesel::update(sets::table)
            .filter(sets::id.eq(set_id))
            .filter(sets::user_id.eq(user_id))
            .set(data)
            .get_result(&mut conn)
            .await?;
        Ok(updated_set)
    }

    pub async fn delete_workout_set(&self, user_id: i32, set_id: i32) -> Result<()> {
        let pool = match &self.pool {
            Some(p) => p,
            None => bail!("Pool not initialized"),
        };
        let mut conn = match pool.get().await {
            Ok(c) => c,
            Err(err) => bail!(err),
        };
        
        let count = diesel::delete(sets::table)
            .filter(sets::id.eq(set_id))
            .filter(sets::user_id.eq(user_id))
            .execute(&mut conn)
            .await?;
        
        if count == 0 {
            bail!("Set not found or access denied");
        }
        Ok(())
    }

    pub async fn update_cardio_log(&self, user_id: i32, log_id: i32, data: UpdateCardioLog) -> Result<CardioLog> {
        let pool = match &self.pool {
            Some(p) => p,
            None => bail!("Pool not initialized"),
        };
        let mut conn = match pool.get().await {
            Ok(c) => c,
            Err(err) => bail!(err),
        };

        let updated_log = diesel::update(cardio_logs::table)
            .filter(cardio_logs::id.eq(log_id))
            .filter(cardio_logs::user_id.eq(user_id))
            .set(data)
            .get_result(&mut conn)
            .await?;
        Ok(updated_log)
    }

    pub async fn delete_cardio_log(&self, user_id: i32, log_id: i32) -> Result<()> {
        let pool = match &self.pool {
            Some(p) => p,
            None => bail!("Pool not initialized"),
        };
        let mut conn = match pool.get().await {
            Ok(c) => c,
            Err(err) => bail!(err),
        };

        let count = diesel::delete(cardio_logs::table)
            .filter(cardio_logs::id.eq(log_id))
            .filter(cardio_logs::user_id.eq(user_id))
            .execute(&mut conn)
            .await?;
        
        if count == 0 {
            bail!("Cardio log not found or access denied");
        }
        Ok(())
    }
}