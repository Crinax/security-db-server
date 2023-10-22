// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "court_cases_decisions"))]
    pub struct CourtCasesDecisions;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "court_cases_kinds"))]
    pub struct CourtCasesKinds;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_profiles_roles"))]
    pub struct UserProfilesRoles;
}

diesel::table! {
    chats (uid) {
        uid -> Uuid,
        creator_uid -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 64]
        connection_hash -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CourtCasesDecisions;
    use super::sql_types::CourtCasesKinds;

    court_cases (uid) {
        uid -> Uuid,
        #[max_length = 50]
        number -> Varchar,
        #[max_length = 255]
        judge_fullname -> Varchar,
        decision -> CourtCasesDecisions,
        kind -> CourtCasesKinds,
        created_at -> Timestamp,
    }
}

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
    message_files (uid) {
        uid -> Uuid,
        message_uid -> Uuid,
        file_uid -> Uuid,
    }
}

diesel::table! {
    messages (uid) {
        uid -> Uuid,
        chat_uid -> Uuid,
        sender_uid -> Nullable<Uuid>,
        content -> Text,
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

diesel::table! {
    services (uid) {
        uid -> Uuid,
        law_uid -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        cost -> Float8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserProfilesRoles;

    user_profiles (uid) {
        uid -> Uuid,
        passport_uid -> Nullable<Uuid>,
        law_profile -> Nullable<Uuid>,
        avatar_uid -> Nullable<Uuid>,
        role -> UserProfilesRoles,
        created_at -> Timestamp,
    }
}

diesel::joinable!(chats -> user_profiles (creator_uid));
diesel::joinable!(message_files -> files (file_uid));
diesel::joinable!(message_files -> messages (message_uid));
diesel::joinable!(messages -> chats (chat_uid));
diesel::joinable!(messages -> user_profiles (sender_uid));
diesel::joinable!(services -> user_profiles (law_uid));
diesel::joinable!(user_profiles -> files (avatar_uid));
diesel::joinable!(user_profiles -> law_profiles (law_profile));
diesel::joinable!(user_profiles -> passports (passport_uid));

diesel::allow_tables_to_appear_in_same_query!(
    chats,
    court_cases,
    files,
    law_profiles,
    message_files,
    messages,
    passports,
    services,
    user_profiles,
);
