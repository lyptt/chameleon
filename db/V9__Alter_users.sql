ALTER TABLE users ALTER COLUMN handle SET NOT NULL;
ALTER TABLE users ADD COLUMN fediverse_uri VARCHAR(2048) NULL;
UPDATE users SET fediverse_uri = '';
ALTER TABLE users ALTER COLUMN fediverse_uri SET NOT NULL;
