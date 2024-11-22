// @generated automatically by Diesel CLI.

diesel::table! {
    distros (id) {
        id -> Integer,
        url -> Text,
        name -> Nullable<Text>,
        path -> Nullable<Text>,
        key -> Nullable<Text>,
        armored_key -> Bool,
    }
}
