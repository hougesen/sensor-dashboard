INSERT INTO locations (location_name)
VALUES ($1)
RETURNING $table_fields;