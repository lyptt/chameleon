ALTER TABLE orbits ADD COLUMN public_key VARCHAR(2048) NULL;
ALTER TABLE orbits ADD COLUMN private_key VARCHAR(2048) NULL;

UPDATE orbits SET public_key = '';
UPDATE orbits SET private_key = '';
ALTER TABLE orbits ALTER COLUMN public_key SET NOT NULL;
ALTER TABLE orbits ALTER COLUMN private_key SET NOT NULL;
