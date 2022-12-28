SELECT COUNT(*) FROM posts WHERE orbit_id = $1
AND visibility IN ('public_federated', 'public_local')
