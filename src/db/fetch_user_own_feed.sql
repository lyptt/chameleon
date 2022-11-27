SELECT DISTINCT e.event_type as event_type, p.*, u.handle AS user_handle, u.fediverse_id AS user_fediverse_id, u.avatar_url AS user_avatar_url, COUNT(DISTINCT l.like_id) AS likes, COUNT(DISTINCT l2.like_id) >= 1 AS liked, COUNT(DISTINCT c.comment_id) AS comments FROM events e
INNER JOIN posts p
ON p.post_id = e.post_id
INNER JOIN users u
ON u.user_id = p.user_id
LEFT OUTER JOIN likes l
ON l.post_id = p.post_id
LEFT OUTER JOIN likes l2
ON l2.post_id = p.post_id
AND l2.user_id = u.user_id
LEFT OUTER JOIN comments c
ON c.post_id = p.post_id
WHERE (e.source_user_id = $1 AND e.visibility IN ('public_federated', 'public_local', 'followers_only', 'private', 'unlisted'))
OR (e.target_user_id = $1 AND e.visibility IN ('public_federated', 'public_local', 'followers_only'))
GROUP BY e.event_type, p.post_id, u.handle, u.fediverse_id, u.avatar_url
ORDER BY p.created_at DESC
LIMIT $2
OFFSET $3
