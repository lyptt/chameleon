ALTER TABLE orbits ADD COLUMN ext_apub_followers_uri VARCHAR(2048) NULL;
ALTER TABLE orbits ADD COLUMN ext_apub_inbox_uri VARCHAR(2048) NULL;
ALTER TABLE orbits ADD COLUMN ext_apub_outbox_uri VARCHAR(2048) NULL;
ALTER TABLE orbits ADD COLUMN fediverse_uri VARCHAR(2048) NOT NULL;
