-- how do i get cargo to build this only in dev mode, when i use include_str!(...)?
insert into user (username, first_name, last_name)
  values
    ('raptorboy', 'alan', 'grant'),
    ('fossilbabe', 'ellie', 'sattler'),
    ('sellout', 'nedry', 'snickermeister'),
    ('unixlove', 'lex', 'kid'),
    ('nerd', 'timmy', 'kid'),
    ('kaos', 'jeff', 'goldblum'),
    ('hntr', 'austrailian', 'guy');

-- migration
insert into post (user_id, body, title) values
   (3, 'tacos are great
- they taste great
- they look great', 'best foods'),
   (1, 'Mr. Hammond, After Careful Consideration, **I Have Decided Not To Endorse Your Park.**', 'dino dna!');

-- migration
insert into post_history (user_id, post_id, body_diff, title) values
  (3, 1, '- fake_del\n+ fake_add', 'worst foods'),
  (3, 1, '- fake_del\n+ fake_add', 'ok foods'),
  (2, 2, '- fake_del\n+ fake_add', 'dino-dna');
