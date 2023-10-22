// @generated automatically by Diesel CLI.

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
    court_cases (uid) {
        uid -> Uuid,
        #[max_length = 50]
        number -> Varchar,
        #[max_length = 255]
        judge_fullname -> Varchar,
        decision -> Int2,
        kind -> Int2,
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
    user_profiles (uid) {
        uid -> Uuid,
        passport_uid -> Nullable<Uuid>,
        law_profile -> Nullable<Uuid>,
        avatar_uid -> Nullable<Uuid>,
        role -> Int2,
        created_at -> Timestamp,
    }
}

diesel::joinable!(chats -> user_profiles (creator_uid));
diesel::joinable!(message_files -> files (file_uid));
diesel::joinable!(message_files -> messages (message_uid));
diesel::joinable!(messages -> chats (chat_uid));
diesel::joinable!(messages -> user_profiles (sender_uid));
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
    user_profiles,
);
