alter table videos add column ts tsvector
  not null
  generated always as (
    setweight(to_tsvector('english', title), 'A') || ' ' ||
    setweight(to_tsvector('english', description), 'B') :: tsvector
) stored;

create index ts_idx on videos using gin (ts);
