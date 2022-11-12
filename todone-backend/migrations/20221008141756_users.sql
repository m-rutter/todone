CREATE TABLE "user" (
    user_id uuid PRIMARY KEY default gen_random_uuid(),
    username TEXT COLLATE "case_insensitive" UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
	created_at timestamptz NOT NULL DEFAULT NOW(),
	updated_at timestamptz
);

SELECT trigger_updated_at('user');