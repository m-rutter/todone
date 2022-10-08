CREATE TABLE "user" (
    user_id SERIAL PRIMARY KEY,
    username TEXT COLLATE "case_insensitive" UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
	created_at timestamptz NOT NULL DEFAULT NOW(),
	updated_at timestamptz
);

SELECT trigger_updated_at('user');