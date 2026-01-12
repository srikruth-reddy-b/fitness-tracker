-- Helper functions from initial setup
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Schema
CREATE SCHEMA IF NOT EXISTS fittrack;

-- Users table (Consolidated from initial + updates)
CREATE TABLE fittrack.users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    fullname VARCHAR(100) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    weight FLOAT,
    height FLOAT,
    dob DATE
);

-- Muscle Groups
CREATE TABLE fittrack.muscle_groups (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    user_id INTEGER REFERENCES fittrack.users(id) ON DELETE CASCADE,
    UNIQUE(name, user_id)
);

-- Variations
CREATE TABLE fittrack.variations (
    id SERIAL PRIMARY KEY,
    muscle_group_id INTEGER NOT NULL REFERENCES fittrack.muscle_groups(id) ON DELETE CASCADE,
    name VARCHAR NOT NULL,
    description TEXT,
    user_id INTEGER REFERENCES fittrack.users(id) ON DELETE CASCADE
);

-- Cardio Exercises
CREATE TABLE fittrack.cardio_exercises (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    user_id INTEGER REFERENCES fittrack.users(id) ON DELETE CASCADE,
    UNIQUE(name, user_id)
);

-- Workout Sessions
CREATE TABLE fittrack.workout_sessions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES fittrack.users(id) ON DELETE CASCADE,
    title VARCHAR,
    date DATE NOT NULL DEFAULT CURRENT_DATE,
    start_time TIMESTAMP NOT NULL DEFAULT NOW(),
    end_time TIMESTAMP NOT NULL DEFAULT NOW(),
    notes TEXT
);

-- Sets
CREATE TABLE fittrack.sets (
    id SERIAL PRIMARY KEY,
    workout_session_id INTEGER REFERENCES fittrack.workout_sessions(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES fittrack.users(id) ON DELETE CASCADE,
    variation_id INTEGER NOT NULL REFERENCES fittrack.variations(id) ON DELETE CASCADE,
    weight FLOAT NOT NULL,
    reps INTEGER NOT NULL,
    performed_on DATE NOT NULL DEFAULT CURRENT_DATE
);

-- Cardio Logs
CREATE TABLE fittrack.cardio_logs (
    id SERIAL PRIMARY KEY,
    workout_session_id INTEGER REFERENCES fittrack.workout_sessions(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES fittrack.users(id) ON DELETE CASCADE,
    cardio_exercise_id INTEGER NOT NULL REFERENCES fittrack.cardio_exercises(id) ON DELETE CASCADE,
    duration_minutes INTEGER NOT NULL,
    performed_on DATE NOT NULL DEFAULT CURRENT_DATE
);
