SELECT COUNT(DISTINCT e.post_id) FROM events e
WHERE e.target_user_id IS NULL
AND e.visibility IN ('public_federated', 'public_local')
