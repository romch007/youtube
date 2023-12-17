alter table videos add column textsearchable_index_col tsvector
  not null
  generated always as (
    setweight(to_tsvector('english', title), 'A') || ' ' ||
    setweight(to_tsvector('english', description), 'B') :: tsvector
) stored;

create index textsearch_idx on videos using gin (textsearchable_index_col);
