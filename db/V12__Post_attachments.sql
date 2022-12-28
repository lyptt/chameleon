ALTER TABLE posts DROP COLUMN content_image_uri_small;
ALTER TABLE posts DROP COLUMN content_image_uri_medium;
ALTER TABLE posts DROP COLUMN content_image_uri_large;
ALTER TABLE posts DROP COLUMN content_width_small;
ALTER TABLE posts DROP COLUMN content_width_medium;
ALTER TABLE posts DROP COLUMN content_width_large;
ALTER TABLE posts DROP COLUMN content_height_small;
ALTER TABLE posts DROP COLUMN content_height_medium;
ALTER TABLE posts DROP COLUMN content_height_large;
ALTER TABLE posts DROP COLUMN content_type_small;
ALTER TABLE posts DROP COLUMN content_type_medium;
ALTER TABLE posts DROP COLUMN content_type_large;
ALTER TABLE posts DROP COLUMN content_image_storage_ref;
ALTER TABLE posts DROP COLUMN content_blurhash;

CREATE TABLE post_attachments (
  attachment_id uuid NOT NULL,
  user_id uuid NOT NULL,
  post_id uuid NOT NULL,
  uri VARCHAR(2048) NULL,
  width INT4 NOT NULL DEFAULT 0,
  height INT4 NOT NULL DEFAULT 0,
  content_type VARCHAR(64) NULL,
  storage_ref VARCHAR(2048) NULL,
  blurhash VARCHAR(256) NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  CONSTRAINT post_attachments_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT post_attachments_post_id_fkey FOREIGN KEY (post_id) REFERENCES posts(post_id) ON DELETE CASCADE ON UPDATE CASCADE,
  PRIMARY KEY (attachment_id)
);

CREATE UNIQUE INDEX post_attachments_user_post_idx ON post_attachments(user_id, post_id);

