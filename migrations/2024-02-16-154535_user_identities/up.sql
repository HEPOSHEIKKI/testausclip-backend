CREATE TABLE user_identities (LIKE users INCLUDING ALL);

INSERT INTO user_identities SELECT * FROM users;

ALTER TABLE user_identities
DROP COLUMN password,
DROP COLUMN salt;