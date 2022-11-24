SELECT COUNT(*)
FROM (
  SELECT DISTINCT p.post_id FROM followers f
  INNER JOIN users u1
  ON u1.user_id = f.user_id
  INNER JOIN users u2
  ON u2.user_id = f.following_user_id
  LEFT OUTER JOIN posts p
  ON p.user_id = u1.user_id
  LEFT OUTER JOIN likes l
  ON l.post_id = p.post_id
  LEFT OUTER JOIN comments c
  ON c.post_id = p.post_id
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
  GROUP BY p.post_id, u1.user_id) AS post_ids
