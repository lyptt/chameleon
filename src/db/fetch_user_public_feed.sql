SELECT DISTINCT e.event_Type, p.*, u.user_id, u.handle as user_handle, u.fediverse_id as user_fediverse_id, u.avatar_url as user_avatar_url, COUNT(DISTINCT l.like_id) AS likes, count(l2.like_id) >= 1 AS liked, count(distinct c.comment_id) as comments FROM events e
INNER JOIN posts p
ON p.post_id = e.post_id
INNER JOIN users u
ON u.user_id = p.user_id
LEFT OUTER JOIN likes l
ON l.post_id = p.post_id
LEFT OUTER JOIN likes l2
ON l2.post_id = p.post_id
AND l2.user_id = $1
LEFT OUTER JOIN comments c
ON c.post_id = p.post_id
WHERE e.source_user_id = $1
AND (
  (e.target_user_id = $2 AND e.visibility IN ('public_federated', 'public_local', 'followers_only') OR 
  (e.visibility IN ('public_federated', 'public_local'))
))
GROUP BY e.event_type, p.post_id, u.user_id
ORDER BY p.created_at DESC
LIMIT $3
OFFSET $4
