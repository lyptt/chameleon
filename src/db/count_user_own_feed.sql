SELECT COUNT(DISTINCT e.post_id) FROM events e
WHERE (e.source_user_id = $1 AND e.visibility IN ('public_federated', 'public_local', 'followers_only', 'private', 'unlisted'))
OR (e.target_user_id = $1 AND e.visibility IN ('public_federated', 'public_local', 'followers_only'))
