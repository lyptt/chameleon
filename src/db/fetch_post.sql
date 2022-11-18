SELECT p.*, u.user_id, u.handle as user_handle, u.fediverse_id as user_fediverse_id, u.avatar_url as user_avatar_url FROM posts p
INNER JOIN users u
ON u.user_id = p.user_id
WHERE p.post_id = $1
