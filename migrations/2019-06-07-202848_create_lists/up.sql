CREATE TABLE lists (
  id serial primary key,
  public_id uuid not null unique default gen_random_uuid(),
  title varchar not null
);
