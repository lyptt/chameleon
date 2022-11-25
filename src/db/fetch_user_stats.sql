SELECT
(
  SELECT COUNT(*) FROM followers f
  INNER JOIN users u
  ON u.user_id = f.user_id
  WHERE u.handle = $1
  AND following_user_id != u.user_id
) AS following_count,

(
  SELECT COUNT(*) FROM followers f
  INNER JOIN users u
  ON u.user_id = f.following_user_id
  WHERE u.handle = $1
  AND f.user_id != u.user_id
) AS followers_count,

(
  SELECT COUNT(*) >= 1 FROM followers f
  INNER JOIN users u
  ON u.user_id = f.following_user_id
  WHERE u.handle = $1
  AND f.user_id = $2
) AS following_user,

(
  SELECT COUNT(*) >= 1 FROM users u
  WHERE u.handle = $1
  AND u.user_id = $2
) AS user_is_you
