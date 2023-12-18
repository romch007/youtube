create table likes (
  user_id serial references users(id),
  video_id serial references videos(id),
  is_liking boolean not null,
  primary key(user_id, video_id)
);
