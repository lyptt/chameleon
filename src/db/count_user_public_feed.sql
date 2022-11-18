SELECT DISTINCT COUNT(p.*) from followers f
INNER JOIN users u1
ON u1.user_id = f.user_id
INNER JOIN users u2
ON u2.user_id = f.following_user_id
LEFT OUTER JOIN posts p
ON p.user_id = u1.user_id
WHERE u1.fediverse_id = $1
AND (
  (p.visibility IN ('public_local', 'public_federated'))
  OR (u2.fediverse_id = $2 AND p.visibility = 'followers_only')
)
