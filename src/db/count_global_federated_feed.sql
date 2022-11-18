SELECT COUNT(DISTINCT p.*) from posts p
WHERE p.visibility IN ('public_local', 'public_federated')
