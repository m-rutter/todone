CREATE TABLE "todo" (
    todo_id uuid primary key default gen_random_uuid(),
    user_id uuid not null references "user"(user_id),
    content text not null,
    complete boolean not null default false,
	created_at timestamptz NOT NULL DEFAULT NOW(),
	updated_at timestamptz
);

SELECT trigger_updated_at('todo');