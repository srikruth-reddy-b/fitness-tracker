use std::sync::Arc;
use std::collections::HashMap;
use chrono::NaiveDate;
use diesel::{ExpressionMethods, QueryDsl, BoolExpressionMethods};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use crate::{db::{database::DBOperations, model::{CardioLog, WorkoutSession, WorkoutSet, MuscleGroup, Variation, CardioExercise}}, schema::fittrack::{cardio_logs, sets, workout_sessions, variations, muscle_groups, cardio_exercises}};
use anyhow::{bail, Result};
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

    pub async fn get_history(&self, user_id: i32, limit: i64, start_date: Option<NaiveDate>, end_date: Option<NaiveDate>) -> Result<Vec<WorkoutSession>> {
        let pool = match &self.pool {
            Some(p) => p,
            None => bail!("Pool not initialized"),
        };
        let mut conn = pool.get().await?;

        use diesel::{ExpressionMethods, QueryDsl};

        let mut query = workout_sessions::table.into_boxed();
        query = query.filter(workout_sessions::user_id.eq(user_id));

        if let Some(s) = start_date {
            query = query.filter(workout_sessions::start_time.ge(s.and_hms_opt(0, 0, 0).unwrap()));
        }

        if let Some(e) = end_date {
            query = query.filter(workout_sessions::start_time.le(e.and_hms_opt(23, 59, 59).unwrap()));
        }

        let history = query
            .order(workout_sessions::start_time.desc())
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

        let session: WorkoutSession = workout_sessions::table
            .filter(workout_sessions::id.eq(session_id))
            .filter(workout_sessions::user_id.eq(user_id))
            .first(&mut conn)
            .await?;

        let session_sets: Vec<WorkoutSet> = sets::table
            .filter(sets::workout_session_id.eq(session_id))
            .filter(sets::user_id.eq(user_id)) // Redundant security but good
            .get_results(&mut conn)
            .await?;

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