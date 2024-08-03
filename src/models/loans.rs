use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::loans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Loans {
    pub id: i32,
    pub amount: Option<i32>,           // Changed to Option<i32>
    pub user_id: Option<i32>,          // Changed to Option<i32>
    pub borrowed_at: Option<NaiveDateTime>,  // Changed to Option<NaiveDateTime>
    pub interest_rate: Option<f64>,    // Changed to Option<f64>
    pub amount_paid: Option<f64>,
    pub loan_limit: i32,
}

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::loans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewLoan { 
    pub amount: Option<i32>,           // Changed to Option<i32>
    pub user_id: Option<i32>,          // Changed to Option<i32>
    pub interest_rate: Option<f64>,    // Changed to Option<f64>
}
