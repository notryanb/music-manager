table! {
    files (id) {
        id -> Int4,
        path -> Text,
    }
}

table! {
    frame_types (id) {
        id -> Int4,
        code -> Text,
        description -> Text,
    }
}

table! {
    frames (id) {
        id -> Int4,
        id3_tag_id -> Nullable<Int4>,
        frame_type_id -> Nullable<Int4>,
        content -> Text,
    }
}

table! {
    id3_tags (id) {
        id -> Int4,
        version_id -> Nullable<Int4>,
        file_id -> Nullable<Int4>,
    }
}

table! {
    posts (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        content -> Text,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

table! {
    versions (id) {
        id -> Int4,
        version -> Text,
    }
}

joinable!(frames -> frame_types (frame_type_id));
joinable!(frames -> id3_tags (id3_tag_id));
joinable!(id3_tags -> files (file_id));
joinable!(id3_tags -> versions (version_id));
joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    files,
    frame_types,
    frames,
    id3_tags,
    posts,
    users,
    versions,
);
