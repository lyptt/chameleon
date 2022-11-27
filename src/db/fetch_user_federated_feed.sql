SELECT DISTINCT e.event_type as event_type, p.* FROM events e
INNER JOIN posts p
ON p.post_id = e.post_id
WHERE e.source_user_id = $1
AND e.target_user_id IS NULL
AND e.visibility IN ('public_federated', 'public_local')
ORDER BY p.created_at DESC
LIMIT $2
OFFSET $3
