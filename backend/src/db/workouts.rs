use std::sync::Arc;
use std::collections::HashMap;
use chrono::NaiveDate;
use diesel::{ExpressionMethods, QueryDsl, BoolExpressionMethods};
// use diesel::RunQueryDsl;
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use crate::{db::{database::DBOperations, model::{CardioLog, NewCardioLog, NewWorkoutSession, NewWorkoutSet, UpdateCardioLog, UpdateWorkoutSession, UpdateWorkoutSet, WorkoutSession, WorkoutSet, MuscleGroup, Variation, CardioExercise}}, schema::fittrack::{cardio_logs, sets, workout_sessions, variations, muscle_groups, cardio_exercises}};


use anyhow::{bail, Result};
// use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;

pub struct WorkoutDB{
    database: Arc<DBOperations>,
    pool: Option<Pool<AsyncPgConnection>>,
}

impl WorkoutDB{
    pub fn new (database: Arc<DBOperations>) -> Self{
        WorkoutDB { 
            database, 
            pool: None
        }
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

        let inserted_log:CardioLog = match diesel::insert_into(cardio_logs::table)
            .values(&logs)
            // .returning(WorkoutSet::as_select()) 
            .get_result(&mut conn)
            .await
            {
                Ok(sok) => sok,
                Err(err) => bail!("{}",err)
            };
        Ok(())
    }

