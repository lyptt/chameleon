SELECT DISTINCT p.*, u.user_id, u.handle as user_handle, u.fediverse_id as user_fediverse_id, u.avatar_url as user_avatar_url, count(distinct l.like_id) as likes, count(distinct c.comment_id) as comments from posts p
INNER JOIN users u
ON u.user_id = p.user_id
LEFT OUTER JOIN likes l
ON l.post_id = p.post_id
LEFT OUTER JOIN comments c
ON c.post_id = p.post_id
WHERE p.visibility IN ('public_local', 'public_federated')
GROUP BY p.post_id, u.user_id
ORDER BY p.created_at DESC
LIMIT $1
OFFSET $2
