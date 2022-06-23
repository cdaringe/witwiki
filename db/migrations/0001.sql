create table if not exists user (
    id integer primary key autoincrement,
    username varchar(50) not null unique,
    first_name varchar(50),
    last_name varchar(50),
    user_preferences_id int
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
  created_at int not null default (strftime('%s','now'))
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
