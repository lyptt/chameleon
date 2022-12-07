SELECT COUNT(DISTINCT e.post_id) FROM events e
INNER JOIN likes l
ON l.post_id = e.post_id
AND l.user_id = $1
WHERE e.source_user_id = $1
AND (
  (e.target_user_id = $2 AND e.visibility IN ('public_federated', 'public_local', 'followers_only') OR 
  (e.visibility IN ('public_federated', 'public_local'))
))
