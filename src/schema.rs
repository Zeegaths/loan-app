// @generated automatically by Diesel CLI.

diesel::table! {
    loans (id) {
        id -> Int4,
        amount -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        borrowed_at -> Nullable<Timestamp>,
        interest_rate -> Nullable<Float8>,
        amount_paid -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 150]
        name -> Nullable<Varchar>,
        #[max_length = 150]
        email -> Nullable<Varchar>,
        sessiontoken -> Nullable<Int4>,
        #[max_length = 255]
        password -> Nullable<Varchar>,
        loan_limit -> Nullable<Int4>,
    }
}

diesel::joinable!(loans -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    loans,
    users,
);
