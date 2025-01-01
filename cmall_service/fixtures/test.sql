-- -- insert 3 workspaces
-- INSERT INTO workspaces(name, owner_id)
--   VALUES ('acm', 0),
-- ('test', 0),
-- ('pro', 0),
-- ('hr', 0),
-- ('dev', 0);

-- insert 5 users, all with hashed password '123456'
INSERT INTO users(dept_id, email, username, password_hash, phone, status, avatar, roles)
  VALUES (1, 'elixy@qq.com', 'Eli Shi', '$argon2id$v=19$m=19456,t=2,p=1$MxGhY+ib/kplwBPLa7u2ug$c5h9u7Sc8Px8J5+qgNdOjSY7ZJO2QN4rugKpapGW4XU','123123','active','default','{1}'),
(1, 'alice@acme.org', 'Alice Shi', '$argon2id$v=19$m=19456,t=2,p=1$MxGhY+ib/kplwBPLa7u2ug$c5h9u7Sc8Px8J5+qgNdOjSY7ZJO2QN4rugKpapGW4XU','234234','off','def','{1,2,3}');


-- -- insert 4 chats
-- -- insert public/private channel
-- INSERT INTO chats(ws_id, name, type, members)
--   VALUES (1, 'general', 'public_channel', '{1,2,3,4,5}'),
-- (1, 'private', 'private_channel', '{1,2,3}');

-- -- insert unnamed chat
-- INSERT INTO chats(ws_id, type, members)
--   VALUES (1, 'single', '{1,2}'),
-- (1, 'group', '{1,3,4}');
