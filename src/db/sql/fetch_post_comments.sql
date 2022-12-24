SELECT DISTINCT c.*, count(ul.comment_like_id) >= 1 AS liked, count(DISTINCT ul2.comment_like_id) as likes, u.handle AS user_handle, u.fediverse_id AS user_fediverse_id, u.fediverse_uri AS user_fediverse_uri, u.avatar_url AS user_avatar_url, p.visibility as visibility FROM comments c
INNER JOIN posts p
ON p.post_id = c.post_id
INNER JOIN users u
ON u.user_id = c.user_id
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
LEFT OUTER JOIN (SELECT DISTINCT comment_id, comment_like_id FROM comment_likes WHERE user_id = $1) AS ul
ON ul.comment_id = c.comment_id
LEFT OUTER JOIN (SELECT DISTINCT comment_id, comment_like_id FROM comment_likes) AS ul2
ON ul2.comment_id = c.comment_id
WHERE c.post_id = $2 -- the post id of this comment collection
AND (
  (p.visibility IN ('public_local', 'public_federated'))
    OR (following IS TRUE AND p.visibility = 'followers_only'))
GROUP BY c.comment_id, u.user_id, p.post_id
ORDER BY c.created_at DESC
LIMIT $3
OFFSET $4
