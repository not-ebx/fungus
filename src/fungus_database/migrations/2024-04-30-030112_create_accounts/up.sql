CREATE TABLE accounts (
      id SERIAL PRIMARY KEY,
      world_id SMALLINT NOT NULL DEFAULT 0,
      storage_mesos BIGINT NOT NULL DEFAULT 0,
      character_slots SMALLINT NOT NULL DEFAULT 3,
      created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
      user_id INTEGER NOT NULL,
      FOREIGN KEY (user_id) REFERENCES users(id)
);