CREATE TABLE followers (
  follower_id uuid NOT NULL,
  user_id uuid NOT NULL,
  following_user_id uuid NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  CONSTRAINT followers_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT followers_follower_user_id_fkey FOREIGN KEY (following_user_id) REFERENCES users(user_id) ON DELETE CASCADE ON UPDATE CASCADE,
  PRIMARY KEY (follower_id)
);

CREATE UNIQUE INDEX followers_user_follower_idx ON followers(user_id, following_user_id);
