SELECT DISTINCT e.event_Type, p.*, u.user_id, u.handle as user_handle, u.fediverse_id as user_fediverse_id, 
u.fediverse_uri AS user_fediverse_uri, u.avatar_url as user_avatar_url, u.handle as event_user_handle, 
u.fediverse_id as event_user_fediverse_id, u.fediverse_uri AS event_user_fediverse_uri, 
u.avatar_url as event_user_avatar_url, COUNT(DISTINCT l.like_id) AS likes, count(l2.like_id) >= 1 AS liked, 
count(distinct c.comment_id) as comments, pa.attachment_id, pa.user_id as attachment_user_id, 
pa.post_id as attachment_post_id, pa.uri as attachment_uri, pa.width as attachment_width, 
pa.height as attachment_height, pa.content_type as attachment_content_type, pa.storage_ref as attachment_storage_ref, 
pa.blurhash as attachment_blurhash, pa.created_at as attachment_created_at FROM events e
INNER JOIN posts p
ON p.post_id = e.post_id
INNER JOIN users u
ON u.user_id = p.user_id
LEFT OUTER JOIN likes l
ON l.post_id = p.post_id
LEFT OUTER JOIN likes l2
ON l2.post_id = p.post_id
AND l2.user_id = $2
LEFT OUTER JOIN comments c
ON c.post_id = p.post_id
LEFT OUTER JOIN post_attachments pa
ON pa.post_id = p.post_id
WHERE p.post_id = $1
GROUP BY e.event_type, p.post_id, u.user_id, pa.attachment_id
