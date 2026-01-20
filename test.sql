WITH litem AS (
    SELECT id
    FROM item
    ORDER BY created
    LIMIT ?1 OFFSET ?2
)
SELECT id, attr.val AS attr_val, attr.name AS attr_name FROM litem
INNER JOIN item_attr ON litem.id = item_attr.item_id
INNER JOIN attr ON attr.name = item_attr.attr_name
	AND attr.val = item_attr.attr_val
ORDER BY item_attr.item_id, attr.name;
;
