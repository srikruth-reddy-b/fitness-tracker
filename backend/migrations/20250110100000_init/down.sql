-- Drop tables in reverse order of dependency
DROP TABLE IF EXISTS fittrack.cardio_logs;
DROP TABLE IF EXISTS fittrack.sets;
DROP TABLE IF EXISTS fittrack.workout_sessions;
DROP TABLE IF EXISTS fittrack.cardio_exercises;
DROP TABLE IF EXISTS fittrack.variations;
DROP TABLE IF EXISTS fittrack.muscle_groups;
DROP TABLE IF EXISTS fittrack.users;

-- Drop Schema
DROP SCHEMA IF EXISTS fittrack CASCADE;

-- Drop Helper Functions
DROP FUNCTION IF EXISTS diesel_manage_updated_at;
DROP FUNCTION IF EXISTS diesel_set_updated_at;
