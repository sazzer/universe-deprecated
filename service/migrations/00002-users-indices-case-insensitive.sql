ALTER TABLE users DROP CONSTRAINT users_username_key;
ALTER TABLE users DROP CONSTRAINT users_email_key;

CREATE UNIQUE INDEX users_username_key ON users (UPPER(username));
CREATE UNIQUE INDEX users_email_key ON users (UPPER(email));
