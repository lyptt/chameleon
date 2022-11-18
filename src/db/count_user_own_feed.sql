SELECT COUNT(DISTINCT p.post_id)
FROM posts p
INNER JOIN users u
ON u.user_id = p.user_id
INNER JOIN (
  SELECT p.post_id as post_id, p.user_id as user_id from followers f
  INNER JOIN users u1
  ON u1.user_id = f.user_id
  INNER JOIN users u2
  ON u2.user_id = f.following_user_id
  LEFT OUTER JOIN posts p
  ON p.user_id = u1.user_id OR p.user_id = u2.user_id
  WHERE u1.fediverse_id = $1
  AND (
    (p.user_id = u1.user_id AND p.visibility IN ('shadow', 'unlisted', 'private', 'followers_only', 'public_local', 'public_federated'))
    OR (p.user_id = u2.user_id AND p.visibility IN ('followers_only', 'public_local', 'public_federated'))
  )
  ORDER BY p.created_at DESC
) AS pu
ON p.post_id = pu.post_id
AND u.user_id = pu.user_id
