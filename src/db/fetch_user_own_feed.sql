SELECT DISTINCT p.*, u.user_id, u.handle as user_handle, u.fediverse_id as user_fediverse_id, u.avatar_url as user_avatar_url, likes, count(l.like_id) = 1 as liked FROM posts p
INNER JOIN users u
ON u.user_id = p.user_id
LEFT OUTER JOIN (
  SELECT count(distinct l.like_id), l.like_id, l.post_id, l.user_id
  FROM likes l
  GROUP BY l.post_id, l.user_id, l.like_id
) AS l
ON l.post_id = p.post_id
AND l.user_id = u.user_id
INNER JOIN (
  SELECT p.post_id as post_id, p.user_id as user_id, count(distinct l.like_id) as likes from followers f
  INNER JOIN users u1
  ON u1.user_id = f.user_id
  INNER JOIN users u2
  ON u2.user_id = f.following_user_id
  LEFT OUTER JOIN posts p
  ON p.user_id = u1.user_id OR p.user_id = u2.user_id
  LEFT OUTER JOIN likes l
  ON l.post_id = p.post_id
  WHERE u1.fediverse_id = $1
  AND (
    (p.user_id = u1.user_id AND p.visibility IN ('shadow', 'unlisted', 'private', 'followers_only', 'public_local', 'public_federated'))
    OR (p.user_id = u2.user_id AND p.visibility IN ('followers_only', 'public_local', 'public_federated'))
  )
  AND (l IS NULL OR (l.post_id = p.post_id AND l.user_id IN (u1.user_id, u2.user_id)))
  GROUP BY p.post_id
  ORDER BY p.created_at DESC
) AS pu
ON p.post_id = pu.post_id
AND u.user_id = pu.user_id
GROUP BY p.post_id, u.user_id, pu.likes
ORDER BY p.created_at DESC
LIMIT $2
OFFSET $3
