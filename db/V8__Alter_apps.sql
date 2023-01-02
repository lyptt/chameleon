ALTER TABLE apps DROP CONSTRAINT fk_app_user;

ALTER TABLE apps DROP COLUMN user_id;
ALTER TABLE apps DROP COLUMN owner_instance_uri;
ALTER TABLE apps ALTER COLUMN client_id TYPE VARCHAR(64);
ALTER TABLE apps ALTER COLUMN client_secret TYPE VARCHAR(64);

CREATE UNIQUE INDEX apps_client_id_secret ON apps(client_id, client_secret);
CREATE UNIQUE INDEX apps_client_id ON apps(client_id);
CREATE UNIQUE INDEX apps_client_secret ON apps(client_secret);
