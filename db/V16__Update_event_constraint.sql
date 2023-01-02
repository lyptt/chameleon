DROP INDEX events_uq_event_idx;
CREATE UNIQUE INDEX events_uq_event_idx ON events(source_user_id, target_user_id, post_id, like_id, comment_id) NULLS NOT DISTINCT;
