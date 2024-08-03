-- 2024-07-21-171053_loans.up.sql

create table loans (
    id SERIAL PRIMARY KEY,
    amount integer default 0,
    user_id int,
    FOREIGN KEY (user_id) REFERENCES users(id),
    borrowed_at timestamp without time zone default CURRENT_TIMESTAMP,
    interest_Rate float NOT NULL,
    amount_paid float NOT NULL

);
