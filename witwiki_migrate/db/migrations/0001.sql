create table if not exists user (
    id integer not null primary key autoincrement,
    username varchar(50) not null unique,
    first_name varchar(50),
    last_name varchar(50),
    user_preferences_id int,
    authentication_strategy integer not null default 1
);
-- migration
create table if not exists identity_authentication_strategy_unpw (
  id integer not null primary key autoincrement,
  salt varchar(200) not null,
  hash varchar(200) not null,
  user_id integer not null unique,
  foreign key (user_id) references user (id)
);
-- migration
create table if not exists user_preference (
    id integer not null primary key autoincrement,
    ui_editor_theme_name varchar(50)
);
-- migration
create table if not exists post (
  id integer not null primary key autoincrement,
  user_id integer not null,
  body text not null,
  title varchar(300) not null,
  slug text not null unique,
  created_at integer not null default (strftime('%s','now')),
  updated_at integer not null default (strftime('%s','now'))
);
-- migration
create table if not exists post_tag (
  id integer not null primary key autoincrement,
  post_id integer not null,
  tag_id integer not null,
  foreign key (post_id) references post (id),
  foreign key (tag_id) references tag (id),
  unique (post_id, tag_id)
);
-- migration
create index if not exists slug_idx ON post (slug);
-- migration
create table if not exists post_history (
  id integer not null primary key autoincrement,
  user_id integer not null,
  post_id integer not null,
  body_revision integer not null,
  title_revision integer not null
);
-- migration
create table if not exists post_body_revision_diff (
  id integer not null primary key autoincrement,
  revision_id integer not null,
  rev_type text not null,
  rev_text text default null,
  foreign key (revision_id) references post_history (id)
);
-- migration
create table if not exists media_type (
  id integer not null primary key autoincrement,
  description varchar(20) not null unique
);
-- migration
create table if not exists tag (
  id integer not null primary key autoincrement,
  media_type_id integer not null,
  tag varchar(30) not null,
  foreign key (media_type_id) references media_type (id),
  unique (media_type_id, tag)
);
-- migration
create table if not exists post_comment (
  id integer not null primary key autoincrement,
  post_id integer not null,
  body text not null,
  user_id integer not null,
  created_at integer not null default (strftime('%s','now')),
  foreign key (post_id) references post (id),
  foreign key (user_id) references user (id),
  unique (user_id, created_at)
);
create table if not exists meta (
  id integer not null primary key autoincrement,
  version integer not null
)
