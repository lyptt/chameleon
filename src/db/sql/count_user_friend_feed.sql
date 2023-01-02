SELECT COUNT(DISTINCT e.post_id) FROM events e
INNER JOIN posts p
ON p.post_id = e.post_id
WHERE e.target_user_id = $1
AND e.source_user_id != $1
AND e.visibility IN ('public_federated', 'public_local', 'followers_only')
AND p.orbit_id IS NULL
