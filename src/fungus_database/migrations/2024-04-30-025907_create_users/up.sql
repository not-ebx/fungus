CREATE TABLE users (
   id SERIAL PRIMARY KEY,
   username VARCHAR NOT NULL,
   password VARCHAR NOT NULL,
   birthday DATE NOT NULL,
   gender SMALLINT NOT NULL, -- Using SMALLINT for gender to store as integer
   nx_cash INTEGER NOT NULL DEFAULT 0,
   maple_points INTEGER NOT NULL DEFAULT 0,
   vote_points INTEGER NOT NULL DEFAULT 0,
   account_type SMALLINT NOT NULL DEFAULT 0, -- Using SMALLINT for account_type to store as integer
   pic SMALLINT,
   spw VARCHAR,
   ban_expire_date TIMESTAMP WITHOUT TIME ZONE,
   ban_reason TEXT,
   last_login TIMESTAMP WITHOUT TIME ZONE,
   created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW()
);
