CREATE TABLE events (
  event_id UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  source_user_id UUID NOT NULL,
  target_user_id UUID NULL,
  visibility VARCHAR(32) NOT NULL DEFAULT 'public_federated',
  post_id UUID NULL REFERENCES posts(post_id) ON DELETE CASCADE ON UPDATE CASCADE,
  like_id UUID NULL REFERENCES likes(like_id) ON DELETE CASCADE ON UPDATE CASCADE,
  comment_id UUID NULL REFERENCES comments(comment_id) ON DELETE CASCADE ON UPDATE CASCADE,
  event_type VARCHAR(32) NOT NULL DEFAULT 'post',
  CONSTRAINT events_source_user_id_fkey FOREIGN KEY (source_user_id) REFERENCES users(user_id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT events_target_user_id_fkey FOREIGN KEY (target_user_id) REFERENCES users(user_id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT events_event_post_id_fkey FOREIGN KEY (post_id) REFERENCES posts(post_id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT events_event_like_id_fkey FOREIGN KEY (like_id) REFERENCES likes(like_id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT events_event_comment_id_fkey FOREIGN KEY (comment_id) REFERENCES comments(comment_id) ON DELETE CASCADE ON UPDATE CASCADE,
  PRIMARY KEY (event_id)
);

CREATE INDEX events_post_idx ON events(post_id);
CREATE UNIQUE INDEX events_uq_event_idx ON events(source_user_id, target_user_id, post_id, like_id, comment_id);
