SELECT DISTINCT p.*, u.user_id, u.handle as user_handle, u.fediverse_id as user_fediverse_id, u.avatar_url as user_avatar_url, COUNT(DISTINCT l.like_id) AS likes, count(ul.like_id) >= 1 AS liked, count(distinct c.comment_id) as comments FROM posts p
INNER JOIN users u
ON u.user_id = p.user_id
LEFT OUTER JOIN likes l
ON l.post_id = p.post_id
LEFT OUTER JOIN (SELECT DISTINCT post_id, like_id FROM likes WHERE user_id = $2) AS ul
ON ul.post_id = p.post_id
LEFT OUTER JOIN comments c
ON c.post_id = p.post_id
WHERE p.post_id = $1
GROUP BY p.post_id, u.user_id
