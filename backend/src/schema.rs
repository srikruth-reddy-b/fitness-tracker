// @generated automatically by Diesel CLI.

pub mod fittrack {
    diesel::table! {
        fittrack.cardio_exercises (id) {
            id -> Int4,
            name -> Varchar,
            user_id -> Nullable<Int4>,
        }
    }

    diesel::table! {
        fittrack.cardio_logs (id) {
            id -> Int4,
            workout_session_id -> Nullable<Int4>,
            user_id -> Int4,
            cardio_exercise_id -> Int4,
            duration_minutes -> Int4,
            performed_on -> Date,
        }
    }

    diesel::table! {
        fittrack.muscle_groups (id) {
            id -> Int4,
            name -> Varchar,
            user_id -> Nullable<Int4>,
        }
    }

    diesel::table! {
        fittrack.sets (id) {
            id -> Int4,
            workout_session_id -> Nullable<Int4>,
            user_id -> Int4,
            variation_id -> Int4,
            weight -> Float8,
            reps -> Int4,
            performed_on -> Date,
        }
    }

    diesel::table! {
        fittrack.users (id) {
            id -> Int4,
            #[max_length = 50]
            username -> Varchar,
            #[max_length = 100]
            fullname -> Varchar,
            #[max_length = 100]
            email -> Varchar,
            #[max_length = 255]
            password -> Varchar,
            created_at -> Nullable<Timestamp>,
            weight -> Nullable<Float8>,
            height -> Nullable<Float8>,
            dob -> Nullable<Date>,
        }
    }

    diesel::table! {
        fittrack.variations (id) {
            id -> Int4,
            muscle_group_id -> Int4,
            name -> Varchar,
            description -> Nullable<Text>,
            user_id -> Nullable<Int4>,
        }
    }

    diesel::table! {
        fittrack.workout_sessions (id) {
            id -> Int4,
            user_id -> Int4,
            title -> Nullable<Varchar>,
            date -> Date,
            start_time -> Timestamp,
            end_time -> Timestamp,
            notes -> Nullable<Text>,
        }
    }

    diesel::joinable!(cardio_exercises -> users (user_id));
    diesel::joinable!(cardio_logs -> cardio_exercises (cardio_exercise_id));
    diesel::joinable!(cardio_logs -> users (user_id));
    diesel::joinable!(cardio_logs -> workout_sessions (workout_session_id));
    diesel::joinable!(muscle_groups -> users (user_id));
    diesel::joinable!(sets -> users (user_id));
    diesel::joinable!(sets -> variations (variation_id));
    diesel::joinable!(sets -> workout_sessions (workout_session_id));
    diesel::joinable!(variations -> muscle_groups (muscle_group_id));
    diesel::joinable!(variations -> users (user_id));
    diesel::joinable!(workout_sessions -> users (user_id));

    diesel::allow_tables_to_appear_in_same_query!(
        cardio_exercises,
        cardio_logs,
        muscle_groups,
        sets,
        users,
        variations,
        workout_sessions,
    );
}
