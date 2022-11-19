SELECT COUNT(DISTINCT c.comment_id) FROM comments c
INNER JOIN posts p
ON p.post_id = c.post_id
LEFT OUTER JOIN (
  SELECT f.user_id, f.following_user_id, COUNT(DISTINCT f.follower_id) >= 1 AS following
  FROM followers f
  INNER JOIN users u1
  ON u1.user_id = f.user_id
  INNER JOIN users u2
  ON u2.user_id = f.following_user_id
  WHERE u1.user_id = $1 -- the user viewing the post's comments
  GROUP BY f.user_id, f.following_user_id
) AS ff
ON ff.following_user_id = p.user_id
WHERE c.post_id = $2 -- the post id of this comment collection
AND (
  (p.visibility IN ('public_local', 'public_federated'))
    OR (following IS TRUE AND p.visibility = 'followers_only'))
