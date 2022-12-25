CREATE TABLE apps (
  "app_id" uuid NOT NULL,
  "user_id" uuid NOT NULL,
  "name" varchar(64) NOT NULL,
  "description" varchar(512) NOT NULL,
  "owner_name" varchar(256) NOT NULL,
  "owner_uri" varchar(256) NOT NULL,
  "owner_instance_uri" varchar(256) NOT NULL,
  "redirect_uri" varchar(256) NOT NULL,
  "client_id" uuid NOT NULL,
  "client_secret" uuid NOT NULL,
  -- 'blessed' apps do not show the confirmation prompt when a user enters their credentials,
  -- they just sign you in directly.
  -- You should only bless apps that should be officially associated to your orbit instance,
  -- or ones you directly trust.
  -- By default only apps registered by a user originating from this instance can be marked as
  -- blessed in the UI. To bypass this security restriction you can manually bless apps from
  -- foreign instances in the DB directly.
  -- The whole idea behind blessing is to smooth out the authentication process for apps that
  -- are percieved to behave well and are good citizens within the fediverse.
  "blessed" bool NOT NULL DEFAULT false,
  PRIMARY KEY ("app_id"),
  CONSTRAINT fk_app_user FOREIGN KEY(user_id) REFERENCES users(user_id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE sessions (
  "session_id" uuid NOT NULL,
  "user_id" uuid NOT NULL,
  "app_id" uuid NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT now(),
  "updated_at" timestamptz NOT NULL DEFAULT now(),
  "refresh_token" VARCHAR(64) NOT NULL,
  "access_expires_at" timestamptz NOT NULL,
  "refresh_expires_at" timestamptz NOT NULL,
  PRIMARY KEY ("session_id"),
  CONSTRAINT fk_session_app FOREIGN KEY(app_id) REFERENCES apps(app_id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT fk_session_user FOREIGN KEY(user_id) REFERENCES users(user_id) ON DELETE CASCADE ON UPDATE CASCADE
);
