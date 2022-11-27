SELECT DISTINCT e.event_type, p.*, u.handle AS user_handle, u.fediverse_id AS user_fediverse_id, 
u.avatar_url AS user_avatar_url, COUNT(DISTINCT l.like_id) AS likes, FALSE AS liked,
COUNT(DISTINCT c.comment_id) AS comments, u2.handle AS event_user_handle, u2.fediverse_id AS event_user_fediverse_id, 
u2.avatar_url AS event_user_avatar_url
FROM events e
INNER JOIN posts p
ON p.post_id = e.post_id
INNER JOIN users u
ON u.user_id = p.user_id
INNER JOIN users u2
ON u2.user_id = e.source_user_id
LEFT OUTER JOIN likes l
ON l.post_id = p.post_id
LEFT OUTER JOIN comments c
ON c.post_id = p.post_id
WHERE e.target_user_id IS NULL
AND e.visibility IN ('public_federated', 'public_local')
GROUP BY e.event_type, p.post_id, u.user_id, u2.user_id
ORDER BY p.created_at DESC
LIMIT $1
OFFSET $2