    pub async fn update_workout_session(&self, user_id: i32, session_id: i32, data: UpdateWorkoutSession) -> Result<WorkoutSession> {
        let pool = match &self.pool {
            Some(p) => p,
            None => bail!("Pool not initialized"),
        };
        let mut conn = pool.get().await?;

        use diesel::{ExpressionMethods, QueryDsl};

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
        let mut conn = pool.get().await?;

        use diesel::{ExpressionMethods, QueryDsl};

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
        let mut conn = pool.get().await?;

        use diesel::{ExpressionMethods, QueryDsl};

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
        let mut conn = pool.get().await?;

        use diesel::{ExpressionMethods, QueryDsl};

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
        let mut conn = pool.get().await?;

        use diesel::{ExpressionMethods, QueryDsl};

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
        let mut conn = pool.get().await?;

        use diesel::{ExpressionMethods, QueryDsl};

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

    pub async fn get_history(&self, user_id: i32, limit: i64) -> Result<Vec<WorkoutSession>> {
        let pool = match &self.pool {
            Some(p) => p,
            None => bail!("Pool not initialized"),
        };
        let mut conn = pool.get().await?;

        use diesel::{ExpressionMethods, QueryDsl};

        let history = workout_sessions::table
            .filter(workout_sessions::user_id.eq(user_id))
            .order(workout_sessions::start_time.desc()) // Wait, checking model it is `start_time` or `date`?
            // Checking model.rs: `start_time: chrono::NaiveDateTime`.
            // Checking DB Schema: `start_time`
            .limit(limit)
            .get_results(&mut conn)
            .await?;
        Ok(history)
    }

    pub async fn get_session_details(&self, user_id: i32, session_id: i32) -> Result<(WorkoutSession, Vec<WorkoutSet>, Vec<CardioLog>)> {
        let pool = match &self.pool {
            Some(p) => p,
            None => bail!("Pool not initialized"),
        };
        let mut conn = pool.get().await?;

        use diesel::{ExpressionMethods, QueryDsl};

        // 1. Get Session
        let session: WorkoutSession = workout_sessions::table
            .filter(workout_sessions::id.eq(session_id))
            .filter(workout_sessions::user_id.eq(user_id))
            .first(&mut conn)
            .await?;

        // 2. Get Sets (belonging to this session). 
        // Note: Sets have `workout_session_id`.
        let session_sets: Vec<WorkoutSet> = sets::table
            .filter(sets::workout_session_id.eq(session_id))
            .filter(sets::user_id.eq(user_id)) // Redundant security but good
            .get_results(&mut conn)
            .await?;

        // 3. Get Cardio Logs
        let session_cardio: Vec<CardioLog> = cardio_logs::table
            .filter(cardio_logs::workout_session_id.eq(session_id))
            .filter(cardio_logs::user_id.eq(user_id))
            .get_results(&mut conn)
            .await?;

        Ok((session, session_sets, session_cardio))
    }


    pub async fn get_monthly_workout_details(&self, user_id: i32, year: i32, month: i32 ) -> Result<Vec<WorkoutSession>>{
        let pool = match &self.pool{
            Some(p) => p,
            None => bail!("Pool not initialised",)
        };
        let mut conn = match pool.get().await{
            Ok(c)  => c,
            Err(err) => bail!(err)
        };


        let monthly_workout: Vec<WorkoutSession> = workout_sessions::table
            .filter(workout_sessions::user_id.eq(user_id))
            .filter(diesel::dsl::sql::<diesel::sql_types::Bool>(&format!("EXTRACT(YEAR FROM date) = {}", year)))
            .filter(diesel::dsl::sql::<diesel::sql_types::Bool>(&format!("EXTRACT(MONTH FROM date) = {}", month)))
            .get_results(&mut conn)
            .await?;
        Ok(monthly_workout)
    }

    pub async fn get_performance_details(&self, user_id: i32, variation_id: i32, start_date: NaiveDate, end_date: NaiveDate) -> Result<Vec<WorkoutSet>,>{
        let pool = match &self.pool{
            Some(p) => p,
            None => bail!("Pool not initialised")
        };

        let mut conn = match pool.get().await{
            Ok(c) => c,
            Err(err) => {
                println!("Error getting connection: {}", err);
                bail!(err)
            }
        };

        let result = sets::table
            .filter(sets::user_id.eq(user_id))
            .filter(sets::variation_id.eq(variation_id))
            .filter(sets::performed_on.ge(start_date))
            .filter(sets::performed_on.le(end_date))
            .get_results(&mut conn)
            .await;

        match result {
            Ok(data) => {
                Ok(data)
            },
            Err(e) => {
                // println!("Error executing query: {:?}", e);
                bail!(e)
            }
        }
    }

    pub async fn get_sets_for_musclegroups(&self, user_id: i32, muscle_group_ids: Vec<i32>, start_date: NaiveDate, end_date: NaiveDate) -> Result<Vec<WorkoutSet>,>{
        let pool = match &self.pool{
            Some(p) => p,
            None => bail!("Pool not initialised")
        };

        let mut conn = match pool.get().await{
            Ok(c) => c,
            Err(err) => {
                println!("Error getting connection: {}", err);
                bail!(err)
            }
        };

        let result: Result<Vec<WorkoutSet>, _> = sets::table
            .inner_join(variations::table)
            .filter(sets::user_id.eq(user_id))
            .filter(sets::performed_on.ge(start_date))
            .filter(sets::performed_on.le(end_date))
            .filter(variations::muscle_group_id.eq_any(muscle_group_ids))
            .filter(variations::user_id.eq(user_id).or(variations::user_id.eq(0)))
            .select(sets::all_columns)
            .get_results(&mut conn)
            .await;

        match result {
            Ok(data) => {
                Ok(data)
            },
            Err(e) => {
                // println!("Error executing query: {:?}", e);
                bail!(e)
            }
        }
    }

    pub async fn get_varaition_ids(&self, user_id: i32, muscle_group_ids: Vec<i32>) -> Result<HashMap<i32, i32>> {
        let pool = match &self.pool {
            Some(p) => p,
            None => bail!("Pool not initialised")
        };

        let mut conn = match pool.get().await {
            Ok(c) => c,
            Err(err) => bail!(err)
        };

        let results = match variations::table
            .filter(variations::muscle_group_id.eq_any(muscle_group_ids))
            .filter(variations::user_id.eq(user_id).or(variations::user_id.eq(0)))
            .select((variations::muscle_group_id, variations::id))
            .get_results::<(i32, i32)>(&mut conn)
            .await{
                Ok(res) => res,
                Err(err) => bail!(err)
            };

        let mut map: HashMap<i32, i32> = HashMap::new();
        for (mg_id , var_id) in results{
            map.insert(var_id, mg_id);
        }
        Ok(map)
    }

    pub async fn get_all_muscle_groups(&self,user_id: i32) -> Result<Vec<MuscleGroup>> {
        let pool = match &self.pool{
            Some(p) => p,
            None => bail!("Pool not initialised")
        };
        let mut conn = pool.get().await?;
        
        // Fetch global (0) or specific user
        let results = muscle_groups::table
            .filter(muscle_groups::user_id.eq(user_id).or(muscle_groups::user_id.eq(0)))
            .load::<MuscleGroup>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_all_variations(&self,user_id: i32) -> Result<Vec<Variation>> {
        let pool = match &self.pool{
            Some(p) => p,
            None => bail!("Pool not initialised")
        };
        let mut conn = pool.get().await?;
        
        let results = variations::table
            .filter(variations::user_id.eq(user_id).or(variations::user_id.eq(0)))
            .load::<Variation>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_all_cardio_exercises(&self,user_id: i32) -> Result<Vec<CardioExercise>> {
        let pool = match &self.pool{
            Some(p) => p,
            None => bail!("Pool not initialised")
        };
        let mut conn = pool.get().await?;
        
        let results = cardio_exercises::table
            .filter(cardio_exercises::user_id.eq(user_id).or(cardio_exercises::user_id.eq(0)))
            .load::<CardioExercise>(&mut conn)
            .await?;
        Ok(results)
    }
}