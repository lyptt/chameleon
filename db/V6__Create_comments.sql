CREATE TABLE comments (
  comment_id uuid NOT NULL,
  user_id uuid NOT NULL,
  post_id uuid NOT NULL,
  content_md text NOT NULL,
  content_html text NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now(),
  CONSTRAINT comments_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT comments_post_id_fkey FOREIGN KEY (post_id) REFERENCES posts(post_id) ON DELETE CASCADE ON UPDATE CASCADE,
  PRIMARY KEY (comment_id)
);

CREATE INDEX comments_user_post_idx ON comments(user_id, post_id);

CREATE TABLE comment_likes (
  comment_like_id uuid NOT NULL,
  user_id uuid NOT NULL,
  comment_id uuid NOT NULL,
  post_id uuid NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  CONSTRAINT comment_likes_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT comment_likes_comment_id_fkey FOREIGN KEY (comment_id) REFERENCES comments(comment_id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT comment_likes_post_id_fkey FOREIGN KEY (post_id) REFERENCES posts(post_id) ON DELETE CASCADE ON UPDATE CASCADE,
  PRIMARY KEY (comment_like_id)
);

CREATE INDEX comment_likes_user_comment_idx ON comment_likes(user_id, comment_id);
