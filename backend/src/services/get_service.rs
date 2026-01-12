use std::{collections::HashMap, os::darwin, sync::Arc};
use anyhow::{Result, bail};
use chrono::{Datelike, NaiveDate};
use log::{info, warn};
use serde::Serialize;
use crate::db::{model::{WorkoutSession, WorkoutSet}, user, workouts::WorkoutDB};


const LEVEL_0:i64 = 0;
const LEVEL_1:i64 = 30;
const LEVEL_2:i64 = 60;
const LEVEL_3:i64 = 90;

#[derive(Debug,Serialize)]
pub struct WorkoutLevels{
    pub date: NaiveDate,
    pub level: u8
}

#[derive(Debug, Serialize)]
pub struct PerformanceMetrics{
    pub week: String,
    pub volume: f64
}

#[derive(Debug, Serialize)]
pub struct MuscleGroupVolume {
    pub muscle_group_id: i32,
    pub total_sets: i64,
}

pub struct GetService{
    pub workout: Arc<WorkoutDB>,
}

impl GetService{
    pub fn new(workout: Arc<WorkoutDB>) -> Self{
        GetService { 
            workout 
        }
    }

    pub async fn get_workout_levels(&self, user_id: i32, year: i32, month: i32) -> Result<Vec<WorkoutLevels>,>{
        let workout_details = match self.workout.get_monthly_workout_details(user_id, year, month).await{
            Ok(details) => details,
            Err(err) => {
                return Err(err);
            }
        };

        let mut workout_durations: HashMap<NaiveDate, i64> = HashMap::new();
        for workout in workout_details.iter(){
            workout_durations.entry(workout.date)
                .and_modify(|duration| *duration += (workout.end_time - workout.start_time).num_minutes())
                .or_insert_with(|| (workout.end_time - workout.start_time).num_minutes());
        }
        let mut workout_levels: Vec<WorkoutLevels> = Vec::new();
        for workout_duration in workout_durations.iter(){
            let level = match workout_duration.1{
                d if *d >= LEVEL_3 => 3,
                d if *d >= LEVEL_2 => 2,
                d if *d >= LEVEL_1 => 1,
                _ => 0,
            };
            workout_levels.push(WorkoutLevels{
                date: *workout_duration.0,
                level
            });
        }

        println!("Workout details retrieved: {:?}", workout_levels);
        Ok(workout_levels)
    }

    pub async fn get_performance_details(&self, user_id: i32, variation_id: i32, start_date: NaiveDate, end_date: NaiveDate)-> Result<Vec<PerformanceMetrics>,> {
        let performance_data = match self.workout.get_performance_details(user_id, variation_id, start_date, end_date).await{
            Ok(data) => data,
            Err(err) => bail!(err)
        };

        // 1. Generate chronological list of weeks
        let mut weeks_order = Vec::new();
        let mut seen_weeks = std::collections::HashSet::new();
        let mut cursor = start_date;
        
        while cursor <= end_date {
            let label = Self::get_week_label(cursor);
            if !seen_weeks.contains(&label) {
                seen_weeks.insert(label.clone());
                weeks_order.push(label);
            }
            cursor += chrono::Duration::days(1);
        }

        // 2. Aggregate actual data
        let mut performance_map = HashMap::new();
        for performance in performance_data.iter(){
            let week_label = Self::get_week_label(performance.performed_on);
            performance_map.entry(week_label)
                .and_modify(|vol| *vol += performance.weight * performance.reps as f64)
                .or_insert_with(|| performance.weight * performance.reps as f64);
        }

        // 3. Build complete sorted result
        let performance_metrics = weeks_order.into_iter()
            .map(|week| PerformanceMetrics{
                week: week.clone(),
                volume: *performance_map.get(&week).unwrap_or(&0.0)
            })
            .collect::<Vec<PerformanceMetrics>>();

        println!("Performance data: {:?}", performance_metrics);
        Ok(performance_metrics)
    }

    pub async fn get_numberof_sets_per_musclegroup(&self, user_id: i32, start_date: NaiveDate, end_date: NaiveDate, muscle_group_ids: Vec<i32>) -> Result<Vec<MuscleGroupVolume>,> {
        let sets = match self.workout.get_sets_for_musclegroups(user_id, muscle_group_ids.clone(), start_date, end_date).await{
            Ok(sets) => sets,
            Err(err) => bail!(err)
        };

        let var_map = match self.workout.get_varaition_ids(user_id, muscle_group_ids).await{
            Ok(map) => map,
            Err(err) => bail!(err)
        };
        let mut set_map: HashMap<i32, i64> = HashMap::new();
        for set in sets.iter(){
            let mg_id = match var_map.get(&set.variation_id){
                Some(id) => id,
                None => {
                    warn!("Muscle group ID not found for variation ID: {}", set.variation_id);
                    continue
                },
            };
            
            set_map.entry(*mg_id)
                .and_modify(|count| *count+=1)
                .or_insert(1);
        }

        let results = set_map.into_iter()
            .map(|(id, count)| MuscleGroupVolume {
                muscle_group_id: id,
                total_sets: count,
            })
            .collect();
        
        info!("Muscle group volume summary: {:?}", results);
        Ok(results)
    }



    pub fn get_week_label(date: NaiveDate) -> String{
        let week = date.iso_week().week();
        format!("{}-W{}", date.month(), week)
    }

    pub async fn get_muscle_groups(&self,user_id: i32) -> Result<Vec<crate::db::model::MuscleGroup>> {
        self.workout.get_all_muscle_groups(user_id).await
    }

    pub async fn get_variations(&self,user_id: i32) -> Result<Vec<crate::db::model::Variation>> {
        self.workout.get_all_variations(user_id).await
    }

    pub async fn get_cardio_exercises(&self,user_id: i32) -> Result<Vec<crate::db::model::CardioExercise>> {
        self.workout.get_all_cardio_exercises(user_id).await
    }
}