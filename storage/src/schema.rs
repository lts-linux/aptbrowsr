// @generated automatically by Diesel CLI.

diesel::table! {
    distros (id) {
        id -> Integer,
        repo_url -> Text,
        name_or_path -> Text,
        repo_key -> Nullable<Text>,
        armored_key -> Bool,
        flat_repo -> Bool,
    }
}
