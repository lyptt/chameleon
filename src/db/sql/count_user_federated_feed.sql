SELECT COUNT(DISTINCT e.post_id) FROM events e
WHERE e.source_user_id = $1
AND e.target_user_id IS NULL
AND e.visibility IN ('public_federated', 'public_local')
