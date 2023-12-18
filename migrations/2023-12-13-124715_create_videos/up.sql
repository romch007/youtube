create table videos (
  id serial primary key,
  title varchar not null,
  description text not null,
  duration_seconds bigint not null,
  bucket varchar not null,
  published_at timestamptz not null default now(),
  author_id int not null references users(id)
)
