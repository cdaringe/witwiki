-- how do i get cargo to build this only in dev mode, when i use include_str!(...)?
insert into user (username, first_name, last_name)
  values
    ('raptorboy', 'alan', 'grant'),
    ('fossilbabe', 'ellie', 'sattler'),
    ('sellout', 'nedry', 'snickermeister'),
    ('unixlove', 'lex', 'kid'),
    ('nerd', 'timmy', 'kid'),
    ('kaos', 'jeff', 'goldblum'),
    ('hntr', 'austrailian', 'guy')
  on conflict(username) do nothing;

-- migration
insert into post (user_id, body, title, created_at, slug) values
   (3, 'tacos are great

- they taste great
- they look great', 'best foods', 1655869054, "tacos-locos"),
   (1, 'Mr. Hammond, After Careful Consideration, **I Have Decided Not To Endorse Your Park.**', 'dino dna!', 1655969058, "jurassic-part-rejection-letter")
  on conflict(slug) do nothing;

-- migration
insert into post_history (user_id, post_id, body_diff, title) values
  (3, 1, '- fake_del\n+ fake_add', 'worst foods'),
  (3, 1, '- fake_del\n+ fake_add', 'ok foods'),
  (2, 2, '- fake_del\n+ fake_add', 'dino-dna');

-- migration
insert into media_type (description) values ('post')
  on conflict(description) do nothing;
-- migration
insert into tag (media_type_id, tag)
  values (1, 'food'), (1, 'tacos'), (1, 'dinos')
  on conflict(media_type_id, tag) do nothing;
-- migration
insert into post_tag (post_id, tag_id)
  values (1, 1), (1, 2), (2, 2), (2, 3)
  on conflict(post_id, tag_id) do nothing;
