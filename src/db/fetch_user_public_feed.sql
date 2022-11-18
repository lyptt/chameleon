SELECT DISTINCT p.*, u1.user_id as user_id, u1.handle as user_handle, u1.fediverse_id as user_fediverse_id, u1.avatar_url as user_avatar_url, count(distinct l.like_id) as likes, count(l2.like_id) >= 1 as liked FROM followers f
INNER JOIN users u1
ON u1.user_id = f.user_id
INNER JOIN users u2
ON u2.user_id = f.following_user_id
LEFT OUTER JOIN posts p
ON p.user_id = u1.user_id
LEFT OUTER JOIN likes l
ON l.post_id = p.post_id
LEFT OUTER JOIN (
  SELECT count(distinct l.like_id), l.like_id, l.post_id, l.user_id
  FROM likes l
  GROUP BY l.post_id, l.user_id, l.like_id
) AS l2
ON l2.post_id = p.post_id
AND l.user_id = u1.user_id
WHERE u1.fediverse_id = $1
AND (
  (p.visibility IN ('public_local', 'public_federated'))
    OR (u2.fediverse_id = $2 AND p.visibility = 'followers_only')
  )
GROUP BY p.post_id, u1.user_id
LIMIT $3
OFFSET $4
