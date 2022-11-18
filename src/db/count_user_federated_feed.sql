SELECT DISTINCT COUNT(p.*) from posts p
INNER JOIN users u
ON u.user_id = p.user_id
WHERE u.fediverse_id = $1
AND p.visibility IN ('public_federated')
