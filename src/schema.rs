// @generated automatically by Diesel CLI.

diesel::table! {
    files (uid) {
        uid -> Uuid,
        #[max_length = 36]
        file_name -> Bpchar,
        original_name -> Varchar,
    }
}

diesel::table! {
    law_profiles (uid) {
        uid -> Uuid,
        #[max_length = 15]
        itn -> Varchar,
        start_activity_date -> Timestamp,
    }
}

diesel::table! {
    passports (uid) {
        uid -> Uuid,
        first_name -> Varchar,
        second_name -> Varchar,
        patronymic -> Nullable<Varchar>,
        #[max_length = 6]
        number -> Bpchar,
        #[max_length = 4]
        series -> Bpchar,
        registration_place -> Varchar,
        birthday_date -> Date,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    files,
    law_profiles,
    passports,
);
