// @generated automatically by Diesel CLI.

diesel::table! {
    law_profiles (uid) {
        uid -> Uuid,
        #[max_length = 15]
        itn -> Varchar,
        start_activity_date -> Timestamp,
    }
}
