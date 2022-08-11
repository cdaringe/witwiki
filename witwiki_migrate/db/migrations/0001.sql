create table if not exists user (
    id integer primary key autoincrement,
    username varchar(50) not null unique,
    first_name varchar(50),
    last_name varchar(50),
    user_preferences_id int,
    authentication_strategy varchar(10) default "unpw"
);
-- migration
create table if not exists user_authentication_strategy_unpw (
  id integer primary key autoincrement,
  hash varchar(200) not null,
  user_id int not null unique,
  foreign key (user_id) references user (id)
);
-- migration
create table if not exists user_preference (
    id integer primary key autoincrement,
    ui_editor_theme_name varchar(50)
);
-- migration
create table if not exists post (
  id integer primary key autoincrement,
  user_id int not null,
  body text not null,
  title varchar(300) not null,
  slug text not null unique,
  created_at int not null default (strftime('%s','now')),
  updated_at int not null default (strftime('%s','now'))
);
-- migration
create table if not exists post_tag (
  id integer primary key autoincrement,
  post_id int not null,
  tag_id int not null,
  foreign key (post_id) references post (id),
  foreign key (tag_id) references tag (id),
  unique (post_id, tag_id)
);
-- migration
create index if not exists slug_idx ON post (slug);
-- migration
create table if not exists post_history (
  id integer primary key autoincrement,
  user_id int not null,
  post_id int not null,
  body_diff text not null,
  title varchar(300) not null
);
-- migration
create table if not exists media_type (
  id integer primary key autoincrement,
  description varchar(20) not null unique
);
-- migration
create table if not exists tag (
  id integer primary key autoincrement,
  media_type_id integer not null,
  tag varchar(30) not null,
  foreign key (media_type_id) references media_type (id),
  unique (media_type_id, tag)
);
-- migration
create table if not exists post_comment (
  id integer primary key autoincrement,
  post_id int not null,
  body text not null,
  user_id int not null,
  created_at int not null default (strftime('%s','now')),
  foreign key (post_id) references post (id),
  foreign key (user_id) references user (id),
  unique (user_id, created_at)
);
create table if not exists meta (
  version int not null
)
