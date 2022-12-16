ALTER TABLE users ADD COLUMN public_key VARCHAR(2048) NULL;
ALTER TABLE users ADD COLUMN private_key VARCHAR(2048) NULL;
ALTER TABLE users ADD COLUMN ext_apub_followers_uri VARCHAR(2048) NULL;
ALTER TABLE users ADD COLUMN ext_apub_following_uri VARCHAR(2048) NULL;
ALTER TABLE users ADD COLUMN ext_apub_inbox_uri VARCHAR(2048) NULL;
ALTER TABLE users ADD COLUMN ext_apub_outbox_uri VARCHAR(2048) NULL;

UPDATE users SET public_key = '';
UPDATE users SET private_key = '';
ALTER TABLE users ALTER COLUMN public_key SET NOT NULL;
ALTER TABLE users ALTER COLUMN private_key SET NOT NULL;
