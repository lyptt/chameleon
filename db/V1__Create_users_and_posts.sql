CREATE TABLE users (
  "user_id" uuid NOT NULL,
  "fediverse_id" varchar(2048) NOT NULL,
  "handle" varchar(256),
  "email" varchar(2048),
  "password_hash" varchar(64),
  "is_external" bool NOT NULL DEFAULT true,
  "is_admin" bool NOT NULL DEFAULT false,
  PRIMARY KEY ("user_id")
);

CREATE TABLE posts (
  "post_id" uuid NOT NULL,
  "user_id" uuid NOT NULL,
  "uri" varchar(2048) NOT NULL,
  "is_external" bool NOT NULL DEFAULT true,
  "content_md" text NOT NULL,
  "content_html" text NOT NULL,
  "content_image_uri_small" varchar(2048),
  "content_image_uri_medium" varchar(2048),
  "content_image_uri_large" varchar(2048),
  "content_image_storage_ref" varchar(2048) NOT NULL,
  "visibility" varchar(32) NOT NULL DEFAULT 'private',
  "created_at" timestamptz NOT NULL DEFAULT now(),
  "updated_at" timestamptz NOT NULL DEFAULT now(),
  "deletion_scheduled_at" timestamptz,
  "content_width_small" int4,
  "content_height_small" int4,
  "content_width_medium" int4,
  "content_height_medium" int4,
  "content_width_large" int4,
  "content_height_large" int4,
  "content_type_small" varchar(64),
  "content_type_medium" varchar(64),
  "content_type_large" varchar(64),
  PRIMARY KEY ("post_id"),
  CONSTRAINT fk_post_user FOREIGN KEY(user_id) REFERENCES users(user_id) ON DELETE CASCADE ON UPDATE CASCADE
);
