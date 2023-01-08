ALTER TABLE orbits ADD COLUMN fediverse_id VARCHAR(2048) NULL;
UPDATE orbits SET fediverse_id = '';
ALTER TABLE orbits ALTER COLUMN fediverse_id SET NOT NULL;
